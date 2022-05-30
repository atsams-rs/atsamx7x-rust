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
                PllaConfig, Pmc, UpllDivider,
            };

            let mut pmc = Pmc::new(ctx.device.PMC);
            let mainck = pmc
                .get_mainck(MainCkSource::ExternalBypass(Megahertz::from_raw(12)))
                .unwrap();
            let _plla = pmc
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

            let upllck = pmc.get_upllck(&mainck, &mut ctx.device.UTMI).unwrap();
            let upllckdiv = pmc.get_upllckdiv(&upllck, UpllDivider::Div2);
            let _pck2 = pmc.get_pck(&upllckdiv, 100 - 1, PckId::Pck2); // @ 2.4MHz
        }

        // Configure PA03 as PCK2 output
        {
            let pioa = ctx.device.PIOA;

            // Configure pins for function C: UART4 (0b10)
            pioa.pio_abcdsr[1].modify(|_, w| w.p3().set_bit());
            pioa.pio_abcdsr[0].modify(|_, w| w.p3().clear_bit());

            // Give pins to the peripheral.
            pioa.pio_pdr.write(|w| w.p3().set_bit());
            cortex_m::asm::dsb();
            assert!(pioa.pio_psr.read().p3().bit_is_clear());

            // disable multidrive
            pioa.pio_mddr.write(|w| w.p3().set_bit());
            cortex_m::asm::dsb();
            assert!(pioa.pio_mdsr.read().p3().bit_is_clear());

            // ensure we dont pull the pin up/down
            pioa.pio_pudr.write(|w| w.p3().set_bit());
            pioa.pio_ppddr.write(|w| w.p3().set_bit());
            cortex_m::asm::dsb();
            assert!(pioa.pio_pusr.read().p3().bit_is_set());
            assert!(pioa.pio_ppdsr.read().p3().bit_is_set());
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
