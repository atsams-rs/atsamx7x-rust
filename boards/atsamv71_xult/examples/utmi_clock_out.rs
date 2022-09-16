//! Example that configures the 480MHz USB clock and outputs it on PA3
//! (on the EXT2 header, see its underside) at 2.4MHz.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = atsamx7x_hal::pac, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::fugit::RateExtU32;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let mainck = clocks.mainck.configure_external_normal(12.MHz()).unwrap();
        let slck = clocks.slck.configure_external_normal();
        let upllck = clocks.upllck.configure(&mainck).unwrap();
        let upllckdiv = clocks.upllckdiv.configure(&upllck, UpllDivider::Div2);
        let _pck2: Pck<Pck2> = clocks.pcks.pck2.configure(&upllckdiv, 100).unwrap(); // @ 2.4MHz
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

        let banka = hal::pio::BankA::new(
            ctx.device.PIOA,
            &mut mck,
            &slck,
            hal::pio::BankConfiguration::default(),
        );
        let _ = banka.pa3.into_peripheral::<hal::pio::C>();

        rprintln!("2.4MHz clock signal fed out onto PA3");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
