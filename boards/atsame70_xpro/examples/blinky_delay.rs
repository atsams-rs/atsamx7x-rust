//! Periodically blinks the board's LED2 (located next to the barrel
//! connector) at ~1Hz using `embedded_hal::blocking::delay`.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::blocking::delay::DelayMs;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::pmc::*;
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
        let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let slck = pmc.get_slck(ctx.device.SUPC, SlowCkSource::ExternalNormal);

        let timer = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_timer();

        let bankb = hal::pio::BankB::new(ctx.device.PIOB, &mut pmc, BankConfiguration::default());
        let led = bankb.pb8.into_output();

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[idle(local = [led, timer])]
    fn idle(ctx: idle::Context) -> ! {
        let idle::LocalResources { led, timer } = ctx.local;

        loop {
            timer.delay_ms(1000);
            led.toggle().unwrap();
        }
    }
}
