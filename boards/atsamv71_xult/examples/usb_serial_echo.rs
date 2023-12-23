//! Exposes a serial device over the USB interface that echoes back
//! bytes written to it.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::fugit::RateExtU32;
    use hal::usb::usb_device::{bus::UsbBusAllocator, prelude::*};
    use rtt_target::{rprint, rprintln, rtt_init_print};
    use usbd_serial::{SerialPort, USB_CLASS_CDC};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        usb_dev: UsbDevice<'static, hal::usb::Usb>,
        serial: SerialPort<'static, hal::usb::Usb>,
        buf: [u8; 64],
    }

    #[init(local = [usb_alloc: Option<UsbBusAllocator<hal::usb::Usb>> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprint!("init...");

        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let mainck = clocks.mainck.configure_external_normal(12.MHz()).unwrap();
        let upllck = clocks.upllck.configure(&mainck).unwrap();
        let upllckdiv = clocks.upllckdiv.configure(&upllck, UpllDivider::Div2);
        let (_hclk, mut mck) = HostClockController::new(clocks.hclk, clocks.mck)
            .configure(
                &upllckdiv,
                &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
                HostClockConfig {
                    pres: HccPrescaler::Div2,
                    div: MckDivider::Div1,
                },
            )
            .unwrap();

        *ctx.local.usb_alloc =
            Some(hal::usb::Usb::new(ctx.device.USBHS, &mut mck, &upllck).into_usb_allocator());
        let serial = SerialPort::new(ctx.local.usb_alloc.as_ref().unwrap());
        let usb_dev = UsbDeviceBuilder::new(
            ctx.local.usb_alloc.as_ref().unwrap(),
            UsbVidPid(0xdead, 0xbeef),
        )
        .strings(&[StringDescriptors::new(LangID::EN)
            .manufacturer("ATSAMx7x HAL Contributors")
            .product("Serial port echo")
            .serial_number("N/A")])
        .unwrap()
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // makes control transfers 8x faster
        .unwrap()
        .build();

        rprintln!(" done");

        (
            Shared {},
            Local {
                serial,
                usb_dev,
                buf: [0; 64],
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBHS, local = [serial, usb_dev, buf])]
    fn usb(ctx: usb::Context) {
        let usb::LocalResources {
            serial,
            usb_dev,
            buf,
        } = ctx.local;

        if !usb_dev.poll(&mut [serial]) {
            return;
        }

        let count = match serial.read(buf) {
            Ok(count) => count,
            Err(UsbError::WouldBlock) => {
                // No data received
                return;
            }
            Err(e) => {
                rprintln!("USB read error: {:?}", e);
                return;
            }
        };

        let echo = &buf[..count];

        match serial.write(echo) {
            Ok(count) => {
                rprintln!("Echoed back {:x?} ({} bytes)", echo, count);
            }
            Err(e) => {
                rprintln!("USB write error: {:?}", e);
            }
        }
    }
}
