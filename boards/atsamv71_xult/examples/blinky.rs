//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::pmc::*;
    use hal::rtt::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[monotonic(binds = RTT, default = true)]
    type MyMono = Mono<8192>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let slck = pmc.get_slck(ctx.device.SUPC, SlowCkSource::ExternalNormal);
        let mono = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_monotonic();

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
