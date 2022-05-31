#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = atsamx7x_hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::watchdog::WatchdogDisable;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        cortex_m::asm::bkpt();

        // Disable the watchdog.
        hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
