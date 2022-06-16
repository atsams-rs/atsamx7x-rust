//! Toggles the board's LED0 (located next to SW0r) on SW0 button
//! press.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [PIOB])]
mod app {
    use atsamx7x_hal as hal;
    use hal::ehal::{digital::v2::ToggleableOutputPin, watchdog::WatchdogDisable};
    use hal::pio::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
        irq: BankInterrupts<A>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        // Disable the watchdog.
        hal::watchdog::Watchdog::new(ctx.device.WDT).disable();

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC);
        let banka = BankA::new(
            ctx.device.PIOA,
            &mut pmc,
            BankConfiguration {
                min_debounce_duration: hal::fugit::MillisDurationU32::from_ticks(50).convert(),
            },
        );
        let mut button = banka.pa9.into_input(PullDir::PullUp);
        button.set_interrupt(Some(InterruptType::FallingEdge));
        button.set_filter(Some(InputFilter::Debounce));
        let led = banka.pa23.into_output();

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
                9 => {
                    ctx.local.led.toggle().unwrap();
                    rprintln!("button pressed, LED0 toggled");
                }
                _ => {}
            }
        }
    }
}
