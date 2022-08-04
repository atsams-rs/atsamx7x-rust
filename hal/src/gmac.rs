use core::{cell::UnsafeCell, ptr::NonNull, ops::{Deref, DerefMut}, marker::PhantomData};
use paste::*;
use crate::pmc::{HostClock, PeripheralIdentifier, Pmc};
use crate::target_device::gmac::RegisterBlock;
use crate::pio::*;

use crate::target_device::{GMAC, generic::Reg, gmac::{gmac_idrpq::GMAC_IDRPQ_SPEC, gmac_isrpq::GMAC_ISRPQ_SPEC}};
use smoltcp::phy::{Device, RxToken, TxToken, DeviceCapabilities, Medium, ChecksumCapabilities, Checksum};


const NUM_RX_BUFS: usize = 4;
const NUM_TX_BUFS: usize = 4;
const RX_BUF_SIZE: usize = 1024;
const TX_BUF_SIZE: usize = 1024;

// TODO: This needs a specific linker section (probably)
// Todo: UnsafeCell?
static RX_BUF_DESCS: [RxBufferDescriptor; NUM_RX_BUFS] = [RX_BUF_DESC_DEFAULT; NUM_RX_BUFS];
static RX_BUFS: [RxBuffer; NUM_RX_BUFS] = [RX_BUF_DEFAULT; NUM_RX_BUFS];
static TX_BUF_DESCS: [TxBufferDescriptor; NUM_TX_BUFS] = [TX_BUF_DESC_DEFAULT; NUM_TX_BUFS];
static TX_BUFS: [TxBuffer; NUM_TX_BUFS] = [TX_BUF_DEFAULT; NUM_TX_BUFS];

static UNUSED_TX_BUF_DESC: TxBufferDescriptor = TX_BUF_DESC_DEFAULT;

pub trait GmacMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

pub trait GmacPins {

}

#[derive(Debug)]
pub enum GmacError {

}

pub struct GmacConfiguration { }


pub struct GmacRxToken<'a> {
    rf: ReadFrame,
    _plt: PhantomData<&'a ()>,
}
pub struct GmacTxToken<'a> {
    gmac: &'a mut Gmac,
}

impl<'a> RxToken for GmacRxToken<'a> {
    fn consume<R, F>(mut self, timestamp: smoltcp::time::Instant, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>
    {
        // defmt::println!("RxCons: {=u32:08X}", timestamp.total_millis() as u32);
        // defmt::trace!("RX: {=[u8]:02X}", &self.rf);
        f(self.rf.deref_mut())
    }
}

impl<'a> TxToken for GmacTxToken<'a> {
    fn consume<R, F>(self, timestamp: smoltcp::time::Instant, len: usize, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>
    {
        // defmt::println!("TxCons: {=u32:08X}", timestamp.total_millis() as u32);
        let mut tf = self.gmac.alloc_write_frame().unwrap();
        let res = f(&mut tf[..len]);
        // defmt::trace!("TX: {=[u8]:02X}", &tf[..len]);
        tf.send(len);
        res
    }
}

impl<'a> Device<'a> for Gmac {
    type RxToken = GmacRxToken<'a>;
    type TxToken = GmacTxToken<'a>;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let rxf = self.read_frame()?;
        Some((
            GmacRxToken {
                rf: rxf,
                _plt: PhantomData,
            },
            GmacTxToken {
                gmac: self,
            }
        ))
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        Some(GmacTxToken {
            gmac: self
        })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut capa = DeviceCapabilities::default();
        capa.medium = Medium::Ethernet;
        capa.max_transmission_unit = 1024; // Is this too big?
        capa.max_burst_size = None;

        let mut cksm = ChecksumCapabilities::ignored();
        cksm.ipv4 = Checksum::None;
        cksm.tcp = Checksum::None;
        cksm.udp = Checksum::None;
        cksm.icmpv4 = Checksum::None;

        capa.checksum = cksm;
        capa
    }
}

pub struct Gmac {
    periph: GMAC,
    next_tx_idx: usize,
    last_txgo: bool,
    last_bna: bool,
    last_stat_poll: u32,
}

pub struct ReadFrame {
    bufr: NonNull<[u8; RX_BUF_SIZE]>,
    len: usize,
    desc: NonNull<RxBufferDescriptor>,
}

pub struct WriteFrame {
    bufr: NonNull<[u8; TX_BUF_SIZE]>,
    desc: NonNull<TxBufferDescriptor>,
    was_sent: bool,
}

impl Drop for WriteFrame {
    fn drop(&mut self) {
        if !self.was_sent {
            // defmt::println!("Dropping unsent frame!");
            unsafe { self.dropper(0) }
        }
    }
}

impl Deref for ReadFrame {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(self.bufr.as_ptr().cast(), self.len)
        }
    }
}

impl DerefMut for ReadFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(self.bufr.as_ptr().cast(), self.len)
        }
    }
}

impl Deref for WriteFrame {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe {
            core::slice::from_raw_parts(self.bufr.as_ptr().cast(), TX_BUF_SIZE)
        }
    }
}

impl DerefMut for WriteFrame {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            core::slice::from_raw_parts_mut(self.bufr.as_ptr().cast(), TX_BUF_SIZE)
        }
    }
}

impl Drop for ReadFrame {
    fn drop(&mut self) {
        // On drop, we must reset the header to "free" it.
        let desc = unsafe { self.desc.as_ref() };
        // Get w0 to figure out if this is the "last" item
        // TODO: Just check against buffer address?
        let is_last = (desc.get_word_0() & 0x0000_0002) != 0;
        let buf_addr = self.bufr.as_ptr();
        let buf_word = buf_addr as u32;
        let buf_word_msk = buf_word & 0xFFFF_FFFC;

        let last_word = if is_last { 0x0000_0002 } else { 0x0000_0000 };

        // defmt::println!("Releasing ReadFrame @ {=u32:08X}", buf_word_msk);

        // Note, bit 0 is ALWAYS cleared here, which marks the buffer as ready for
        // re-use by the GMAC
        desc.set_word_0(buf_word_msk | last_word);
    }
}

impl WriteFrame {
    unsafe fn dropper(&mut self, len: usize) {
        let desc = { self.desc.as_ref() };
        let old_w1 = desc.get_word_1();
        let wrap_bit = old_w1 & 0x4000_0000;
        let len = len.min(TX_BUF_SIZE).min(0x3FFF) as u32;

        let mut new_w1 = 0;
                                    // Bit 31 is zeroed to mark this as "ready"
        new_w1 |= wrap_bit;         // Bit 30: Wrap
                                    // Bits 29:17 are status/reserved bits, okay to clear
                                    // Bit 16 is zeroed to have CRC calc offloaded
        new_w1 |= 0x0000_8000;      // Bit 15: Last Buffer in Frame
                                    // Bit 14 is reserved
        new_w1 |= len;              // Bits 13:0: Size

        // Store the word to make it active for the transmit hardware to process.
        desc.set_word_1(new_w1);
        self.was_sent = true;

        // yolo
        {
            let gmac = &*GMAC::ptr();
            gmac.gmac_ncr.modify(|_r, w| {
                w.tstart().set_bit()
            });
        }
    }

    pub fn send(mut self, len: usize) {
        unsafe { self.dropper(len) };
    }
}
impl GmacPins for () {
    
}

impl Gmac {
    // TODO: Mark safe when possible.
    // pub unsafe fn new(periph: GMAC) -> Self {
    //     // defmt::println!("WARNING: Don't forget! We rely on configuration elsewhere for pins and stuff.");
    //     Self {
    //         periph,
    //         next_tx_idx: 0,
    //         last_txgo: false,
    //         last_bna: false,
    //         last_stat_poll: 0,
    //     }
    // }
    // #[inline]
    // fn reg(&self) -> &RegisterBlock {
    //     unsafe { &*self::REG }
    // }

    pub fn  new_gmac<Pins: GmacPins>(gmac: GMAC, _pins: Pins, cfg: GmacConfiguration, pmc: &mut Pmc) -> Result<Self, GmacError> {
        Self::new(gmac, pmc, cfg)
    }


    fn new(gmac: GMAC, pmc: &mut Pmc, cfg: GmacConfiguration) -> Result<Self,GmacError> {
        pmc.enable_peripherals(&[PeripheralIdentifier::GMAC]).unwrap();
        let mut gmac = Gmac {
            periph: gmac,
            next_tx_idx: 0,
            last_txgo: false,
            last_bna: false,
            last_stat_poll: 0,
        };

        Ok(gmac)
    }

    pub fn read_frame(&mut self) -> Option<ReadFrame> {
        // Scan through the read frames, and attempt to find one marked as "used"
        RX_BUF_DESCS.iter().find_map(|desc| {
            let w0 = desc.get_word_0();
            let addr = w0 & 0xFFFF_FFFC;
            let ready = (w0 & 0x0000_0001) != 0;

            if ready && (addr != 0) {
                // Erase address, but leave 'ready' and potentially 'last' bit set.
                desc.set_word_0(w0 & 0x0000_0003);
                let len = (desc.get_word_1() & 0x0000_0FFF) as usize;

                let desc_addr = NonNull::new(desc.words.get().cast())?;
                let buf_addr = NonNull::new(addr as *const [u8; RX_BUF_SIZE] as *mut [u8; RX_BUF_SIZE])?;
                Some(ReadFrame { bufr: buf_addr, len, desc: desc_addr })
            } else {
                None
            }
        })
    }

    pub fn alloc_write_frame(&mut self) -> Option<WriteFrame> {
        // let tsr = self.periph.gmac_tsr.read().bits();
        // defmt::println!("TSR: {=u32:08x}", tsr);

        let desc = &TX_BUF_DESCS[self.next_tx_idx];
        let w1 = desc.get_word_1();

        // Is this packet ready to be used by software?
        let ready = (w1 & 0x8000_0000) != 0;
        if !ready {
            return None;
        }

        // Yes, it is. Clear out old status registers, but leave the used (and wrap) bit set.
        // Also update the "next index".
        let wrap_bit = w1 & 0x4000_0000;
        desc.set_word_1(0x8000_0000 | wrap_bit);

        let cur_idx = self.next_tx_idx;

        self.next_tx_idx = (cur_idx + 1) % NUM_TX_BUFS;

        Some(WriteFrame {
            bufr: NonNull::new(TX_BUFS[cur_idx].buf.get().cast())?,
            desc: NonNull::new(TX_BUF_DESCS[cur_idx].words.get().cast())?,
            was_sent: false,
        })
    }

    fn miim_mgmt_port_enable(&mut self) {
        self.periph.gmac_ncr.modify(|_r, w| {
            w.mpe().set_bit()
        });
    }

    fn miim_mgmt_port_disable(&mut self) {
        self.periph.gmac_ncr.modify(|_r, w| {
            w.mpe().clear_bit()
        });
    }

    fn miim_is_busy(&mut self) -> bool {
        self.periph.gmac_nsr.read().idle().bit_is_clear()
    }

    fn miim_write_data(&mut self, reg_idx: u8, op_data: u16) {
        self.periph.gmac_man.write(|w| {
            w.wzo().clear_bit();
            w.cltto().set_bit();
            unsafe {
                w.op().bits(0b01);
                w.wtn().bits(0b10);
                // TODO: Hardcoded PHY Address
                w.phya().bits(0);
                w.rega().bits(reg_idx);
                w.data().bits(op_data);
            }
            w
        });
    }

    fn miim_start_read(&mut self, reg_idx: u8) {
        self.periph.gmac_man.write(|w| {
            w.wzo().clear_bit();
            w.cltto().set_bit();
            unsafe {
                w.op().bits(0b10);
                w.wtn().bits(0b10);
                // TODO: Hardcoded PHY Address
                w.phya().bits(0);
                w.rega().bits(reg_idx);
                w.data().bits(0);
            }
            w
        });
    }

    fn miim_read_data_get(&mut self) -> u16 {
        self.periph.gmac_man.read().data().bits()
    }

    pub fn miim_post_setup(&mut self) {

        // defmt::println!("Starting MIIM setup");
        // defmt::println!("Enabling management port...");
        self.miim_mgmt_port_enable();

        // defmt::println!("Waiting for miim idle...");
        let val = self.periph.gmac_nsr.read().bits();
        // defmt::println!("{=u32:08X}", val);
        while self.miim_is_busy() { }

        // defmt::println!("Reset PHY...");

        self.miim_write_data(0, 0x8000); // 0.15: Software reset

        // TODO: How long SHOULD this be?
        while self.miim_is_busy() { }

        self.miim_start_read(0);
        while self.miim_is_busy() { }
        let val = self.miim_read_data_get();
        // defmt::println!("New Reg 0 Val: {=u16:04X}", val);

        // TODO: Skipping Autonegotiation Adv step (reg 4)...
        // TODO: Skipping Autonegotiation restart since we didn't change anything...

        // Wait for link to come up
        loop {
            self.miim_start_read(1);
            while self.miim_is_busy() { }
            let val = self.miim_read_data_get();
            if (val & 0x0004) != 0 {
                // defmt::println!("Link up!");
                break;
            }
        }

        self.miim_mgmt_port_disable();
    }

    pub fn init(&mut self) {
        // Based on DRV_PIC32CGMAC_LibInit
        // //disable Tx
        // GMAC_REGS->GMAC_NCR &= ~GMAC_NCR_TXEN_Msk;
        // //disable Rx
        // GMAC_REGS->GMAC_NCR &= ~GMAC_NCR_RXEN_Msk;
        self.periph.gmac_ncr.modify(|_r, w| {
            w.txen().clear_bit();
            w.rxen().clear_bit();
            w
        });

        if self.miim_is_busy() {
            // defmt::println!("Busy at start???");
        }

        // //disable all GMAC interrupts for QUEUE 0
        // GMAC_REGS->GMAC_IDR = GMAC_INT_ALL;
        self.periph.gmac_idr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });

        // //disable all GMAC interrupts for QUEUE 1
        // GMAC_REGS->GMAC_IDRPQ[0] = GMAC_INT_ALL;
        // //disable all GMAC interrupts for QUEUE 2
        // GMAC_REGS->GMAC_IDRPQ[1] = GMAC_INT_ALL;
        // //disable all GMAC interrupts for QUEUE 3
        // GMAC_REGS->GMAC_IDRPQ[2] = GMAC_INT_ALL;
        // //disable all GMAC interrupts for QUEUE 4
        // GMAC_REGS->GMAC_IDRPQ[3] = GMAC_INT_ALL;
        // //disable all GMAC interrupts for QUEUE 5
        // GMAC_REGS->GMAC_IDRPQ[4] = GMAC_INT_ALL;
        for i in 0..5 {
            self.periph.gmac_idrpq[i].write(|w| unsafe { w.bits(0xFFFF_FFFF) });
        }

        // //Clear statistics register
        // GMAC_REGS->GMAC_NCR |=  GMAC_NCR_CLRSTAT_Msk;
        self.periph.gmac_ncr.modify(|_r, w| {
            w.clrstat().set_bit()
        });

        // //Clear RX Status
        // GMAC_REGS->GMAC_RSR =  GMAC_RSR_RXOVR_Msk | GMAC_RSR_REC_Msk | GMAC_RSR_BNA_Msk  | GMAC_RSR_HNO_Msk;
        self.periph.gmac_rsr.write(|w| {
            w.rxovr().set_bit();
            w.rec().set_bit();
            w.bna().set_bit();
            w.hno().set_bit();
            w
        });

        // //Clear TX Status
        // GMAC_REGS->GMAC_TSR = GMAC_TSR_UBR_Msk  | GMAC_TSR_COL_Msk  | GMAC_TSR_RLE_Msk | GMAC_TSR_TXGO_Msk |
        //                                         GMAC_TSR_TFC_Msk  | GMAC_TSR_TXCOMP_Msk  | GMAC_TSR_HRESP_Msk;
        self.periph.gmac_tsr.write(|w| {
            w.ubr().set_bit();
            w.col().set_bit();
            w.rle().set_bit();
            w.txgo().set_bit();
            w.tfc().set_bit();
            w.txcomp().set_bit();
            w.hresp().set_bit();
            w
        });

        // //Clear all GMAC Interrupt status
        // GMAC_REGS->GMAC_ISR;
        let _ = self.periph.gmac_isr.read().bits();

        // GMAC_REGS->GMAC_ISRPQ[0] ;
        // GMAC_REGS->GMAC_ISRPQ[1] ;
        // GMAC_REGS->GMAC_ISRPQ[2] ;
        // GMAC_REGS->GMAC_ISRPQ[3] ;
        // GMAC_REGS->GMAC_ISRPQ[4] ;
        for i in 0..5 {
            let _ = self.periph.gmac_isrpq[i].read().bits();
        }

        // //Set network configurations like speed, full duplex, copy all frames, no broadcast,
        // // pause enable, remove FCS, MDC clock
        // GMAC_REGS->GMAC_NCFGR = GMAC_NCFGR_SPD(1) | GMAC_NCFGR_FD(1) | GMAC_NCFGR_DBW(0) | GMAC_NCFGR_CLK(4)  | GMAC_NCFGR_PEN(1)  | GMAC_NCFGR_RFCS(1);
        // if((pMACDrv->sGmacData.gmacConfig.checksumOffloadRx) != TCPIP_MAC_CHECKSUM_NONE)
        // {
        //     GMAC_REGS->GMAC_NCFGR |= GMAC_NCFGR_RXCOEN_Msk;
        // }
        self.periph.gmac_ncfgr.write(|w| {
            w.spd().set_bit();
            w.fd().set_bit();
            unsafe {
                // 0 = 32-bit data bus
                w.dbw().bits(0);
            }
            // 4 == mck / 64
            w.clk().mck_64();
            w.pen().set_bit();
            w.rfcs().clear_bit();
            // Note: Always enabling checksum offloading for now
            w.rxcoen().set_bit();
            w
        });


        // // Set MAC address
        // DRV_PIC32CGMAC_LibSetMacAddr((const uint8_t *)(pMACDrv->sGmacData.gmacConfig.macAddress.v));

        // For now, use (one of) Microchip's MACs. This is temporary.
        //
        // 04-91-62   (hex)        Microchip Technology Inc.
        // 049162     (base 16)    Microchip Technology Inc.
        //                         2355 W. Chandler Blvd.
        //                         Chandler  AZ  85224
        //                         US

        // GMAC_REGS->GMAC_SA[0].GMAC_SAB = (pMacAddr[3] << 24)
        //                             | (pMacAddr[2] << 16)
        //                             | (pMacAddr[1] <<  8)
        //                             | (pMacAddr[0]);

        // GMAC_REGS->GMAC_SA[0].GMAC_SAT = (pMacAddr[5] <<  8)
        //                             | (pMacAddr[4]) ;

        //  0  1  2  3  4  5
        // 04:91:62:01:02:03

        self.periph.gmac_sa1.gmac_sab.write(|w| unsafe {
            let bottom: u32 = {
                (0x01 << 24) |
                (0x62 << 16) |
                (0x91 <<  8) |
                0x04
            };
            w.addr().bits(bottom)
        });
        self.periph.gmac_sa1.gmac_sat.write(|w| unsafe {
            let top: u16 = {
                (0x03 << 8) |
                0x02
            };
            w.addr().bits(top)
        });

        // // MII mode config
        // //Configure in RMII mode
        // if((TCPIP_INTMAC_PHY_CONFIG_FLAGS) & DRV_ETHPHY_CFG_RMII)
        //     GMAC_REGS->GMAC_UR = GMAC_UR_RMII(0); //initial mode set as RMII
        // else
        //     GMAC_REGS->GMAC_UR = GMAC_UR_RMII(1); //initial mode set as MII

        // We have an RMII Phy.
        self.periph.gmac_ur.write(|w| {
            // 0 => RMII
            // 1 => MII
            w.rmii().clear_bit()
        });

        // DRV_PIC32CGMAC_LibRxFilterHash_Calculate
        //
        // Note: Set to all 1's to accept all multi-cast addresses
        self.periph.gmac_hrb.write(|w| unsafe { w.addr().bits(0xFFFF_FFFF) });
        self.periph.gmac_hrt.write(|w| unsafe { w.addr().bits(0xFFFF_FFFF) });

        // _DRV_GMAC_MacToEthFilter
        //
        // Let's just leave these as defaults, they look sane, but meh.

        // DRV_PIC32CGMAC_LibRxQueFilterInit
        //
        // Let's skip priority filters for now...

        // DRV_PIC32CGMAC_LibRxInit
        //
        // This boils down to a single write to GMAC_RBQB (or GMAC_RBQBAPQ), I think this means
        // I need to set up the receive buffers. NOTE: I think they need to be 8-byte aligned (or something?)
        // (datasheet says 4-byte aligned...)
        //
        // Table 38-2 describes "Receive Buffer Descriptor Entry"
        unsafe {
            // Set the receive buffer addresses in the upper word
            for (desc, buf) in RX_BUF_DESCS.iter().zip(RX_BUFS.iter()) {
                // Take the buffer pointer...
                let buf_addr_ptr: *mut u8 = buf.buf.get().cast();
                let buf_wrd_raw: u32 = buf_addr_ptr as u32;
                let buf_wrd_msk: u32 = buf_wrd_raw & 0xFFFF_FFFC;
                // defmt::assert_eq!(buf_wrd_raw, buf_wrd_msk, "RX Buf Alignment Wrong!");

                // ...and store it in the buffer descriptor
                desc.set_word_0(buf_wrd_msk);
            }

            // This is probably UB and should be fixed...
            let last = &RX_BUF_DESCS[NUM_RX_BUFS - 1];
            let mut word_0 = last.get_word_0();

            // Mark as last buffer
            word_0 |= 0x0000_0002;
            last.set_word_0(word_0);
        }

        // TODO: I *think* I need to set DCFGR.DRBS = (1024 / 64) = 16 = 0x10
        // This is done "later" in DRV_PIC32CGMAC_LibInitTransfer

        self.periph.gmac_rbqb.write(|w| unsafe {
            // Take the buffer descriptor pointer...
            let desc_ptr: *const RxBufferDescriptor = RX_BUF_DESCS.as_ptr();
            let desc_wrd_raw: u32 = desc_ptr as u32;
            let desc_wrd_msk: u32 = desc_wrd_raw & 0xFFFF_FFFC;

            // defmt::assert_eq!(desc_wrd_raw, desc_wrd_msk, "RX Buf Desc Alignment Wrong!");

            // ... and store it in the RBQB register
            w.bits(desc_wrd_msk)
        });

        // DRV_PIC32CGMAC_LibTxInit
        //
        // Again, this boils down to essentially a single write to GMAC_TBQB, similar to above.
        //
        // Table 38-3 describes "Transmit Buffer Descriptor Entry"
        unsafe {
            // Set the transmit buffer addresses in the upper word
            for (desc, buf) in TX_BUF_DESCS.iter().zip(TX_BUFS.iter()) {
                // Take the buffer pointer...
                let buf_addr_ptr: *mut u8 = buf.buf.get().cast();
                let buf_wrd_raw: u32 = buf_addr_ptr as u32;
                let buf_wrd_msk: u32 = buf_wrd_raw & 0xFFFF_FFFC;
                // defmt::assert_eq!(buf_wrd_raw, buf_wrd_msk, "TX Buf Alignment Wrong!");

                // ...and store it in the buffer descriptor
                desc.set_word_0(buf_wrd_msk);

                // Mark this buffer as "used" by software, so the hardware will
                // not attempt to use this buffer until later.
                desc.set_word_1(0x8000_0000);
            }

            // This is probably UB and should be fixed...
            let last = &TX_BUF_DESCS[NUM_TX_BUFS - 1];
            let mut word_1 = last.get_word_1();

            // Mark as wrap buffer
            word_1 |= 0x4000_0000;
            last.set_word_1(word_1);
        }

        self.periph.gmac_tbqb.write(|w| unsafe {
            // Take the buffer descriptor pointer...
            let desc_ptr: *const TxBufferDescriptor = TX_BUF_DESCS.as_ptr();
            let desc_wrd_raw: u32 = desc_ptr as u32;
            let desc_wrd_msk: u32 = desc_wrd_raw & 0xFFFF_FFFC;

            // defmt::assert_eq!(desc_wrd_raw, desc_wrd_msk, "TX Buf Desc Alignment Wrong!");

            // ... and store it in the TBQB register
            w.bits(desc_wrd_msk)
        });

        // Note! We need to stub out the prio queues
        for buf in self.periph.gmac_tbqbapq.iter() {
            // Take the buffer descriptor pointer...
            let desc_ptr: *const TxBufferDescriptor = &UNUSED_TX_BUF_DESC;
            let desc_wrd_raw: u32 = desc_ptr as u32;
            let desc_wrd_msk: u32 = desc_wrd_raw & 0xFFFF_FFFC;

            // defmt::assert_eq!(desc_wrd_raw, desc_wrd_msk, "TX Buf Desc Alignment Wrong!");

            // ... and store it in the TBQB register
            buf.write(|w| unsafe { w.bits(desc_wrd_msk) });
        }

        // DRV_PIC32CGMAC_LibInitTransfer
        let drbs = (RX_BUF_SIZE / 64).min(255) as u8;
        // defmt::assert_ne!(drbs, 0, "Invalid RX Buffer size!");

        self.periph.gmac_dcfgr.write(|w| {
            // ? - DMA Discard Receive Packets
            //
            // 0 - Received packets are stored in the SRAM based packet buffer until next AHB buffer
            // resource becomes available.
            //
            // 1 - Receive packets from the receiver packet buffer memory are automatically discarded when
            // no AHB resource is available.
            //
            // TODO: Example code sets this, so let's do that for now.
            w.ddrp().set_bit();
            unsafe {
                // DRBS is defined in multiples of 64-bytes
                w.drbs().bits(drbs);
            }
            w.txcoen().set_bit();   // Enable Checksum Offload
            w.txpbms().set_bit();   // Use full 4KiB of TX space (???)
            w.rxbms().full();       // Use full 4KiB of RX space (???)
            w.espa().clear_bit();   // Disable endianness swap for packet data access
            w.esma().clear_bit();   // Disable endianness swap for management desc access
            w.fbldo().incr4();      // AHB increments of 4 (???)

            w
        });

        // TODO(AJM): We do NOT enable any interrupts at this point. For early bringup,
        // I plan to poll the relevant status registers. This will change at some point.
        //
        // This note applies to the behavior at the end of DRV_PIC32CGMAC_LibInitTransfer,
        // as well as the next two steps.

        // DRV_PIC32CGMAC_LibSysIntStatus_Clear
        // DRV_PIC32CGMAC_LibSysInt_Enable

        // DRV_PIC32CGMAC_LibTransferEnable
        self.periph.gmac_ncr.modify(|_r, w| {
            w.txen().set_bit();
            w.rxen().set_bit();
            w.westat().set_bit();
            w
        });

    }
}

// Note: MIIM == MDIO == SMI

// Relevant driver call chain
//
// DRV_GMAC_Initialize
//     DRV_PIC32CGMAC_LibSysInt_Disable
//         * Not much, just disabling interrupts?
//     _DRV_GMAC_PHYInitialise
//         * DRV_ETHPHY_Initialize
//             * Data structure init?
//         * DRV_ETHPHY_Open
                // _DRV_ETHPHY_ClientObjectAllocate
                //     * Data structures...
                // DRV_MIIM_Open
                //     _DRV_MIIM_GetObjectAndLock
                //         * Data structures...
                //     _DRV_MIIM_ClientAllocate
                //         * Data structures...
                //     _DRV_MIIM_ObjUnlock
                //         * FreeRTOS stuff?
//     DRV_PIC32CGMAC_LibInit
//         Important! See below
//     DRV_PIC32CGMAC_LibRxFilterHash_Calculate
//         Important! (I think?)
//     _DRV_GMAC_MacToEthFilter
//         Used to calculate GMAC_NCFGR ?
//     DRV_PIC32CGMAC_LibRxQueFilterInit
//         Used to calculate priority filters? Unsure if necessary
//     DRV_PIC32CGMAC_LibRxInit
//     DRV_PIC32CGMAC_LibTxInit
//     for each queue:
//         DRV_PIC32CGMAC_LibInitTransfer
//     DRV_PIC32CGMAC_LibSysIntStatus_Clear
//     DRV_PIC32CGMAC_LibSysInt_Enable
//     DRV_PIC32CGMAC_LibTransferEnable
//     DRV_GMAC_EventInit
//     if failed:
//         _MACDeinit
//     "remaining initialization is done by DRV_ETHMAC_PIC32MACTasks"

#[repr(C, align(8))]
struct RxBufferDescriptor {
    words: UnsafeCell<[u32; 2]>,
}

impl RxBufferDescriptor {
    fn get_word_0(&self) -> u32 {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                // .add(0)
                .read_volatile()
        }
    }

    fn set_word_0(&self, val: u32) {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                // .add(0)
                .write_volatile(val)
        }
    }

    fn get_word_1(&self) -> u32 {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                .add(1)
                .read_volatile()
        }
    }

    fn set_word_1(&self, val: u32) {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                .add(1)
                .write_volatile(val)
        }
    }
}

impl TxBufferDescriptor {
    fn get_word_0(&self) -> u32 {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                // .add(0)
                .read_volatile()
        }
    }

    fn set_word_0(&self, val: u32) {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                // .add(0)
                .write_volatile(val)
        }
    }

    fn get_word_1(&self) -> u32 {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                .add(1)
                .read_volatile()
        }
    }

    fn set_word_1(&self, val: u32) {
        unsafe {
            self.words
                .get()
                .cast::<u32>()
                .add(1)
                .write_volatile(val)
        }
    }
}

macro_rules! impl_gmac_pins {
    (
        $(
            $( #[$cfg:meta] )?
            $Gmac:ident: {
                GTXCK: [ $GtxckType:ty ],
                GTXEN: [ $GtxenType:ty ],
                GTX3: [ $Gtx3Type:ty ],
                GTX2: [ $Gtx2Type:ty ],
                GTX1: [ $Gtx1Type:ty ],
                GTX0: [ $Gtx0Type:ty ],
                GTXER: [ $GtxerType:ty ],
                GRXCK: [ $GrxckType:ty ],
                GRXDV: [ $GrxdvType:ty ],
                GRX3: [ $Grx3Type:ty ],
                GRX2: [ $Grx2Type:ty ],
                GRX1: [ $Grx1Type:ty ],
                GRX0: [ $Grx0Type:ty ],
                GRXER: [ $GrxerType:ty ],
                GCRS: [ $GcrsType:ty ],
                GCOL: [ $GcolType:ty ],
                GMDC: [ $GmdcType:ty ],
                GMDIO: [ $GmdioType:ty ],
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg] )?
                mod [<$Gmac:lower _impl>] {
                    use super::*;

                    #[doc = "Trait that identifies valid GTXCK [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GtxckPin>] {}
                    #[doc = "Trait that identifies valid GTXEN [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GtxenPin>] {}
                    #[doc = "Trait that identifies valid GTX3 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Gtx3Pin>] {}
                    #[doc = "Trait that identifies valid GTX2 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Gtx2Pin>] {}
                    #[doc = "Trait that identifies valid GTX1 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Gtx1Pin>] {}
                    #[doc = "Trait that identifies valid GTX0 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Gtx0Pin>] {}
                    #[doc = "Trait that identifies valid GTXER [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GtxerPin>] {}
                    #[doc = "Trait that identifies valid GRXCK [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GrxckPin>] {}
                    #[doc = "Trait that identifies valid GRXDV [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GrxdvPin>] {}
                    #[doc = "Trait that identifies valid GRX3 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Grx3Pin>] {}
                    #[doc = "Trait that identifies valid GRX2 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Grx2Pin>] {}
                    #[doc = "Trait that identifies valid GRX1 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Grx1Pin>] {}
                    #[doc = "Trait that identifies valid GRX0 [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac Grx0Pin>] {}
                    #[doc = "Trait that identifies valid GRXER [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GrxerPin>] {}
                    #[doc = "Trait that identifies valid GCRS [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GcrsPin>] {}
                    #[doc = "Trait that identifies valid GCOL [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GcolPin>] {}
                    #[doc = "Trait that identifies valid GMDC [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GmdcPin>] {}
                    #[doc = "Trait that identifies valid GMDIO [`Pin`]s for [`" [<$Gmac:upper>] "`]."]
                    pub trait [<$Gmac GmdioPin>] {}
                    
                    impl [<$Gmac GtxckPin>] for $GtxckType {}
                    impl [<$Gmac GtxenPin>] for $GtxenType {}
                    impl [<$Gmac Gtx3Pin>] for $Gtx3Type {}
                    impl [<$Gmac Gtx2Pin>] for $Gtx2Type {}
                    impl [<$Gmac Gtx1Pin>] for $Gtx1Type {}
                    impl [<$Gmac Gtx0Pin>] for $Gtx0Type {}
                    impl [<$Gmac GtxerPin>] for $GtxerType {}
                    impl [<$Gmac GrxckPin>] for $GrxckType {}
                    impl [<$Gmac GrxdvPin>] for $GrxdvType {}
                    impl [<$Gmac Grx3Pin>] for $Grx3Type {}
                    impl [<$Gmac Grx2Pin>] for $Grx2Type {}
                    impl [<$Gmac Grx1Pin>] for $Grx1Type {}
                    impl [<$Gmac Grx0Pin>] for $Grx0Type {}
                    impl [<$Gmac GrxerPin>] for $GrxerType {}
                    impl [<$Gmac GcrsPin>] for $GcrsType {}
                    impl [<$Gmac GcolPin>] for $GcolType {}
                    impl [<$Gmac GmdcPin>] for $GmdcType {}
                    impl [<$Gmac GmdioPin>] for $GmdioType {}

                    impl [<GmacPins>] for (
                        $GtxckType,                           
                        $GtxenType,
                        $Gtx3Type, 
                        $Gtx2Type, 
                        $Gtx1Type, 
                        $Gtx0Type, 
                        $GtxerType,
                        $GrxckType,
                        $GrxdvType,
                        $Grx3Type, 
                        $Grx2Type, 
                        $Grx1Type, 
                        $Grx0Type, 
                        $GrxerType,
                        $GcrsType, 
                        $GcolType, 
                        $GmdcType, 
                        $GmdioType, 
                    ) {}

                    impl [<GmacPins>] for (
                        $GtxckType,                           
                        $GtxenType,
                        $Gtx1Type, 
                        $Gtx0Type, 
                        $GrxdvType,
                        $Grx1Type, 
                        $Grx0Type, 
                        $GrxerType,
                        $GmdcType, 
                        $GmdioType, 
                    ) {}



                }
                $( #[$cfg] )?
                    pub use [<$Gmac:lower _impl>]::*;
            )+
        }
    };
}

impl_gmac_pins!(
    Gmac: { 
        GTXCK: [ Pin<PD0, PeripheralA> ],
        GTXEN: [ Pin<PD1, PeripheralA> ],
        GTX3: [ Pin<PD16, PeripheralA> ],
        GTX2: [ Pin<PD15, PeripheralA> ],
        GTX1: [ Pin<PD3, PeripheralA> ],
        GTX0: [ Pin<PD2, PeripheralA> ],
        GTXER: [ Pin<PD17, PeripheralA> ],
        GRXCK: [ Pin<PD14, PeripheralA> ],
        GRXDV: [ Pin<PD4, PeripheralA> ],
        GRX3: [ Pin<PD12, PeripheralA> ],
        GRX2: [ Pin<PD11, PeripheralA> ],
        GRX1: [ Pin<PD6, PeripheralA> ],
        GRX0: [ Pin<PD5, PeripheralA> ],
        GRXER: [ Pin<PD7, PeripheralA> ],
        GCRS: [ Pin<PD10, PeripheralA> ],
        GCOL: [ Pin<PD13, PeripheralA> ],
        GMDC: [ Pin<PD8, PeripheralA> ],
        GMDIO: [ Pin<PD9, PeripheralA> ],
        
    },
);

#[repr(C, align(8))]
struct TxBufferDescriptor {
    // TODO: Bitfields for this!
    words: UnsafeCell<[u32; 2]>,
}

#[repr(C, align(8))]
struct RxBuffer {
    buf: UnsafeCell<[u8; RX_BUF_SIZE]>,
}

#[repr(C, align(8))]
struct TxBuffer {
    buf: UnsafeCell<[u8; TX_BUF_SIZE]>,
}

unsafe impl Sync for RxBuffer { }
unsafe impl Sync for TxBuffer { }
unsafe impl Sync for RxBufferDescriptor { }
unsafe impl Sync for TxBufferDescriptor { }

const RX_BUF_DEFAULT: RxBuffer = RxBuffer { buf: UnsafeCell::new([0u8; RX_BUF_SIZE]) };
const TX_BUF_DEFAULT: TxBuffer = TxBuffer { buf: UnsafeCell::new([0u8; TX_BUF_SIZE]) };

const RX_BUF_DESC_DEFAULT: RxBufferDescriptor = RxBufferDescriptor {
    words: UnsafeCell::new([0u32; 2]),
};

const TX_BUF_DESC_DEFAULT: TxBufferDescriptor = TxBufferDescriptor {
    words: UnsafeCell::new([0u32; 2]),
};
