//! Periodically writes an 8B payload on the TWIHS0 bus, for client
//! with address of 0x0.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::target_device, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::efc::*;
    use hal::ehal::blocking::i2c::Write;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
    use hal::pmc::*;
    use hal::serial::twi::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        twi: Twi<TwiHS0>,
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

        let banka = BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
        let sda = banka.pa3.into_peripheral();
        let sdc = banka.pa4.into_peripheral();
        let twi = Twi::new_twihs0(
            ctx.device.TWIHS0,
            (sdc, sda),
            I2cConfiguration { freq: 1.MHz() },
            &mut pmc,
            &mck,
        )
        .unwrap();

        (Shared {}, Local { twi }, init::Monotonics())
    }

    #[idle(local = [twi])]
    fn idle(ctx: idle::Context) -> ! {
        let idle::LocalResources { twi } = ctx.local;

        let payload = [1, 2, 3, 4, 5, 6, 7, 8];
        loop {
            match twi.write(0x0, &payload) {
                Ok(_) => rprintln!("Wrote {:x?} to client address 0x0 on TWIHS0", payload),
                Err(e) => rprintln!("Failed to write: {:?}", e),
            }
            cortex_m::asm::delay(12_000_000);
        }
    }
}
