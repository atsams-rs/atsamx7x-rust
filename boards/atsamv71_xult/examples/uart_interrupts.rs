//! UART0 loopback test example: rprints "Hello World" every second.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = atsamx7x_hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::prelude::*;
    use hal::event_system::*;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
    use hal::serial::{uart::*, ExtU32};
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        uart: Uart<Uart0>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let slck = clocks.slck.configure_external_normal();
        let mainck = clocks.mainck.configure_external_normal(12.MHz()).unwrap();
        let pck: Pck<Pck4> = clocks.pcks.pck4.configure(&mainck, 1);
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

        let banka = hal::pio::BankA::new(
            ctx.device.PIOA,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );

        let tx = banka.pa10.into_peripheral();
        let rx = banka.pa9.into_peripheral();
        let mut uart = Uart::new_uart0(
            ctx.device.UART0,
            (tx, rx),
            UartConfiguration::default(115_200.bps()).mode(ChannelMode::LocalLoopback),
            PeripheralClock::Other(&mut mck, &pck),
        )
        .unwrap();
        uart.listen_slice(&[Event::RxReady, Event::TxReady]);
        (Shared {}, Local { uart }, init::Monotonics())
    }

    #[task(binds=UART0, local = [uart], priority = 2)]
    fn uart(ctx: uart::Context) {
        use hal::serial::uart::Event::*;
        let uart::LocalResources { uart } = ctx.local;
        for event in uart.events() {
            match event {
                RxReady => {
                    rprintln!("{}", uart.read().unwrap() as char);
                }
                TxReady | TxEmpty => {
                    uart.write(48).unwrap();
                }
            }
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
