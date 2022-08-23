//! Outputs a square wave on a pwm channel
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::PwmPin;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
    use hal::pwm::*;
    use rtt_target::{rprint, rprintln, rtt_init_print};

    #[monotonic(binds = RTT, default = true)]
    type MyMono = hal::rtt::Mono<1>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprint!("init...");

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
            BankConfiguration::default(),
        );

        let pwmh = banka.pa0.into_peripheral();
        let pwml = banka.pa19.into_peripheral();
        let pwm = Pwm::new_pwm0(ctx.device.PWM0, &mut mck);

        let mono = hal::rtt::Rtt::new_1Hz(ctx.device.RTT, &slck).into_monotonic();

        let mut ch = pwm.ch0.configure(ChannelConfiguration {
            freq: 30.kHz(),
            duty: Percentage::try_from(0.1).unwrap(),
            invert: false,
        });
        ch.output_on(pwmh);
        ch.output_on(pwml);
        ch.enable();
        ch.set_duty(Percentage::try_from(0.5).unwrap());
        ch.set_freq(2.kHz());
        rprintln!(" done!");

        (Shared {}, Local {}, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
