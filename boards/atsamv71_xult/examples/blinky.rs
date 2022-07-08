//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use dwt_systick_monotonic::{DwtSystick, ExtU32};
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
    use hal::pmc::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<12_000_000>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let mainck = pmc
            .get_mainck(MainCkSource::ExternalNormal(12.MHz()))
            .unwrap();
        let (hclk, _mck) = pmc
            .get_hclk(
                HostClockConfig {
                    pres: MckPrescaler::CLK_1,
                    div: MckDivider::EQ_PCK,
                },
                &mainck,
                &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
            )
            .unwrap();

        let mono = DwtSystick::new(
            &mut ctx.core.DCB,
            ctx.core.DWT,
            ctx.core.SYST,
            hclk.systick_freq().to_Hz(),
        );

        let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let led = banka.pa23.into_output();

        toggle_led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn toggle_led(ctx: toggle_led::Context) {
        ctx.local.led.toggle().unwrap();
        rprintln!("LED0 toggled");
        toggle_led::spawn_after(1.secs()).unwrap();
    }
}
