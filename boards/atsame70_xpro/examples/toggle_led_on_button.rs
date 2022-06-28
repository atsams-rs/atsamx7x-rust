//! Toggles the board's LED2 (located next to the barrel connector) on
//! SWD (SW400) button press.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [PIOB])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PB8, Output>,
        irq: BankInterrupts<A>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        // Disable the watchdog.
        let wd = hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &wd);
        let banka = BankA::new(
            ctx.device.PIOA,
            &mut pmc,
            BankConfiguration {
                min_debounce_duration: hal::fugit::MillisDurationU32::from_ticks(50).convert(),
            },
        );
        let mut button = banka.pa11.into_input(PullDir::PullUp);
        button.set_interrupt(Some(InterruptType::FallingEdge));
        button.set_filter(Some(InputFilter::Debounce));

        let bankb = BankB::new(ctx.device.PIOB, &mut pmc, BankConfiguration::default());
        let led = bankb.pb8.into_output();

        (
            Shared {},
            Local {
                led,
                irq: banka.interrupts,
            },
            init::Monotonics(),
        )
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }

    #[task(binds = PIOA, local = [irq, led])]
    fn pioa(ctx: pioa::Context) {
        for pin in ctx.local.irq.iter() {
            match pin {
                11 => ctx.local.led.toggle().unwrap(),
                _ => {}
            }
        }
    }
}
