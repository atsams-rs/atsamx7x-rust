//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // Disable the watchdog.
        let wd = hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &wd);
        let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let led = banka.pa23.into_output();

        (Shared {}, Local { led }, init::Monotonics())
    }

    #[idle(local = [led])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            ctx.local.led.toggle().unwrap();
            rprintln!("LED0 toggled");
            cortex_m::asm::delay(12_000_000);
        }
    }
}
