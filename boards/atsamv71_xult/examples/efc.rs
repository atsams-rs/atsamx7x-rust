//! Test that system io pins behave as expected
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use core::ptr;
    use hal::efc::*;
    use hal::ehal::digital::v2::ToggleableOutputPin;
    use hal::pio::*;
    use hal::rtt::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        let mut efc = ctx.device.EFC;
        let wdt = ctx.device.WDT;
        wdt.wdt_mr.modify(|_, w| w.wddis().set_bit());
        rprintln!("0x{:x}", efc.eefc_fsr.read().bits());
        rprintln!("0x{:x}", efc.eefc_fmr.read().bits());
        let flash_tokens = Tokens::new(efc);
        let sector0 = flash_tokens.sector0;
        for i in 0..32768 {
            rprintln!("Word 0x{:x}: 0x{:x}", i, sector0.read_word(i).unwrap());
        }
        // let sector1 = flash_tokens.sector1;
        // rprintln!("Reading Flash Sector 1...");
        // for i in 0..32768 {
        //     rprintln!("Word 0x{:x}: 0x{:x}", i, sector1.read_word(i).unwrap());
        // }

        let dead_beef: [usize; 128] = [0xdeadbeef; 128];
        let sector8 = flash_tokens.sector8;
        unsafe { sector8.write_page(0, &dead_beef).unwrap() };
        unsafe { sector8.write_page(1, &dead_beef).unwrap() };
        unsafe { sector8.write_page(2, &dead_beef).unwrap() };
        unsafe { sector8.write_page(3, &dead_beef).unwrap() };
        for i in 8..12 {
            rprintln!("Word 0x{:x}: 0x{:x}", i, sector8.read_word(i).unwrap());
        }
        rprintln!("Erasing Flash Sector...");
        unsafe { sector8.erase_sector().unwrap() };
        rprintln!("Flash Sector Erased");
        for i in 8..12 {
            rprintln!("Word 0x{:x}: 0x{:x}", i, sector8.read_word(i).unwrap());
        }
        unsafe {sector8.write_word(0xbeefbeef,10).unwrap() };
        rprintln!("0x{:x}", sector8.read_word(10).unwrap());

        (Shared {}, Local {}, init::Monotonics())
    }

}
