//! Periodically writes an 8B payload on the TWIHS0 bus, for client
//! with address of 0x0.
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true)]
mod app {
    use atsamx7x_hal as hal;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::ehal::blocking::i2c::Write;
    use hal::fugit::RateExtU32;
    use hal::pio::*;
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

        let sda = banka.pa3.into_peripheral();
        let sdc = banka.pa4.into_peripheral();
        let twi = Twi::new_twihs0(
            ctx.device.TWIHS0,
            (sdc, sda),
            I2cConfiguration { freq: 1.MHz() },
            &mut mck,
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
