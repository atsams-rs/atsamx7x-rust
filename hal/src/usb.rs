/*!
Universal Serial Bus (USB)
---

This module implements [`usb_device`] for the device's [`USBHS`].
Currently, the implementation is limited to Full-Speed even though
High-Speed is supported in hardware. The implementation has not been
extensively tested, and should be considered unstable at the moment.

# Example usage

```
let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
let mainck = pmc
    .get_mainck(MainCkSource::ExternalNormal(12.MHz()))
    .unwrap();
let upllck = pmc.get_upllck(&mainck, ctx.device.UTMI).unwrap();
let upllckdiv = pmc.get_upllckdiv(&upllck, UpllDivider::Div2);
pmc.get_hclk(
    HostClockConfig {
        pres: MckPrescaler::CLK_2,
        div: MckDivider::EQ_PCK,
    },
    &upllckdiv,
    &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
)
.unwrap();

let usb_alloc = unsafe {
    USB_ALLOCATOR = Some(
        hal::usb::Usb::new(ctx.device.USBHS, &mut pmc, &upllck).into_usb_allocator(),
    );
    USB_ALLOCATOR.as_ref().unwrap()
};

let serial = SerialPort::new(&usb_alloc);
let usb_dev = UsbDeviceBuilder::new(&usb_alloc, UsbVidPid(0x16c0, 0x27dd))
    .manufacturer("Fake company")
    .product("Serial port")
    .serial_number("TEST")
    .device_class(USB_CLASS_CDC)
    .max_packet_size_0(64) // makes control transfers 8x faster
    .build();

let mut buf = [0u8; 64];

loop {
    if !usb_dev.poll(&mut [serial]) {
        continue;
    }

    let count = match serial.read(&mut buf) {
        Ok(count) => count,
        Err(UsbError::WouldBlock) => {
            // No data received
            continue;
        }
        Err(e) => {
            // read error
            continue;
        }
    };

    match serial.write(&buf[..count]) {
        Ok(count) => {
            // echoed back count bytes
        }
        Err(e) => {
            // write error
        }
    }
}
```
 */

use crate::pmc::{PeripheralIdentifier, Pmc, UpllClock};
use crate::target_device::{usbhs::RegisterBlock, USBHS};

use core::cell::UnsafeCell;

use cortex_m::interrupt::{self, Mutex};
use usb_device::bus::{PollResult, UsbBusAllocator};
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result as UsbResult, UsbDirection, UsbError};

use bit_iter::BitIter;
pub use usb_device;

const NUM_ENDPOINTS: usize = 10;

/// Endpoint configuration.
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

#[derive(Debug, PartialEq)]
struct Endpoints {
    ep_config: [Option<EPConfig>; NUM_ENDPOINTS],
}

impl Endpoints {
    fn new() -> Self {
        Self {
            ep_config: [None; NUM_ENDPOINTS],
        }
    }

    fn find_free_endpoint(&self) -> UsbResult<usize> {
        // start with 1 because 0 is reserved for Control
        for idx in 1..NUM_ENDPOINTS {
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

/// [`usb_device`] implementation.
pub struct Usb {
    inner: Mutex<UnsafeCell<Inner>>,
}

impl Usb {
    /// Create a new [`Usb`] from the device's [`USBHS`] and
    /// configured clock hierarchy.
    pub fn new(_usb: USBHS, pmc: &mut Pmc, _clk: &UpllClock) -> Self {
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

    /// Consume the [`Usb`] and create a [`UsbBusAllocator`] that
    /// implements [`usb_device`].
    pub fn into_usb_allocator(self) -> UsbBusAllocator<Usb> {
        UsbBusAllocator::new(self)
    }
}

impl Inner {
    #[inline(always)]
    fn reg(&self) -> &RegisterBlock {
        // Safety: Usb owns the USBHS
        unsafe { &*USBHS::ptr() }
    }

    #[inline(always)]
    fn enable_endpoint(&self, ep: usize) {
        self.reg().usbhs_devept.modify(|_, w| match ep {
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
    fn enable_endpoint_interrupt(&self, ep: usize) {
        self.reg().usbhs_devier.write(|w| match ep {
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

    /// Computes the FIFO address
    #[inline(always)]
    fn fifo_addr(&self, ep: usize) -> usize {
        // Also known as USBHS RAM; c.f. §10
        const DPRAM: usize = 0xA0100000;

        DPRAM + ep * 0x8000
    }

    // write endpoint fifo
    #[inline(always)]
    fn write_fifo(&self, ep: usize, buf: &[u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                buf.as_ptr() as *const u8,
                self.fifo_addr(ep) as *mut u8,
                buf.len(),
            );
        }
    }

    // read endpoint fifo
    #[inline(always)]
    fn read_fifo(&self, ep: usize, buf: &mut [u8]) {
        unsafe {
            core::ptr::copy_nonoverlapping(
                self.fifo_addr(ep) as *const u8,
                buf.as_mut_ptr() as *mut u8,
                buf.len(),
            );
        }
    }

    fn open_endpoint(&self, ep: usize) {
        if let Some(conf) = self.endpoints.ep_config[ep] {
            // Enable the endpoint and apply common endpoint
            // configuration.
            self.enable_endpoint(ep);
            self.reg().usbhs_deveptcfg[ep].write(|w| {
                match conf.max_packet_size {
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

                // single-bank endpoint
                w.epbk()._1_bank();

                w.eptype().bits(conf.ep_type as u8);
                w.alloc().set_bit();

                w
            });

            // setup RSTDTS, STALLRQC
            self.reg().usbhs_deveptier_ctrl_mode()[ep].write(|w| w.rstdts().set_bit());
            self.reg().usbhs_deveptidr_ctrl_mode()[ep].write(|w| w.stallrqc().set_bit());

            if ep != 0 {
                // Configure endpoint direction.
                self.reg().usbhs_deveptcfg[ep]
                    .modify(|_, w| w.epdir().bit(conf.ep_dir == UsbDirection::In));
            }

            // Check if endpoint configuration was successful
            if self.reg().usbhs_deveptisr_ctrl_mode()[ep]
                .read()
                .cfgok()
                .bit_is_set()
            {
                self.reg().usbhs_deveptier_ctrl_mode()[ep].write(|w| w.rxstpes().set_bit());
                self.enable_endpoint_interrupt(ep);
            } else {
                todo!("endpoint configuration failed");
            }
        }
    }

    fn enable(&mut self) {
        // enable the peripheral and enter device mode
        self.reg().usbhs_ctrl.write(|w| {
            w.usbe().set_bit();
            w.uimod().set_bit();

            // Must be set; C.f. §39.7.1
            w.vbushwc().set_bit();
            // Must be cleared; C.f. ditto
            w.uid().clear_bit();

            w
        });

        // wait until clock stable
        while self.reg().usbhs_sr.read().clkusable().bit_is_clear() {}

        // Force full-speed: automatic reset switch to high-speed
        // yields protocol errors.
        self.reg()
            .usbhs_devctrl
            .modify(|_, w| w.spdconf().forced_fs());

        // enable interrupts
        self.reg().usbhs_devier.write(|w| {
            w.eorstes().set_bit();
            w.suspes().set_bit();
            w.wakeupes().set_bit();

            // should we use this?
            w.sofes().set_bit();

            w
        });

        // Clear any transient interrupts
        self.reg().usbhs_devicr.write(|w| {
            w.eorstc().set_bit();
            w.sofc().set_bit();
            w.wakeupc().set_bit()
        });

        // attach the device
        self.reg()
            .usbhs_devctrl
            .modify(|_, w| w.detach().clear_bit());

        // un-freeze the clock
        self.reg().usbhs_ctrl.modify(|_, w| w.frzclk().clear_bit());
    }

    /// Resets: the internal state machines of the endpoints,
    /// receive/transmit bank FIFO counters, and endpoint
    /// configuration registers.
    fn reset(&self) {
        // Assume USB endpoints already configured by `enable`.
        // Further, we don't need to reset ep0; this is done by
        // hardware.

        // First, initiate the reset sequence by setting all bits, and
        // finalize the sequence by clearing all bits. C.f. §39.7.12.
        self.reg().usbhs_devept.modify(|_, w| {
            w.eprst1().set_bit();
            w.eprst2().set_bit();
            w.eprst3().set_bit();
            w.eprst4().set_bit();
            w.eprst5().set_bit();
            w.eprst6().set_bit();
            w.eprst7().set_bit();
            w.eprst8().set_bit();
            w.eprst9().set_bit();
            w
        });
        self.reg().usbhs_devept.modify(|_, w| {
            w.eprst1().clear_bit();
            w.eprst2().clear_bit();
            w.eprst3().clear_bit();
            w.eprst4().clear_bit();
            w.eprst5().clear_bit();
            w.eprst6().clear_bit();
            w.eprst7().clear_bit();
            w.eprst8().clear_bit();
            w.eprst9().clear_bit();
            w
        });

        // setup endpoints
        for ep in 0..NUM_ENDPOINTS {
            self.open_endpoint(ep);
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
        // Set the address in hardware, but do not enable it. This is
        // done in poll() on an TXINI.
        self.reg().usbhs_devctrl.modify(|_, w| {
            unsafe {
                w.uadd().bits(addr);
            }
            w.adden().clear_bit();
            w
        });
        self.set_address = true;
    }

    fn poll(&mut self) -> PollResult {
        let dev_isr = self.reg().usbhs_devisr.read();

        if dev_isr.eorst().bit_is_set() {
            // EORST - End of Reset, ack the interrupt and notify
            // stack.
            self.reg().usbhs_devicr.write(|w| w.eorstc().set_bit());
            return PollResult::Reset;
        }

        let mut ep_out = 0;
        let mut ep_in_complete = 0;
        let mut ep_setup = 0;

        // C.f. §39.7.6
        const DEVISR_PEPS_MASK: u32 = 0x3ff000;
        const DEVISR_PEPS_OFFSET: u8 = 12;
        for ep in BitIter::from((dev_isr.bits() & DEVISR_PEPS_MASK) >> DEVISR_PEPS_OFFSET) {
            let sr = self.reg().usbhs_deveptisr_ctrl_mode()[ep as usize].read();

            // SETUP packet?
            if sr.rxstpi().bit_is_set() {
                ep_setup |= 1 << ep;
            };

            // OUT packet?
            if sr.rxouti().bit_is_set() {
                ep_out |= 1 << ep;
            };

            // IN packet?
            if sr.txini().bit_is_set() {
                if self.set_address {
                    // commit the new address
                    self.reg().usbhs_devctrl.modify(|_, w| w.adden().set_bit());
                    self.set_address = false;
                }

                ep_in_complete |= 1 << ep;
            };
        }

        if ep_out == 0 && ep_in_complete == 0 && ep_setup == 0 {
            return PollResult::None;
        } else {
            return PollResult::Data {
                ep_out,
                ep_in_complete,
                ep_setup,
            };
        }
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        let ep = ep_addr.index();
        assert!(buf.len() as u16 <= self.endpoints.ep_config[ep].unwrap().max_packet_size);

        self.write_fifo(ep, buf);

        if ep == 0 {
            // clear TXINI to send the package
            self.reg().usbhs_devepticr_ctrl_mode()[0].write(|w| w.txinic().set_bit());
            // enable TXINI interrupt
            self.reg().usbhs_deveptier_ctrl_mode()[0].write(|w| w.txines().set_bit());
            // enable RXOUTI interrupt
            self.reg().usbhs_deveptier_ctrl_mode()[0].write(|w| w.rxoutes().set_bit());
        } else {
            // Clear the FIFO control send the package.
            self.reg().usbhs_deveptidr_ctrl_mode()[ep].write(|w| w.fifoconc().set_bit());

            // clear TXINI to send the package
            // XXX required?
            self.reg().usbhs_devepticr_ctrl_mode()[ep].write(|w| w.txinic().set_bit());
        }

        // assume all fit in one transfer
        Ok(buf.len())
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        let ep = ep_addr.index();
        let len = core::cmp::min(
            self.reg().usbhs_deveptisr_ctrl_mode()[ep]
                .read()
                .byct()
                .bits() as usize,
            buf.len(),
        );

        self.read_fifo(ep, &mut buf[0..len]);

        if ep == 0 {
            // control endpoints

            // Clear RXSTPI interrupt, and make FIFO available
            self.reg().usbhs_devepticr_ctrl_mode()[0].write(|w| w.rxstpic().set_bit());

            // Clear RXOUTI
            self.reg().usbhs_devepticr_ctrl_mode()[0].write(|w| w.rxoutic().set_bit());
        } else {
            // Other Endpoints

            // Clear the FIFO control flag to receive more data.
            self.reg().usbhs_deveptidr_ctrl_mode()[ep].write(|w| w.fifoconc().set_bit());

            // Clear RXOUTI
            self.reg().usbhs_devepticr_ctrl_mode()[ep].write(|w| w.rxoutic().set_bit());
        }

        Ok(len)
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
    /// Ensure the address is set before write of zero sized packet to
    /// confirm a SET_ADDRESS transaction.
    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = true;

    fn enable(&mut self) {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.enable());
    }

    fn reset(&self) {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.reset());
    }

    fn suspend(&self) {
        // stub: not required
    }

    fn resume(&self) {
        // stub: not required
    }

    fn alloc_ep(
        &mut self,
        dir: UsbDirection,
        addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        interrupt::free(|cs| {
            unsafe { &mut *self.inner.borrow(cs).get() }.alloc_ep(
                dir,
                addr,
                ep_type,
                max_packet_size,
                interval,
            )
        })
    }

    fn set_device_address(&self, addr: u8) {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.set_device_address(addr))
    }

    fn poll(&self) -> PollResult {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.poll())
    }

    fn write(&self, ep: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.write(ep, buf))
    }

    fn read(&self, ep: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.read(ep, buf))
    }

    fn set_stalled(&self, ep: EndpointAddress, stalled: bool) {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.set_stalled(ep, stalled))
    }

    fn is_stalled(&self, ep: EndpointAddress) -> bool {
        interrupt::free(|cs| unsafe { &mut *self.inner.borrow(cs).get() }.is_stalled(ep))
    }

    fn force_reset(&self) -> UsbResult<()> {
        Err(UsbError::Unsupported)
    }
}
