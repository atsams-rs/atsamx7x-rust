//! Periodically blinks the board's LED2 (located next to the barrel
//! connector) at ~1Hz.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::rtt::*;

    #[monotonic(binds = RTT, default = true)]
    type MyMon = Mono<8192>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PB8, Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let slck = clocks.slck.configure_external_bypass();
        let mainck = clocks.mainck.configure_external_bypass(12.MHz()).unwrap();
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

        let bankb = hal::pio::BankB::new(
            ctx.device.PIOB,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );
        let led = bankb.pb8.into_output(true);

        let mono = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_monotonic();

        toggle_led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn toggle_led(ctx: toggle_led::Context) {
        ctx.local.led.toggle().unwrap();
        toggle_led::spawn_after(1.secs()).unwrap();
    }
}
