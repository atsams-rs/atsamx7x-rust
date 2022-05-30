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
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        cortex_m::asm::bkpt();

        // Disable the watchdog.
        hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        let mut efc = {
            use hal::efc::{Efc, VddioLevel};
            Efc::new(ctx.device.EFC, VddioLevel::V3)
        };

        // Configure the clock hierarchy
        {
            use hal::pmc::{
                HostClockConfig, MainCkSource, MckDivider, MckPrescaler, Megahertz, PckId,
                PllaConfig, Pmc,
            };

            let mut pmc = Pmc::new(ctx.device.PMC);
            let mainck = pmc
                .get_mainck(MainCkSource::ExternalBypass(Megahertz::from_raw(12)))
                .unwrap();
            let plla = pmc
                .get_pllack(PllaConfig { div: 1, mult: 8 }, &mainck)
                .unwrap();
            let _hclk = pmc
                .get_hclk(
                    HostClockConfig {
                        pres: MckPrescaler::CLK_1,
                        div: MckDivider::EQ_PCK,
                    },
                    &mainck,
                    &mut efc,
                )
                .unwrap();

            let _pck2 = pmc.get_pck(&plla, 0, PckId::Pck2);
        }

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
