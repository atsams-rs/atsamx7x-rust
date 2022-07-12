//! Periodically blinks the board's LED2 (located next to the barrel
//! connector) at ~1Hz.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::pmc::*;
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
        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let slck = pmc.get_slck(ctx.device.SUPC, SlowCkSource::ExternalBypass);
        let mono = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_monotonic();

        let bankb = hal::pio::BankB::new(ctx.device.PIOB, &mut pmc, BankConfiguration::default());
        let led = bankb.pb8.into_output();

        toggle_led::spawn().unwrap();

        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[task(local = [led])]
    fn toggle_led(ctx: toggle_led::Context) {
        ctx.local.led.toggle().unwrap();
        toggle_led::spawn_after(1.secs()).unwrap();
    }
}
