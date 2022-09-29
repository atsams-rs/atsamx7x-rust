use crate::clocks::*;
use crate::pac::gmac::RegisterBlock;
use crate::pio::*;
use core::sync::atomic::{compiler_fence, fence, Ordering};
use core::{
    assert,
    cell::UnsafeCell,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};
use paste::*;

use crate::pac::{
    generic::Reg,
    gmac::{idrpq::IDRPQ_SPEC, isrpq::ISRPQ_SPEC},
    GMAC,
};
use smoltcp::phy::{
    Checksum, ChecksumCapabilities, Device, DeviceCapabilities, Medium, RxToken, TxToken,
};

pub trait GmacMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

pub trait GmacPins {}

#[derive(Debug)]
pub enum GmacError {
    ClockRateError,
}

pub struct GmacConfiguration {
    pub speed: GmacSpeed,
    pub duplex: GmacDuplex,
    pub mii: GmacMii,
    pub mac: [u8; 6],
}

pub enum GmacSpeed {
    _100Mbit,
    _10Mbit,
}

pub enum GmacDuplex {
    HalfDuplex,
    FullDuplex,
}

pub enum GmacMii {
    Mii,
    Rmii,
}

pub struct GmacRxToken<'a, const RX_S: usize> {
    rf: ReadFrame<RX_S>,
    _plt: PhantomData<&'a ()>,
}
pub struct GmacTxToken<
    'a,
    const TX_N: usize,
    const TX_S: usize,
    const RX_N: usize,
    const RX_S: usize,
> {
    gmac_buf: &'a mut GmacBuffers<TX_N, TX_S, RX_N, RX_S>,
    // tf: WriteFrame<TX_S>,
    _plt: PhantomData<&'a ()>,
    // gmac: &'a mut Gmac<'a, TX_N, TX_S, RX_N, RX_S>,
}

impl<'a, const RX_S: usize> RxToken for GmacRxToken<'a, RX_S> {
    fn consume<R, F>(mut self, timestamp: smoltcp::time::Instant, f: F) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        f(self.rf.deref_mut())
    }
}

impl<'a, const TX_N: usize, const TX_S: usize, const RX_N: usize, const RX_S: usize> TxToken
    for GmacTxToken<'a, TX_N, TX_S, RX_N, RX_S>
{
    fn consume<R, F>(
        self,
        timestamp: smoltcp::time::Instant,
        len: usize,
        f: F,
    ) -> smoltcp::Result<R>
    where
        F: FnOnce(&mut [u8]) -> smoltcp::Result<R>,
    {
        let mut tf = self.gmac_buf.alloc_write_frame().unwrap();
        let res = f(&mut tf[..len]);
        tf.send(len);
        res
    }
}

impl<'a, const TX_N: usize, const TX_S: usize, const RX_N: usize, const RX_S: usize> Device<'a>
    for Gmac<'_, TX_N, TX_S, RX_N, RX_S>
{
    type RxToken = GmacRxToken<'a, RX_S>;
    type TxToken = GmacTxToken<'a, TX_N, TX_S, RX_N, RX_S>;

    fn receive(&'a mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        let rxf = self.read_frame()?;
        // let txf = self.alloc_write_frame()?;
        Some((
            GmacRxToken {
                rf: rxf,
                _plt: PhantomData,
            },
            GmacTxToken {
                gmac_buf: &mut self.gmac_buffers,
                _plt: PhantomData,
            },
        ))
    }

    fn transmit(&'a mut self) -> Option<Self::TxToken> {
        Some(GmacTxToken {
            gmac_buf: &mut self.gmac_buffers,
            _plt: PhantomData,
        })
    }

    fn capabilities(&self) -> DeviceCapabilities {
        let mut capa = DeviceCapabilities::default();
        capa.medium = Medium::Ethernet;
        capa.max_transmission_unit = TX_S; // Is this too big? I think we are capable of full ethernet frames
        capa.max_burst_size = None;

        // TODO The Gmac can do checksum offloading, and I believe is configured to do checksum
        // offloading.
        let mut cksm = ChecksumCapabilities::ignored();
        cksm.ipv4 = Checksum::Both;
        cksm.tcp = Checksum::Both;
        cksm.udp = Checksum::Both;
        cksm.icmpv4 = Checksum::None;

        capa.checksum = cksm;
        capa
    }
}

pub struct Gmac<'a, const TX_N: usize, const TX_S: usize, const RX_N: usize, const RX_S: usize> {
    periph: GMAC,

    gmac_buffers: &'a mut GmacBuffers<TX_N, TX_S, RX_N, RX_S>,
    unused_tx_buf_desc: TxBufferDescriptor,
}

pub struct ReadFrame<const RX_S: usize> {
    bufr: NonNull<[u8; RX_S]>,
    len: usize,
    desc: NonNull<RxBufferDescriptor>,
}

pub struct WriteFrame<const TX_S: usize> {
    bufr: NonNull<[u8; TX_S]>,
    desc: NonNull<TxBufferDescriptor>,
    was_sent: bool,
}

impl<const TX_S: usize> Drop for WriteFrame<TX_S> {
    fn drop(&mut self) {
        if !self.was_sent {}
    }
}

impl<const RX_S: usize> Deref for ReadFrame<RX_S> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.bufr.as_ptr().cast(), self.len) }
    }
}

impl<const RX_S: usize> DerefMut for ReadFrame<RX_S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.bufr.as_ptr().cast(), self.len) }
    }
}

impl<const TX_S: usize> Deref for WriteFrame<TX_S> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { core::slice::from_raw_parts(self.bufr.as_ptr().cast(), TX_S) }
    }
}

impl<const TX_S: usize> DerefMut for WriteFrame<TX_S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { core::slice::from_raw_parts_mut(self.bufr.as_ptr().cast(), TX_S) }
    }
}

impl<const RX_S: usize> Drop for ReadFrame<RX_S> {
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

        // Note, bit 0 is ALWAYS cleared here, which marks the buffer as ready for
        // re-use by the GMAC
        desc.set_word_0(buf_word_msk | last_word);
    }
}

impl<const TX_S: usize> WriteFrame<TX_S> {
    unsafe fn dropper(&mut self, len: usize) {
        compiler_fence(Ordering::SeqCst);
        let desc = { self.desc.as_ref() };
        let old_w1 = desc.get_word_1();
        let wrap_bit = old_w1 & 0x4000_0000;
        let len = len.min(TX_S).min(0x3FFF) as u32;

        let mut new_w1 = 0;
        // Bit 31 is zeroed to mark this as "ready"
        new_w1 |= wrap_bit; // Bit 30: Wrap
                            // Bits 29:17 are status/reserved bits, okay to clear
                            // Bit 16 is zeroed to have CRC calc offloaded
        new_w1 |= 0x0000_8000; // Bit 15: Last Buffer in Frame
                               // Bit 14 is reserved
        new_w1 |= len; // Bits 13:0: Size

        // Store the word to make it active for the transmit hardware to process.
        desc.set_word_1(new_w1);
        self.was_sent = true;

        fence(Ordering::SeqCst);
        // yolo
        {
            let gmac = &*GMAC::ptr();
            gmac.ncr.modify(|_r, w| w.tstart().set_bit());
        }
    }

    pub fn send(mut self, len: usize) {
        unsafe { self.dropper(len) };
    }
}
impl GmacPins for () {}

impl<'a, const TX_N: usize, const TX_S: usize, const RX_N: usize, const RX_S: usize>
    Gmac<'a, TX_N, TX_S, RX_N, RX_S>
{
    pub fn new_gmac<Pins: GmacPins>(
        gmac: GMAC,
        _pins: Pins,
        cfg: GmacConfiguration,
        bufs: &'a mut GmacBuffers<TX_N, TX_S, RX_N, RX_S>,
        clk: &mut HostClock,
    ) -> Result<Self, GmacError> {
        Self::new(gmac, clk, cfg, bufs)
    }

    fn new(
        gmac: GMAC,
        clk: &mut HostClock,
        cfg: GmacConfiguration,
        bufs: &'a mut GmacBuffers<TX_N, TX_S, RX_N, RX_S>,
    ) -> Result<Self, GmacError> {
        clk.enable_peripheral(PeripheralIdentifier::GMAC);
        let mut gmac = Gmac {
            periph: gmac,
            gmac_buffers: bufs,

            unused_tx_buf_desc: TX_BUF_DESC_DEFAULT,
        };
        // TODO Remove if we can assume reset register values
        gmac.periph.ncr.modify(|_r, w| {
            w.txen().clear_bit();
            w.rxen().clear_bit();
            w
        });

        if gmac.miim_is_busy() {
            // defmt::println!("Busy at start???");
        }

        // //disable all GMAC interrupts for QUEUE 0
        gmac.periph.idr.write(|w| unsafe { w.bits(0xFFFF_FFFF) });

        // //disable all GMAC interrupts for QUEUE 1
        // //disable all GMAC interrupts for QUEUE 2
        // //disable all GMAC interrupts for QUEUE 3
        // //disable all GMAC interrupts for QUEUE 4
        // //disable all GMAC interrupts for QUEUE 5
        for i in 0..5 {
            gmac.periph.idrpq[i].write(|w| unsafe { w.bits(0xFFFF_FFFF) });
        }

        // TODO if we can assume reset conditions, we can also skip these three register writes
        // //Clear statistics register
        gmac.periph.ncr.modify(|_r, w| w.clrstat().set_bit());

        // //Clear RX Status
        gmac.periph.rsr.write(|w| {
            w.rxovr().set_bit();
            w.rec().set_bit();
            w.bna().set_bit();
            w.hno().set_bit();
            w
        });

        // //Clear TX Status
        gmac.periph.tsr.write(|w| {
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
        let _ = gmac.periph.isr.read().bits();

        for i in 0..5 {
            let _ = gmac.periph.isrpq[i].read().bits();
        }

        // //Set network configurations like speed, full duplex, copy all frames, no broadcast,
        // // pause enable, remove FCS, MDC clock
        gmac.periph.ncfgr.modify(|_, w| {
            match cfg.speed {
                GmacSpeed::_100Mbit => w.spd().set_bit(),
                GmacSpeed::_10Mbit => w.spd().clear_bit(),
            };
            match cfg.duplex {
                GmacDuplex::FullDuplex => w.fd().set_bit(),
                GmacDuplex::HalfDuplex => w.fd().clear_bit(),
            };
            w
        });
        match clk.freq().to_MHz() {
            0..=20 => {
                gmac.periph.ncfgr.modify(|_, w| w.clk().mck_8());
                Ok(())
            }
            21..=40 => {
                gmac.periph.ncfgr.modify(|_, w| w.clk().mck_16());
                Ok(())
            }
            41..=80 => {
                gmac.periph.ncfgr.modify(|_, w| w.clk().mck_32());
                Ok(())
            }
            81..=160 => {
                gmac.periph.ncfgr.modify(|_, w| w.clk().mck_64());
                Ok(())
            }
            161..=240 => {
                gmac.periph.ncfgr.modify(|_, w| w.clk().mck_96());
                Ok(())
            }
            241.. => Err(GmacError::ClockRateError),
        }?;
        gmac.periph.ncfgr.modify(|_, w| {
            unsafe {
                // 0 = 32-bit data bus
                w.dbw().bits(0);
            }
            w.pen().set_bit();
            w.rfcs().clear_bit();
            // Note: Always enabling checksum offloading for now
            w.rxcoen().set_bit();
            w
        });

        // // Set MAC address

        // For now, use (one of) Microchip's MACs. This is temporary.
        //
        // 04-91-62   (hex)        Microchip Technology Inc.
        // 049162     (base 16)    Microchip Technology Inc.
        //                         2355 W. Chandler Blvd.
        //                         Chandler  AZ  85224
        //                         US

        //  0  1  2  3  4  5
        // 04:91:62:01:02:03

        gmac.periph.gmac_sa[0].sab.write(|w| unsafe {
            let bottom: u32 = {
                ((cfg.mac[3] as u32) << 24)
                    | ((cfg.mac[2] as u32) << 16)
                    | ((cfg.mac[1] as u32) << 8)
                    | (cfg.mac[0] as u32)
            };
            w.addr().bits(bottom)
        });
        gmac.periph.gmac_sa[0].sat.write(|w| unsafe {
            let top: u16 = { ((cfg.mac[5] as u16) << 8) | (cfg.mac[4] as u16) };
            w.addr().bits(top)
        });

        // // MII mode config TODO Save user from mii config and pin config incompatibilities
        gmac.periph.ur.write(|w| {
            // 0 => RMII
            // 1 => MII
            match cfg.mii {
                GmacMii::Mii => w.rmii().set_bit(),
                GmacMii::Rmii => w.rmii().clear_bit(),
            }
        });

        // DRV_PIC32CGMAC_LibRxFilterHash_Calculate
        //
        // Note: Set to all 1's to accept all multi-cast addresses
        gmac.periph
            .hrb
            .write(|w| unsafe { w.addr().bits(0xFFFF_FFFF) });
        gmac.periph
            .hrt
            .write(|w| unsafe { w.addr().bits(0xFFFF_FFFF) });

        // _DRV_GMAC_MacToEthFilter
        //
        // Let's just leave these as defaults, they look sane, but meh.

        // DRV_PIC32CGMAC_LibRxQueFilterInit
        //
        // Let's skip priority filters for now...

        // DRV_PIC32CGMAC_LibRxInit
        // This boils down to a single write to GMAC_RBQB (or GMAC_RBQBAPQ), I think this means I need to set up the receive buffers. NOTE: I think they need to be 8-byte aligned (or something?) (datasheet says 4-byte aligned...) Table 38-2 describes "Receive Buffer Descriptor Entry"
        // Set the receive buffer addresses in the upper word
        // for (desc, buf) in gmac.RX_BUF_DESCS.iter().zip(gmac.RX_BUFS.iter()) {
        for (desc, buf) in gmac
            .gmac_buffers
            .rx_buf_desc
            .iter()
            .zip(gmac.gmac_buffers.rx_buf.iter())
        {
            // Take the buffer pointer...
            let buf_addr_ptr: *mut u8 = buf.buf.get().cast();
            let buf_wrd_raw: u32 = buf_addr_ptr as u32;
            let buf_wrd_msk: u32 = buf_wrd_raw & 0xFFFF_FFFC;
            assert!(buf_wrd_raw == buf_wrd_msk);
            // defmt::assert_eq!(buf_wrd_raw, buf_wrd_msk, "RX Buf Alignment Wrong!");

            // ...and store it in the buffer descriptor
            desc.set_word_0(buf_wrd_msk);
        }

        // This is probably UB and should be fixed...
        // let last = &gmac.RX_BUF_DESCS[RX_N - 1];
        let last = &gmac.gmac_buffers.rx_buf_desc[RX_N - 1];
        let mut word_0 = last.get_word_0();

        // Mark as last buffer
        word_0 |= 0x0000_0002;
        last.set_word_0(word_0);

        // TODO: I *think* I need to set DCFGR.DRBS = (1024 / 64) = 16 = 0x10
        // This is done "later" in DRV_PIC32CGMAC_LibInitTransfer

        gmac.periph.rbqb.write(|w| unsafe {
            // Take the buffer descriptor pointer...
            // let desc_ptr: *const RxBufferDescriptor = gmac.RX_BUF_DESCS.as_ptr();
            let desc_ptr: *const RxBufferDescriptor = gmac.gmac_buffers.rx_buf_desc.as_ptr();
            let desc_wrd_raw: u32 = desc_ptr as u32;
            let desc_wrd_msk: u32 = desc_wrd_raw & 0xFFFF_FFFC;
            assert!(desc_wrd_raw == desc_wrd_msk);

            // ... and store it in the RBQB register
            w.bits(desc_wrd_msk)
        });

        // DRV_PIC32CGMAC_LibTxInit
        //
        // Again, this boils down to essentially a single write to GMAC_TBQB, similar to above.
        //
        // Table 38-3 describes "Transmit Buffer Descriptor Entry"
        // Set the transmit buffer addresses in the upper word
        // for (desc, buf) in gmac.TX_BUF_DESCS.iter().zip(gmac.TX_BUFS.iter()) {
        for (desc, buf) in gmac
            .gmac_buffers
            .tx_buf_desc
            .iter()
            .zip(gmac.gmac_buffers.tx_buf.iter())
        {
            // Take the buffer pointer...
            let buf_addr_ptr: *mut u8 = buf.buf.get().cast();
            let buf_wrd_raw: u32 = buf_addr_ptr as u32;
            let buf_wrd_msk: u32 = buf_wrd_raw & 0xFFFF_FFFC;
            assert!(buf_wrd_raw == buf_wrd_msk);

            // ...and store it in the buffer descriptor
            desc.set_word_0(buf_wrd_msk);

            // Mark this buffer as "used" by software, so the hardware will
            // not attempt to use this buffer until later.
            desc.set_word_1(0x8000_0000);
        }

        // This is probably UB and should be fixed...
        let last = &(gmac.gmac_buffers.tx_buf_desc[TX_N - 1]);
        let mut word_1 = last.get_word_1();

        // Mark as wrap buffer
        word_1 |= 0x4000_0000;
        last.set_word_1(word_1);

        gmac.periph.tbqb.write(|w| unsafe {
            // Take the buffer descriptor pointer...
            // let desc_ptr: *const TxBufferDescriptor = gmac.TX_BUF_DESCS.as_ptr();
            let desc_ptr: *const TxBufferDescriptor = gmac.gmac_buffers.tx_buf_desc.as_ptr();
            let desc_wrd_raw: u32 = desc_ptr as u32;
            let desc_wrd_msk: u32 = desc_wrd_raw & 0xFFFF_FFFC;
            assert!(desc_wrd_raw == desc_wrd_msk);

            // ... and store it in the TBQB register
            w.bits(desc_wrd_msk)
        });

        // Note! We need to stub out the prio queues
        for buf in gmac.periph.tbqbapq.iter() {
            // Take the buffer descriptor pointer...
            let desc_ptr: *const TxBufferDescriptor = &(gmac.unused_tx_buf_desc);
            let desc_wrd_raw: u32 = desc_ptr as u32;
            let desc_wrd_msk: u32 = desc_wrd_raw & 0xFFFF_FFFC;
            assert!(desc_wrd_raw == desc_wrd_msk);

            // ... and store it in the TBQB register
            buf.write(|w| unsafe { w.bits(desc_wrd_msk) });
        }

        // DRV_PIC32CGMAC_LibInitTransfer
        // TODO DMA buffer and RX buffer do not need to be the same size.
        // Yes they do. 38.7.1.2
        let drbs = (RX_S / 64).min(255) as u8;

        gmac.periph.dcfgr.write(|w| {
            // ? - DMA Discard Receive Packets
            //
            // 0 - Received packets are stored in the SRAM based packet buffer until next AHB buffer
            // resource becomes available.
            //
            // 1 - Receive packets from the receiver packet buffer memory are automatically discarded when
            // no AHB resource is available.
            //
            // TODO: Example code sets this, so let's do that for now.
            // TODO: Probs clear it
            w.ddrp().set_bit();
            unsafe {
                // DRBS is defined in multiples of 64-bytes
                w.drbs().bits(drbs);
            }
            w.txcoen().set_bit(); // Enable Checksum Offload
            w.txpbms().set_bit(); // Use full 4KiB of TX space (???)
            w.rxbms().full(); // Use full 4KiB of RX space (???)
            w.espa().clear_bit(); // Disable endianness swap for packet data access
            w.esma().clear_bit(); // Disable endianness swap for management desc access
            w.fbldo().incr4(); // AHB increments of 4 (???)

            w
        });

        // TODO(AJM): We do NOT enable any interrupts at this point. For early bringup,
        // I plan to poll the relevant status registers. This will change at some point.
        //
        // This note applies to the behavior at the end of DRV_PIC32CGMAC_LibInitTransfer,
        // as well as the next two steps.

        gmac.periph.ncr.modify(|_r, w| {
            w.txen().set_bit();
            w.rxen().set_bit();
            w.westat().set_bit();
            w
        });

        Ok(gmac)
    }

    pub fn read_frame(&mut self) -> Option<ReadFrame<RX_S>> {
        // Scan through the read frames, and attempt to find one marked as "used"
        compiler_fence(Ordering::SeqCst);
        // self.RX_BUF_DESCS.iter().find_map(|desc| {
        self.gmac_buffers.rx_buf_desc.iter().find_map(|desc| {
            let w0 = desc.get_word_0();
            let addr = w0 & 0xFFFF_FFFC;
            let ready = (w0 & 0x0000_0001) != 0;

            if ready && (addr != 0) {
                // Erase address, but leave 'ready' and potentially 'last' bit set.
                desc.set_word_0(w0 & 0x0000_0003);
                let len = (desc.get_word_1() & 0x0000_0FFF) as usize;

                fence(Ordering::SeqCst);

                let desc_addr = NonNull::new(desc.words.get().cast())?;
                let buf_addr = NonNull::new(addr as *const [u8; RX_S] as *mut [u8; RX_S])?;
                Some(ReadFrame {
                    bufr: buf_addr,
                    len,
                    desc: desc_addr,
                })
            } else {
                None
            }
        })
    }

    fn miim_mgmt_port_enable(&mut self) {
        self.periph.ncr.modify(|_r, w| w.mpe().set_bit());
    }

    fn miim_mgmt_port_disable(&mut self) {
        self.periph.ncr.modify(|_r, w| w.mpe().clear_bit());
    }

    fn miim_is_busy(&mut self) -> bool {
        self.periph.nsr.read().idle().bit_is_clear()
    }

    fn miim_write_data(&mut self, reg_idx: u8, op_data: u16) {
        self.periph.man.write(|w| {
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

    // TODO: Should miim_read be consolidated into a single function?
    fn miim_start_read(&mut self, reg_idx: u8) {
        self.periph.man.write(|w| {
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
        self.periph.man.read().data().bits()
    }

    pub fn miim_post_setup(&mut self) {
        // Starting MIIM setup
        // Enabling management port...
        self.miim_mgmt_port_enable();

        // Waiting for miim idle...
        let val = self.periph.nsr.read().bits();
        // defmt::println!("{=u32:08X}", val);
        while self.miim_is_busy() {}

        // Reset PHY...

        self.miim_write_data(0, 0x8000); // 0.15: Software reset

        // This may just block forever
        while self.miim_is_busy() {}

        self.miim_start_read(0);
        while self.miim_is_busy() {}
        let val = self.miim_read_data_get();
        // defmt::println!("New Reg 0 Val: {=u16:04X}", val);

        // TODO: Skipping Autonegotiation Adv step (reg 4)...
        // TODO: Skipping Autonegotiation restart since we didn't change anything...

        // Wait for link to come up
        loop {
            self.miim_start_read(1);
            while self.miim_is_busy() {}
            let val = self.miim_read_data_get();
            if (val & 0x0004) != 0 {
                // defmt::println!("Link up!");
                break;
            }
        }

        self.miim_mgmt_port_disable();
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
        unsafe { self.words.get().cast::<u32>().add(1).read_volatile() }
    }

    fn set_word_1(&self, val: u32) {
        unsafe { self.words.get().cast::<u32>().add(1).write_volatile(val) }
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
        unsafe { self.words.get().cast::<u32>().add(1).read_volatile() }
    }

    fn set_word_1(&self, val: u32) {
        unsafe { self.words.get().cast::<u32>().add(1).write_volatile(val) }
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

                    // TODO There are more valid permutations of GmacPins
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
struct RxBuffer<const RX_S: usize> {
    buf: UnsafeCell<[u8; RX_S]>,
}

#[repr(C, align(8))]
struct TxBuffer<const TX_S: usize> {
    buf: UnsafeCell<[u8; TX_S]>,
}

unsafe impl<const RX_S: usize> Sync for RxBuffer<RX_S> {}
unsafe impl<const TX_S: usize> Sync for TxBuffer<TX_S> {}
unsafe impl Sync for RxBufferDescriptor {}
unsafe impl Sync for TxBufferDescriptor {}

const RX_BUF_DESC_DEFAULT: RxBufferDescriptor = RxBufferDescriptor {
    words: UnsafeCell::new([0u32; 2]),
};

const TX_BUF_DESC_DEFAULT: TxBufferDescriptor = TxBufferDescriptor {
    words: UnsafeCell::new([0u32; 2]),
};

pub struct GmacBuffers<const TX_N: usize, const TX_S: usize, const RX_N: usize, const RX_S: usize> {
    tx_buf_desc: [TxBufferDescriptor; TX_N],
    tx_buf: [TxBuffer<TX_S>; TX_N],
    rx_buf_desc: [RxBufferDescriptor; RX_N],
    rx_buf: [RxBuffer<RX_S>; RX_N],
    next_tx_idx: usize,
}

impl<const TX_N: usize, const TX_S: usize, const RX_N: usize, const RX_S: usize>
    GmacBuffers<TX_N, TX_S, RX_N, RX_S>
{
    const RX_BUF_DEFAULT: RxBuffer<RX_S> = RxBuffer {
        buf: UnsafeCell::new([0u8; RX_S]),
    };
    const TX_BUF_DEFAULT: TxBuffer<TX_S> = TxBuffer {
        buf: UnsafeCell::new([0u8; TX_S]),
    };

    pub const fn default() -> Self {
        GmacBuffers {
            tx_buf_desc: [TX_BUF_DESC_DEFAULT; TX_N],
            tx_buf: [GmacBuffers::<TX_N, TX_S, RX_N, RX_S>::TX_BUF_DEFAULT; TX_N],
            rx_buf_desc: [RX_BUF_DESC_DEFAULT; RX_N],
            rx_buf: [GmacBuffers::<TX_N, TX_S, RX_N, RX_S>::RX_BUF_DEFAULT; RX_N],
            next_tx_idx: 0,
        }
    }

    pub fn alloc_write_frame(&mut self) -> Option<WriteFrame<TX_S>> {
        let desc = &(self.tx_buf_desc[self.next_tx_idx]);
        let w1 = desc.get_word_1();

        // Is this packet ready to be used by software?
        let ready = (w1 & 0x8000_0000) != 0;
        if !ready {
            return None;
        }
        let cur_idx = self.next_tx_idx;

        self.next_tx_idx = (cur_idx + 1) % TX_N;

        Some(WriteFrame {
            bufr: NonNull::new(self.tx_buf[cur_idx].buf.get().cast())?,
            desc: NonNull::new(self.tx_buf_desc[cur_idx].words.get().cast())?,
            was_sent: false,
        })
    }
}
