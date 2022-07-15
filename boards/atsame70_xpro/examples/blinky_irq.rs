//! Periodically blinks the board's LED2 (located next to the barrel
//! connector) at ~1Hz using RTT IRQs.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::ehal::timer::CountDown;
    use hal::pio::*;
    use hal::rtt::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PB8, Output>,
        timer: Timer<8192>,
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
        let led = bankb.pb8.into_output();

        let mut timer = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_timer();
        timer.start(1.secs());

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[task(binds = RTT, local = [led, timer])]
    fn toggle_led(ctx: toggle_led::Context) {
        let toggle_led::LocalResources { led, timer } = ctx.local;

        led.toggle().unwrap();
        timer.reset();
    }
}
