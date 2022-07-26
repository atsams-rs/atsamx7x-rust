//!  Example that configures 1 uart to read using interrupts and print continously in the idle function
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = atsamx7x_hal::target_device, peripherals = true, dispatchers = [IXC])]
mod app {
    use atsamx7x_hal as hal;
    use cortex_m::prelude::*;
    use hal::clocks::*;
    use hal::efc::*;
    use hal::fugit::ExtU32 as OtherExtU32;
    use hal::fugit::RateExtU32;
    use hal::nb::block;
    use hal::pio::*;
    use hal::serial::spi::*;
    use hal::serial::ExtU32;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        spi: Spi<Spi0>,
        pcs0: Pin<PB2, PeripheralD>,
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

        // configure pin banks
        let bankb = hal::pio::BankB::new(
            ctx.device.PIOB,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );
        let bankd = hal::pio::BankD::new(
            ctx.device.PIOD,
            &mut mck,
            &slck,
            BankConfiguration::default(),
        );

        // configure pins in to alternate modes
        let miso = bankd.pd20.into_peripheral();
        let pck = bankd.pd22.into_peripheral();
        let mosi = bankd.pd21.into_peripheral();
        let pcs0 = bankb.pb2.into_peripheral();

        // Create a new spi, this always starts cs at index 0.
        let mut spi = Spi::new_spi0(
            ctx.device.SPI0,
            (pck, mosi, miso),
            SpiConfiguration::default().test_mode(true),
            &mut mck,
        )
        .unwrap();

        spi.setup_client(
            &pcs0,
            ClientConfiguration::default(115_200.bps(), hal::ehal::spi::MODE_0)
                .delay_before_clock_edge(830.nanos()),
            &mck,
        )
        .unwrap();

        spi::spawn().unwrap();

        (Shared {}, Local { spi, pcs0 }, init::Monotonics())
    }

    #[task(local = [spi, pcs0])]
    fn spi(ctx: spi::Context) {
        let spi::LocalResources { spi, pcs0 } = ctx.local;

        let mut client = spi.select(pcs0).unwrap();
        for b in 0..=u8::MAX {
            block!(client.send(b)).unwrap();
            let r = block!(client.read()).unwrap();
            rprintln!("Sent 0x{:x}, read back 0x{:x}", b, r);
        }
    }
}
