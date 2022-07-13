//! Periodically blinks the board's LED0 (located below SW0) at ~1Hz
//! using `embedded_hal::blocking::delay`.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use hal::efc::*;
    use hal::ehal::blocking::delay::DelayMs;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::pmc::*;
    use hal::rtt::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<PA23, Output>,
        timer: Timer<8192>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();

        let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let slck = pmc.get_slck(ctx.device.SUPC, SlowCkSource::ExternalNormal);
        let mainck = pmc
            .get_mainck(MainCkSource::ExternalNormal(12.MHz()))
            .unwrap();
        let (_hclk, _mck) = pmc
            .get_hclk(
                HostClockConfig {
                    pres: MckPrescaler::CLK_1,
                    div: MckDivider::EQ_PCK,
                },
                &mainck,
                &mut Efc::new(ctx.device.EFC, VddioLevel::V3),
            )
            .unwrap();

        let timer = Rtt::new_8192Hz(ctx.device.RTT, &slck).into_timer();

        let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let led = banka.pa23.into_output();

        (Shared {}, Local { led, timer }, init::Monotonics())
    }

    #[idle(local = [led, timer])]
    fn idle(ctx: idle::Context) -> ! {
        let idle::LocalResources { led, timer } = ctx.local;

        loop {
            timer.delay_ms(1000);
            led.toggle().unwrap();
            rprintln!("LED0 toggled");
        }
    }
}
