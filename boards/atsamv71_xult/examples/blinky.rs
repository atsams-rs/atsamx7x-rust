//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz.
#![no_std]
#![no_main]

use panic_rtt_target as _;
// use panic_halt as _;
#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::{Efc, VddioLevel};
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::rtt::*;
    use rtt_target::{rprintln, rtt_init_print};

    // #[monotonic(binds = RTT, default = true)]
    // type MyMono = Mono<8192>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Init");

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
        let led = banka.pa23.into_output(true);

        // let mono = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_monotonic();

        // toggle_led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics())
    }

    #[idle(local = [led])]
    fn idle(ctx: idle::Context) -> ! {
        rprintln!("Idle");
        ctx.local.led.toggle().unwrap();
        loop {
            rprintln!("Loop");
            ctx.local.led.toggle().unwrap();
            cortex_m::asm::delay(2000000);
        }
    }
}
