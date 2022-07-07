use crate::pmc::{PeripheralIdentifier, Pmc, UpllClock};
use crate::target_device::{usbhs::RegisterBlock, USBHS};

use core::cell::UnsafeCell;

use cortex_m::interrupt::{self, Mutex};
use rtt_target::{rprint, rprintln};
use usb_device::bus::{PollResult, UsbBusAllocator};
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result as UsbResult, UsbDirection, UsbError};

/// EPConfig tracks the desired configuration for one side of an endpoint.
#[derive(Debug, Clone, Copy, PartialEq)]
struct EPConfig {
    ep_type: EndpointType,
    ep_dir: UsbDirection,
    max_packet_size: u16,
}

impl EPConfig {
    fn new(ep_type: EndpointType, dir: UsbDirection, max_packet_size: u16) -> Self {
        Self {
            ep_type,
            ep_dir: dir,
            max_packet_size,
        }
    }
}

/// Endpoints tracks the desired configuration of all endpoints managed
/// by the USB peripheral.
///
const NR_ENDPOINTS: usize = 10;

#[derive(Debug, PartialEq)]
struct Endpoints {
    ep_config: [Option<EPConfig>; NR_ENDPOINTS],
}

impl Endpoints {
    fn new() -> Self {
        Self {
            ep_config: [None; NR_ENDPOINTS],
        }
    }

    fn find_free_endpoint(&self) -> UsbResult<usize> {
        // start with 1 because 0 is reserved for Control
        for idx in 1..NR_ENDPOINTS {
            if self.ep_config[idx] == None {
                return Ok(idx);
            }
        }
        Err(UsbError::EndpointOverflow)
    }

    #[allow(clippy::too_many_arguments)]
    fn allocate_endpoint(
        &mut self,
        dir: UsbDirection,
        idx: usize,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> UsbResult<EndpointAddress> {
        if idx != 0 && self.ep_config[idx] != None {
            return Err(UsbError::EndpointOverflow);
        }

        self.ep_config[idx] = Some(EPConfig::new(ep_type, dir, max_packet_size));

        Ok(EndpointAddress::from_parts(idx, dir))
    }
}

struct Inner {
    endpoints: Endpoints,
    set_address: bool,
}

pub struct Usb {
    inner: Mutex<UnsafeCell<Inner>>,
}

impl Usb {
    pub fn new_usbhs(_usb: USBHS, pmc: &mut Pmc, _clk: &UpllClock) -> Self {
        pmc.enable_peripherals(&[PeripheralIdentifier::USBHS])
            .unwrap();

        let inner = Inner {
            endpoints: Endpoints::new(),
            set_address: false,
        };

        Self {
            inner: Mutex::new(UnsafeCell::new(inner)),
        }
    }

    pub fn into_usb_allocator(self) -> UsbBusAllocator<Usb> {
        UsbBusAllocator::new(self)
    }
}

// helper functions
impl Inner {
    #[inline(always)]
    fn usbhs(&self) -> &RegisterBlock {
        // Safety: Usbd owns the USBHS
        unsafe { &*USBHS::ptr() }
    }

    #[inline(always)]
    fn enable_endpoint(&self, ep_index: usize) {
        self.usbhs().usbhs_devept.modify(|_, w| match ep_index {
            0 => w.epen0().set_bit(),
            1 => w.epen1().set_bit(),
            2 => w.epen2().set_bit(),
            3 => w.epen3().set_bit(),
            4 => w.epen4().set_bit(),
            5 => w.epen5().set_bit(),
            6 => w.epen6().set_bit(),
            7 => w.epen7().set_bit(),
            8 => w.epen8().set_bit(),
            9 => w.epen9().set_bit(),
            _ => unreachable!(),
        });
    }

    #[inline(always)]
    fn enable_endpoint_interrupt(&self, ep_index: usize) {
        self.usbhs().usbhs_devier.write(|w| match ep_index {
            0 => w.pep_0().set_bit(),
            1 => w.pep_1().set_bit(),
            2 => w.pep_2().set_bit(),
            3 => w.pep_3().set_bit(),
            4 => w.pep_4().set_bit(),
            5 => w.pep_5().set_bit(),
            6 => w.pep_6().set_bit(),
            7 => w.pep_7().set_bit(),
            8 => w.pep_8().set_bit(),
            9 => w.pep_9().set_bit(),
            _ => unreachable!(),
        });
    }

    // compute fifo address
    #[inline(always)]
    fn fifo_addr(&self, ep_index: usize) -> usize {
        const DPRAM: usize = 0xA0100000;
        DPRAM + ep_index * 0x8000
    }

    // write endpoint fifo
    #[inline(always)]
    fn write_fifo(&self, ep_index: usize, buf: &[u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                buf.as_ptr() as *const u8,
                self.fifo_addr(ep_index) as *mut u8,
                buf.len(),
            );
        }
    }

    // read endpoint fifo
    #[inline(always)]
    fn read_fifo(&self, ep_index: usize, buf: &mut [u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.fifo_addr(ep_index) as *const u8,
                buf.as_mut_ptr() as *mut u8,
                buf.len(),
            );
        }
    }

    fn open_endpoint(&self, ep_index: usize) {
        let usbhs = self.usbhs();
        match self.endpoints.ep_config[ep_index] {
            Some(ep_config) => {
                // enable endpoint
                self.enable_endpoint(ep_index);

                rprintln!("reset and enable {:08x?}", usbhs.usbhs_devept.read().bits());

                // generic configuration
                usbhs.usbhs_deveptcfg[ep_index].write(|w| {
                    match ep_config.max_packet_size {
                        0..=8 => w.epsize()._8_byte(),
                        9..=16 => w.epsize()._16_byte(),
                        17..=32 => w.epsize()._32_byte(),
                        33..=64 => w.epsize()._64_byte(),
                        65..=128 => w.epsize()._128_byte(),
                        129..=256 => w.epsize()._256_byte(),
                        257..=512 => w.epsize()._512_byte(),
                        513..=1024 => w.epsize()._1024_byte(),
                        _ => unreachable!(),
                    };
                    // set bank
                    w.epbk()._1_bank();

                    // set end point type
                    w.eptype().bits(ep_config.ep_type as u8);

                    // force allocation
                    w.alloc().set_bit()
                });

                if ep_index == 0 {
                    rprintln!("ep {} - set ctrl", ep_index);
                    // ep0 used as ctrl endpoint
                    // Configure the Endpoint 0 configuration register
                    usbhs.usbhs_deveptcfg[0].modify(|_, w| {
                        // setup RSTDTS
                        usbhs.usbhs_deveptier_ctrl_mode()[ep_index].write(|w| w.rstdts().set_bit());
                        // setup STALLRQC
                        usbhs.usbhs_deveptidr_ctrl_mode()[ep_index]
                            .write(|w| w.stallrqc().set_bit());

                        w
                    });

                    // check that endpoint was correctly initiated
                    // Notice, re-allocation might fail in case size is larger, so be aware
                    if usbhs.usbhs_deveptisr_ctrl_mode()[ep_index]
                        .read()
                        .cfgok()
                        .bit_is_set()
                    {
                        // Endpoint configuration is successful
                        usbhs.usbhs_deveptier_ctrl_mode()[ep_index]
                            .write(|w| w.rxstpes().set_bit());
                        // Enable Endpoint Interrupt
                        self.enable_endpoint_interrupt(ep_index);
                    } else {
                        todo!();
                    };
                } else {
                    rprintln!("ep {} - set type {:?}", ep_index, ep_config.ep_type);
                    usbhs.usbhs_deveptcfg[ep_index].modify(|_, w| {
                        // direction,
                        // 0 (OUT): The endpoint direction is OUT.
                        // 1 (IN): The endpoint direction is IN (nor for control endpoints).
                        w.epdir().bit(ep_config.ep_dir == UsbDirection::In);

                        //     // autosw, Per: do we really need this if not supporting multiple banks
                        w.autosw().set_bit();

                        // set nbtrans
                        //     if ep_config.ep_type == EndpointType::Isochronous {
                        //         w.nbtrans()._1_trans();
                        //         todo!()
                        //     }

                        w
                    });

                    // setup RSTDTS
                    usbhs.usbhs_deveptier_ctrl_mode()[ep_index].write(|w| w.rstdts().set_bit());
                    // setup STALLRQC
                    usbhs.usbhs_deveptidr_ctrl_mode()[ep_index].write(|w| w.stallrqc().set_bit());

                    if usbhs.usbhs_deveptisr_ctrl_mode()[ep_index]
                        .read()
                        .cfgok()
                        .bit_is_set()
                    {
                        // Endpoint configuration is successful
                        usbhs.usbhs_deveptier_ctrl_mode()[ep_index]
                            .write(|w| w.rxstpes().set_bit());
                        // Enable Endpoint Interrupt
                        self.enable_endpoint_interrupt(ep_index);
                    } else {
                        todo!();
                    };
                }
            }
            None => {}
        }
    }

    fn enable(&mut self) {
        // Per: Comments
        // - x5 sets device descriptors in HW
        // - x5 configures end points before attach

        rprintln!("inner:enable");

        let usbhs = self.usbhs();
        // 39.5.2
        usbhs.usbhs_ctrl.write(|w| {
            w.usbe().set_bit(); // enable usb_hs
            w.uimod().set_bit(); // enable device mode
            w.vbushwc().set_bit() // must be set?
        });

        //  wait until clock stable
        while usbhs.usbhs_sr.read().clkusable().bit_is_clear() {}

        // normal mode, both fs and hs available, will autodetect
        //  usbhs.usbhs_devctrl.modify(|_, w| w.spdconf().normal());

        // normal mode, both fs and hs available, will autodetect
        usbhs.usbhs_devctrl.modify(|_, w| w.spdconf().forced_fs());

        // enable interrupts
        usbhs.usbhs_devier.write(|w| {
            w.eorstes().set_bit();
            w.suspes().set_bit();
            w.wakeupes().set_bit();
            w.sofes().set_bit() // should we use this?
        });

        // Manually set the Suspend Interrupt
        usbhs.usbhs_devifr.write(|w| w.susps().set_bit());

        // Clear/Ack interrupts
        usbhs.usbhs_devicr.write(|w| {
            w.eorstc().set_bit();
            w.sofc().set_bit();
            w.wakeupc().set_bit()
        });

        // attach the device
        usbhs.usbhs_devctrl.modify(|_, w| w.detach().clear_bit());

        // We setup the endpoints on reset

        // un-freeze the clock, we want it enabled at all times
        usbhs.usbhs_ctrl.modify(|_, w| w.frzclk().clear_bit());
    }

    // 39.6.2.3 USB Reset
    // The USB bus reset is managed by hardware. It is initiated by a connected host.
    // When a USB reset is detected on the USB line, the following operations are performed by the controller:
    // • All endpoints are disabled, except the default control endpoint.
    // • The default control endpoint is reset (see 39.6.2.4. Endpoint Reset for more details).
    // • The data toggle sequence of the default control endpoint is cleared.
    // • At the end of the reset process, the End of Reset (USBHS_DEVISR.EORST) bit is set.
    // • During a reset, the USBHS automatically switches to High-speed mode if the host is High-speed-capable (the
    //   reset is called High-speed reset). The user should observe the USBHS_SR.SPEED field to know the speed
    //   running at the end of the reset (USBHS_DEVISR.EORST = 1).

    // 39.6.2.4 Endpoint Reset
    // An endpoint can be reset at any time by writing a one to the Endpoint x Reset bit USBHS_DEVEPT.EPRSTx. This
    // is recommended before using an endpoint upon hardware reset or when a USB bus reset has been received. This
    // resets:
    // • The internal state machine of the endpoint.
    // • The receive and transmit bank FIFO counters,
    // • All registers of this endpoint (USBHS_DEVEPTCFGx, USBHS_DEVEPTISRx, the Endpoint x
    //   Control (USBHS_DEVEPTIMRx) register), except its configuration (USBHS_DEVEPTCFGx.ALLOC,
    //   USBHS_DEVEPTCFGx.EPBK, USBHS_DEVEPTCFGx.EPSIZE, USBHS_DEVEPTCFGx.EPDIR,
    //   USBHS_DEVEPTCFGx.EPTYPE) and the Data Toggle Sequence (USBHS_DEVEPTISRx.DTSEQ) field.
    //
    // Note: The interrupt sources located in USBHS_DEVEPTISRx are not cleared when a USB bus reset has been
    // received.
    // The endpoint configuration remains active and the endpoint is still enabled.

    // Called when a USB Reset is detected
    fn reset(&self) {
        rprintln!("inner:reset");
        let usbhs = self.usbhs();
        // assume USB endpoints already configured by `enable`
        // we don't need to reset ep0, done by HW
        usbhs.usbhs_devept.modify(|_, w| {
            // set the reset bit(s)
            w.eprst1().set_bit();
            w.eprst2().set_bit();
            w.eprst3().set_bit();
            w.eprst4().set_bit();
            w.eprst5().set_bit();
            w.eprst6().set_bit();
            w.eprst7().set_bit();
            w.eprst8().set_bit();
            w.eprst9().set_bit()
        });

        usbhs.usbhs_devept.modify(|_, w| {
            // clear the reset bit(s)
            w.eprst1().clear_bit();
            w.eprst2().clear_bit();
            w.eprst3().clear_bit();
            w.eprst4().clear_bit();
            w.eprst5().clear_bit();
            w.eprst6().clear_bit();
            w.eprst7().clear_bit();
            w.eprst8().clear_bit();
            w.eprst9().clear_bit()
        });

        // setup endpoints
        for ep_index in 0..NR_ENDPOINTS {
            self.open_endpoint(ep_index);
        }
    }

    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        rprintln!(
            "inner:alloc_ep dir:{:?}, addr:{:?}, type:{:?}, size:{}",
            dir,
            addr,
            ep_type,
            max_packet_size
        );
        let idx = match addr {
            None => self.endpoints.find_free_endpoint()?,
            Some(addr) => addr.index(),
        };

        let addr =
            self.endpoints
                .allocate_endpoint(dir, idx, ep_type, max_packet_size, interval)?;

        Ok(addr)
    }

    fn set_device_address(&mut self, addr: u8) {
        rprintln!("set_device_address {}", addr);
        let usbhs = self.usbhs();
        usbhs.usbhs_devctrl.modify(|_, w| {
            unsafe { w.uadd().bits(addr) }; // set the address
            w.adden().clear_bit(); // do not enable just yet, done on TXINI for transaction
            w
        });
        self.set_address = true;
    }

    fn poll(&mut self) -> PollResult {
        // rprintln!("inner:poll");

        // Safety: Usbd owns the USBHS
        let usbhs = self.usbhs();
        let _dev_ctrl = usbhs.usbhs_devctrl.read();
        // rprintln!(
        //     "dev_ctrl {:#10x}, uadd {}, adden {}",
        //     dev_ctrl.bits(),
        //     dev_ctrl.uadd().bits(),
        //     dev_ctrl.adden().bit_is_set()
        // );
        // let ctrl = usbhs.usbhs_ctrl.read().bits();
        // rprintln!("ctrl {:x}", ctrl);
        let dev_isr = usbhs.usbhs_devisr.read();
        // rprintln!("dev_irs : {:#010x}", dev_isr.bits());

        if dev_isr.eorst().bit_is_set() {
            // EORST - End of Reset

            rprintln!("eorst");
            let speed = usbhs.usbhs_sr.read().speed();
            rprintln!(
                "speed {:?}",
                match speed.bits() {
                    0 => "full speed",
                    1 => "high speed",
                    2 => "low speed",
                    _ => "reserved",
                }
            );

            // clear the eorst interrupt
            usbhs.usbhs_devicr.write(|w| w.eorstc().set_bit());

            return PollResult::Reset;
        }

        // As the suspend & wakup interrupts/states cannot distinguish between
        // unconnected & unsuspended, we do not handle them to avoid spurious transitions.

        // let mut ep_out = 0;
        // let mut ep_in_complete = 0;
        // let mut ep_setup = 0;

        // for now just care about ep0
        // a bit ugly
        if dev_isr.pep_0().bit_is_set() {
            // rprintln!("pep0");

            let sr = usbhs.usbhs_deveptisr_ctrl_mode()[0].read();

            // setup packet?
            if sr.rxstpi().bit_is_set() {
                // rprintln!("- rxstpi");
                // setup packet received

                return PollResult::Data {
                    ep_out: 0,
                    ep_in_complete: 0,
                    ep_setup: 1, // setup occurred at endpoint 0
                };
            };

            // out packet done
            if sr.rxouti().bit_is_set() {
                // rprintln!("rxouti");
                // could be that we should read the out packet to confirm

                return PollResult::Data {
                    ep_out: 1,
                    ep_in_complete: 0, // assuming the write is the end of the transaction
                    ep_setup: 0,
                };
            };

            // in packet done
            if sr.txini().bit_is_set() {
                // rprintln!("txini");
                // for now assume that this is called only for a SET_ADDRESS
                if self.set_address {
                    rprintln!("--- set addressed");
                    let usbhs = self.usbhs();
                    usbhs.usbhs_devctrl.modify(|_, w| {
                        w.adden().set_bit(); // commit the new address
                        w
                    });
                    self.set_address = false;
                }
                return PollResult::Data {
                    ep_out: 0,
                    ep_in_complete: 1, // assuming the write is the end of the transaction
                    ep_setup: 0,
                };
            };

            panic!("should receive setup packet")
        }

        // ugly should iterate
        if dev_isr.pep_1().bit_is_set() {
            panic!("pep_1");
        }
        if dev_isr.pep_2().bit_is_set() {
            panic!("pep_2");
        }
        if dev_isr.pep_3().bit_is_set() {
            panic!("pep_3");
        }
        if dev_isr.pep_4().bit_is_set() {
            panic!("pep_4");
        }
        if dev_isr.pep_5().bit_is_set() {
            panic!("pep_5");
        }
        if dev_isr.pep_6().bit_is_set() {
            panic!("pep_6");
        }
        if dev_isr.pep_7().bit_is_set() {
            panic!("pep_7");
        }
        if dev_isr.pep_8().bit_is_set() {
            panic!("pep_8");
        }
        if dev_isr.pep_9().bit_is_set() {
            panic!("pep_9");
        }

        PollResult::None
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        // rprintln!("inner:write");
        rprintln!("ep_addr {:?}", ep_addr);
        // rprintln!("buf {:02x?}", buf);
        // rprintln!("buf.len {:?}", buf.len());

        let usbhs = self.usbhs();
        let ep_index = ep_addr.index();
        // rprintln!("ep_index {}", ep_index);
        assert!(buf.len() as u16 <= self.endpoints.ep_config[ep_index].unwrap().max_packet_size);
        self.write_fifo(ep_index, buf);
        if ep_index == 0 {
            // clear TXINI to send the package
            usbhs.usbhs_devepticr_ctrl_mode()[0].write(|w| w.txinic().set_bit());
            // enable TXINI interrupt
            usbhs.usbhs_deveptier_ctrl_mode()[0].write(|w| w.txines().set_bit());
            // enable RXOUTI interrupt
            usbhs.usbhs_deveptier_ctrl_mode()[0].write(|w| w.rxoutes().set_bit());
        } else {
            // panic!("write ep {}", ep_index);
            // Clear the FIFO control send the package.
            usbhs.usbhs_deveptidr_ctrl_mode()[ep_index].write(|w| w.fifoconc().set_bit());

            // clear TXINI to send the package
            // not sure if needed
            usbhs.usbhs_devepticr_ctrl_mode()[ep_index].write(|w| w.txinic().set_bit());
        }

        // assume all fit in one transfer
        Ok(buf.len())
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        // rprintln!("inner:read");
        rprintln!("ep_addr {:?}", ep_addr);
        // rprintln!("buf.len {:?}", buf.len());

        let ep_index = ep_addr.index();

        let usbhs = self.usbhs();

        let sr = usbhs.usbhs_deveptisr_ctrl_mode()[ep_index].read();
        let count = core::cmp::min(sr.byct().bits() as usize, buf.len());

        // rprintln!("--- read count {}", count);

        self.read_fifo(ep_index, &mut buf[0..count]);

        // rprintln!("--- read buf {:x?}", &buf[0..count]);

        if ep_index == 0 {
            // End Point 0

            // Clear RXSTPI interrupt, and make FIFO available
            usbhs.usbhs_devepticr_ctrl_mode()[0].write(|w| w.rxstpic().set_bit());

            // Clear RXOUTI
            usbhs.usbhs_devepticr_ctrl_mode()[0].write(|w| w.rxoutic().set_bit());
        } else {
            // Other Endpoints

            // Clear the FIFO control flag to receive more data.
            usbhs.usbhs_deveptidr_ctrl_mode()[ep_index].write(|w| w.fifoconc().set_bit());

            // Clear RXOUTI
            usbhs.usbhs_devepticr_ctrl_mode()[ep_index].write(|w| w.rxoutic().set_bit());
        }
        Ok(count)
    }

    fn is_stalled(&self, _ep: EndpointAddress) -> bool {
        // stub: seemingly not required
        false
    }

    fn set_stalled(&self, _ep: EndpointAddress, _stalled: bool) {
        // stub: seemingly not required
    }
}

impl usb_device::bus::UsbBus for Usb {
    // ensure the address is set before write of zero sized packed to confirm a SET_ADDRESS transaction
    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = true;

    fn enable(&mut self) {
        rprintln!("==> enable");
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.enable());
        // rprintln!("<== enable");
    }

    fn reset(&self) {
        rprintln!("==> reset");
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.reset());
        // rprintln!("<== reset");
    }

    fn suspend(&self) {
        todo!()
        // disable_interrupts(|cs| self.inner.borrow(cs).borrow().suspend())
    }

    fn resume(&self) {
        todo!()
        // disable_interrupts(|cs| self.inner.borrow(cs).borrow().resume())
    }

    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        rprintln!("==> alloc_ep");
        let res = interrupt::free(|cs| {
            unsafe { &mut *self.inner.borrow(cs).get() }.alloc_ep(
                dir,
                addr,
                ep_type,
                max_packet_size,
                interval,
            )
        });
        rprintln!("<== alloc_ep {:?}", res);
        res
    }

    fn set_device_address(&self, addr: u8) {
        rprintln!("==> set_device_address");
        let res = interrupt::free(|cs| {
            unsafe { &mut *self.inner.borrow(cs).get() }.set_device_address(addr)
        });
        rprintln!("<== set_device_address {:?}", res);
        res
    }

    fn poll(&self) -> PollResult {
        rprintln!("==> poll");
        let res = interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.poll());

        rprint!("<== poll PollResult::");
        match res {
            PollResult::None => rprintln!("None"),
            PollResult::Reset => rprintln!("Reset"),
            PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            } => rprintln!(
                "Data {{ ep_out {}, ep_in_complete {}, ep_setup {}}}",
                ep_out,
                ep_in_complete,
                ep_setup
            ),
            PollResult::Suspend => rprintln!("Suspend"),
            PollResult::Resume => rprintln!("Resume"),
        };
        res
    }

    fn write(&self, ep: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        rprintln!("==> write");
        let res = interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.write(ep, buf));
        rprintln!("<== write {:?}", res);
        res
    }

    fn read(&self, ep: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        rprintln!("==> read banana");
        let res = interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.read(ep, buf));
        rprintln!("<== read {:?}", res);
        res
    }

    fn set_stalled(&self, ep: EndpointAddress, stalled: bool) {
        rprintln!("==> set_stalled");
        let res = interrupt::free(|cs| {
            unsafe { &mut *self.inner.borrow(cs).get() }.set_stalled(ep, stalled)
        });
        rprintln!("<== set_stalled");
        res
    }

    fn is_stalled(&self, ep: EndpointAddress) -> bool {
        rprintln!("==> is_stalled");
        let res = interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.is_stalled(ep));
        rprintln!("<== is_stalled {}", res);
        res
    }

    fn force_reset(&self) -> UsbResult<()> {
        rprintln!("==> force_reset");
        Err(UsbError::Unsupported)
    }
}
