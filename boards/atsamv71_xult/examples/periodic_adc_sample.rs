//! Periodically reads the voltage of an AFEC0 channel.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use dwt_systick_monotonic::{DwtSystick, ExtU32};
    use hal::afec::*;
    use hal::efc::*;
    use hal::ehal::adc::OneShot;
    use hal::pio::*;
    use hal::pmc::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[monotonic(binds = SysTick, default = true)]
    type Mono = DwtSystick<12_000_000>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        afec: Afec<Afec0>,
        pin: Pin<PA17, Input>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        let mut efc = Efc::new(ctx.device.EFC, VddioLevel::V3);

        let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
        let mainck = pmc
            .get_mainck(MainCkSource::InternalRC(MainRcFreq::_12_MHZ))
            .unwrap();
        let (hclk, mck) = pmc
            .get_hclk(
                HostClockConfig {
                    pres: MckPrescaler::CLK_1,
                    div: MckDivider::EQ_PCK,
                },
                &mainck,
                &mut efc,
            )
            .unwrap();

        let mono = DwtSystick::new(
            &mut ctx.core.DCB,
            ctx.core.DWT,
            ctx.core.SYST,
            hclk.systick_freq().to_Hz(),
        );

        let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let afec = Afec::new_afec0(ctx.device.AFEC0, &mut pmc, &mck).unwrap();
        let pin = banka.pa17.into_input(PullDir::PullUp);

        adc_sample::spawn().unwrap();

        (Shared {}, Local { afec, pin }, init::Monotonics(mono))
    }

    #[task(local = [afec, pin])]
    fn adc_sample(ctx: adc_sample::Context) {
        let adc_sample::LocalResources { afec, pin } = ctx.local;

        let v: f32 = afec.read(pin).unwrap();
        rprintln!("PA17 (channel 6) = {:.2}V", v);

        adc_sample::spawn_after(1.secs()).unwrap();
    }
}
