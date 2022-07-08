//! UART0 loopback test example: rprints "Hello World" every second.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = atsamx7x_hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::prelude::*;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
    use hal::pmc::*;
    use hal::serial::{uart::*, ExtU32};
    use heapless::String;
    use rtt_target::{rprint, rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        tx: Tx<Uart0>,
        rx: Rx<Uart0>,
        buf: String<32>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        // Get main clock
        let mainck = pmc
            .get_mainck(MainCkSource::ExternalNormal(12.MHz()))
            .unwrap();
        let upllck = pmc.get_upllck(&mainck, ctx.device.UTMI).unwrap();
        let upllckdiv = pmc.get_upllckdiv(&upllck, UpllDivider::Div2);
        let pck: Pck<Pck4> = pmc.get_pck(&upllckdiv, 100 - 1);

        let banka = BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());

        let tx = banka.pa10.into_peripheral();
        let rx = banka.pa9.into_peripheral();
        let mut uart = Uart::new_uart0(
            ctx.device.UART0,
            (tx, rx),
            UartConfiguration::default(115_200.bps()).mode(ChannelMode::LocalLoopback),
            &mut pmc,
            &pck,
        )
        .unwrap();
        uart.listen(Event::RxReady);
        let (tx, rx) = uart.split();

        (
            Shared {},
            Local {
                rx,
                tx,
                buf: String::new(),
            },
            init::Monotonics(),
        )
    }

    #[task(binds=UART0, local = [rx, buf], priority = 2)]
    fn rx(ctx: rx::Context) {
        let rx::LocalResources { rx, buf } = ctx.local;

        let ch = rx.read().unwrap() as char;
        buf.push(ch).unwrap();

        if ch == '\n' {
            rprint!("{}", buf.as_str());
            buf.clear();
        }
    }

    #[task(local= [tx], priority = 1)]
    fn tx(ctx: tx::Context, msg: &'static str) {
        ctx.local.tx.bwrite_all(msg.as_bytes()).unwrap();
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            tx::spawn("Hello world!\n").unwrap();
            cortex_m::asm::delay(12_000_000);
        }
    }
}
