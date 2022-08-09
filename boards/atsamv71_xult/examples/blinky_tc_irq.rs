//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz
//! using TC Timer IRQs.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::ehal::timer::CountDown;
    use hal::pio::*;
    use hal::tc::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
        timer: Timer<Tc0, Ch1, Channel<Tc0, Ch0, Generate<HostClock, 12_000_000>>, 100>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

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

        let banka = hal::pio::BankA::new(
            ctx.device.PIOA,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );
        let led = banka.pa23.into_output(false);

        let tc = Tc::new_tc0(ctx.device.TC0, &mut mck);
        let mut timer = {
            let driver = tc.channel_0.generate::<12_000_000>(&mck).unwrap();
            tc.channel_1
                .generate::<12_000_000>(&mck)
                .unwrap()
                .chain(driver)
                .into_timer::<100>()
                .unwrap()
        };
        timer.start(1.secs());

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[task(binds = TC1, local = [led, timer])]
    fn toggle_led(ctx: toggle_led::Context) {
        let toggle_led::LocalResources { led, timer } = ctx.local;

        led.toggle().unwrap();
        rprintln!("LED0 toggled");
        timer.start(1.secs());
    }
}
