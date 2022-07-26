//! Toggles the board's LED0 (located next to SW0r) on SW0 button
//! press.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [PIOB])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::fugit::RateExtU32;
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
