//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz,
//! scheduled via a TC channel.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::fugit::{ExtU32, RateExtU32};
    use hal::pio::*;
    use hal::tc::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[monotonic(binds = TC1, default = true)]
    type TcMonotonic = Monotonic<Tc0, Ch1, Channel<Tc0, Ch0, Generate<HostClock, 12_000_000>>, 100>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
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
        let mono = {
            let driver = tc.channel_0.generate::<12_000_000>(&mck).unwrap();
            tc.channel_1
                .generate::<12_000_000>(&mck)
                .unwrap()
                .chain(driver)
                .into_monotonic()
                .unwrap()
        };

        toggle_led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led], capacity = 4)]
    fn toggle_led(ctx: toggle_led::Context) {
        ctx.local.led.toggle().unwrap();
        rprintln!("toggle");
        toggle_led::spawn_at(monotonics::now() + 1.secs()).unwrap();
    }
}
