//! Periodically reads the voltage of an AFEC0 channel.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::afec::*;
    use hal::efc::*;
    use hal::ehal::adc::OneShot;
    use hal::pio::*;
    use hal::pmc::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        afec: Afec<Afec0>,
        pin: Pin<PA17, Input>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let mut efc = Efc::new(ctx.device.EFC, VddioLevel::V3);

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let mainck = pmc
            .get_mainck(MainCkSource::InternalRC(MainRcFreq::_12_MHZ))
            .unwrap();
        let (_, mck) = pmc
            .get_hclk(
                HostClockConfig {
                    pres: MckPrescaler::CLK_1,
                    div: MckDivider::EQ_PCK,
                },
                &mainck,
                &mut efc,
            )
            .unwrap();

        let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let afec = Afec::new_afec0(ctx.device.AFEC0, &mut pmc, &mck).unwrap();
        let pin = banka.pa17.into_input(PullDir::PullUp);

        (Shared {}, Local { afec, pin }, init::Monotonics())
    }

    #[idle(local = [afec, pin])]
    fn idle(ctx: idle::Context) -> ! {
        let idle::LocalResources { afec, pin } = ctx.local;
        loop {
            let v: f32 = afec.read(pin).unwrap();
            rprintln!("PA17 (channel 6) = {:.2}V", v);
            cortex_m::asm::delay(12_000_000);
        }
    }
}
