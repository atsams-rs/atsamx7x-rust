//! Example that configures the 480MHz USB clock and outputs it on PA3
//! (on the EXT2 header, see its underside) at 2.4MHz.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = atsamx7x_hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::watchdog::WatchdogDisable;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Disable the watchdog.
        hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        // Configure the clock hierarchy
        let mut pmc = {
            use hal::pmc::{MainCkSource, Megahertz, PckId, Pmc, UpllDivider};

            let mut pmc = Pmc::new(ctx.device.PMC);
            let mainck = pmc
                .get_mainck(MainCkSource::ExternalBypass(Megahertz::from_raw(12)))
                .unwrap();
            let upllck = pmc.get_upllck(&mainck, &mut ctx.device.UTMI).unwrap();
            let upllckdiv = pmc.get_upllckdiv(&upllck, UpllDivider::Div2);
            let _pck2 = pmc.get_pck(&upllckdiv, 100 - 1, PckId::Pck2); // @ 2.4MHz

            pmc
        };

        let banka = hal::pio::BankA::new(
            ctx.device.PIOA,
            &mut pmc,
            hal::pio::BankConfiguration::default(),
        );
        let _ = banka.pa3.into_peripheral::<hal::pio::C>();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
