//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz
//! using RTT IRQs.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::ehal::timer::CountDown;
    use hal::pio::*;
    use hal::pmc::*;
    use hal::rtt::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
        timer: Timer<8192>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let slck = pmc.get_slck(ctx.device.SUPC, SlowCkSource::ExternalNormal);

        let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let led = banka.pa23.into_output();

        let mut timer = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_timer();
        timer.start(1.secs());

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[task(binds = RTT, local = [led, timer])]
    fn toggle_led(ctx: toggle_led::Context) {
        let toggle_led::LocalResources { led, timer } = ctx.local;

        led.toggle().unwrap();
        rprintln!("LED0 toggled");
        timer.reset();
    }
}
