//! Measures the frequency of the PWM signal on PA19 via a Timer
//! Counter Channel on PA0 when SW0 is pressed. Bridge these pins.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::PwmPin;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
    use hal::rtt::*;
    use hal::tc::*;
    use rtt_target::{rprintln, rtt_init_print};

    use hal::pwm::{self, Percentage, Pwm};

    #[monotonic(binds = RTT, default = true)]
    type RttMono = Mono<8192>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        counter: Counter<Tc0, Ch0, HostClock, 93750>,
        irq: BankInterrupts<hal::pio::A>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

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

        let mono = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_monotonic();

        let banka = hal::pio::BankA::new(
            ctx.device.PIOA,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );
        let mut button = banka.pa9.into_input(PullDir::PullUp);
        button.set_interrupt(Some(InterruptType::FallingEdge));
        button.set_filter(Some(InputFilter::Debounce));
        let ioa = banka.pa0.into_peripheral();

        let pwml = banka.pa19.into_peripheral();
        let pwm = Pwm::new_pwm0(ctx.device.PWM0, &mut mck);
        let mut ch = pwm.ch0.configure(pwm::ChannelConfiguration {
            freq: 30_000.Hz(),
            duty: Percentage::try_from(0.5).unwrap(),
            invert: true,
        });
        ch.output_on(pwml);
        ch.enable();
        ch.set_freq(1_000.Hz());

        let counter = Tc::new_tc0(ctx.device.TC0, &mut mck)
            .channel_0
            .capture::<{ 12_000_000 / 128 }>(&mck, ioa)
            .unwrap()
            .into_freq_measure(CounterConfiguration {
                sampling: CaptureSamplingRatio::Eight,
                leading_criteria: CaptureMode::Rising,
                trailing_criteria: CaptureMode::Rising,
            });

        (
            Shared {},
            Local {
                irq: banka.interrupts,
                counter,
            },
            init::Monotonics(mono),
        )
    }

    #[task(local = [counter])]
    fn sample_freq(ctx: sample_freq::Context) {
        rprintln!("measuring... ");
        let freq = ctx.local.counter.sample_freq(100.millis());
        rprintln!("measured: {:?}", freq);
        if let Ok(freq) = freq {
            assert_eq!(freq.raw(), 1_000);
        }
    }

    #[task(binds = PIOA, local = [irq])]
    fn pioa(ctx: pioa::Context) {
        for pin in ctx.local.irq.iter() {
            if pin == 9 {
                sample_freq::spawn().unwrap();
            }
        }
    }
}
