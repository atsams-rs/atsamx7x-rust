//! Switches USART0 between UART and SPI mode, transmitting a one-byte
//! payload. Bridge PB00 and PB01.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = atsamx7x_hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::prelude::*;
    use hal::fugit::RateExtU32;
    use hal::generics::events::EventHandler;
    use hal::pio::*;
    use hal::serial::{usart::*, ExtU32};
    use rtt_target::{rprint, rprintln, rtt_init_print};

    const PAYLOAD: u8 = b'x';

    #[shared]
    struct Shared {
        usart: &'static mut Usart<Usart0>,
    }

    #[local]
    struct Local {
        uart: Uart<Usart0>,
        spi: Spi<Usart0, Host>,
    }

    #[init(local = [usart: Option<Usart<Usart0>> = None])]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let slck = clocks.slck.configure_external_normal();
        let mainck = clocks.mainck.configure_external_normal(12.MHz()).unwrap();
        let (_hclk, mut mck) = HostClockController::new(clocks.hclk, clocks.mck)
            .configure(
                &mainck,
                &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
                HostClockConfig {
                    pres: HccPrescaler::Div1,
                    div: MckDivider::Div1,
                },
            )
            .unwrap();

        let bankb = BankB::new(
            ctx.device.PIOB,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );

        let miso = bankb.pb0.into_peripheral();
        let mosi = bankb.pb1.into_peripheral();
        let clk = bankb.pb13.into_peripheral();
        let nss = bankb.pb3.into_peripheral();

        let (handles, mut usart) =
            Usart::new_usart0(ctx.device.USART0, (mosi, miso, clk, nss), &mut mck);
        let spi = handles
            .spi_host
            .configure(
                &usart,
                SpiConfig {
                    bitrate: 115_200.bps(),
                    mode: MODE_0,
                },
            )
            .unwrap();
        let mut uart = handles
            .uart
            .configure(&usart, UartConfiguration::default(115_200.bps()))
            .unwrap();

        usart.listen(Event::RxReady);

        usart.enter_mode(&uart).unwrap();
        uart.write(PAYLOAD).unwrap();

        *ctx.local.usart = Some(usart);
        let usart = ctx.local.usart.as_mut().unwrap();

        (Shared { usart }, Local { uart, spi }, init::Monotonics())
    }

    #[task(binds = USART0, shared = [usart], local = [uart, spi])]
    fn usart(mut ctx: usart::Context) {
        let usart::LocalResources { uart, spi } = ctx.local;

        ctx.shared.usart.lock(|usart| {
            let mode = usart.mode();
            rprint!("USART test: {:?}...", mode);

            match mode {
                UsartMode::Uart => {
                    assert_eq!(uart.read(), Ok(PAYLOAD));
                    usart.enter_mode(spi).unwrap();
                    spi.send(PAYLOAD).unwrap();
                }
                UsartMode::Spi(SpiMode::Host) => {
                    assert_eq!(spi.read(), Ok(PAYLOAD));
                    usart.enter_mode(uart).unwrap();
                    uart.write(PAYLOAD).unwrap();
                }
                _ => unreachable!(),
            }

            rprintln!("success!");
        });
    }
}
