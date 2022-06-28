//! Periodically blinks the board's LED2 (located next to the barrel
//! connector) at ~1Hz.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = hal::target_device, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PB8, Output>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Disable the watchdog.
        let wd = hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &wd);
        let bankb = hal::pio::BankB::new(ctx.device.PIOB, &mut pmc, BankConfiguration::default());
        let led = bankb.pb8.into_output();

        (Shared {}, Local { led }, init::Monotonics())
    }

    #[idle(local = [led])]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            ctx.local.led.toggle().unwrap();
            cortex_m::asm::delay(12_000_000);
        }
    }
}
