//! Toggles the board's LED2 (located next to the barrel connector) on
//! SWD (SW400) button press.
#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [PIOB])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::fugit::RateExtU32;
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
        let clocks = Tokens::new(
            (ctx.device.PMC, ctx.device.SUPC, ctx.device.UTMI),
            &ctx.device.WDT.into(),
        );
        let slck = clocks.slck.configure_external_bypass();
        let mainck = clocks.mainck.configure_external_bypass(12.MHz()).unwrap();
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
        let mut button = banka.pa11.into_input(PullDir::PullUp);
        button.set_interrupt(Some(InterruptType::FallingEdge));
        button.set_filter(Some(InputFilter::Debounce));

        let bankb = hal::pio::BankB::new(
            ctx.device.PIOB,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );
        let led = bankb.pb8.into_output(true);

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
            if pin == 11 {
                ctx.local.led.toggle().unwrap()
            }
        }
    }
}
