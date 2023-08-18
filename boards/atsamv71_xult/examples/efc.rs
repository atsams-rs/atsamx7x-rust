//! Test that system io pins behave as expected
#![no_std]
#![no_main]

use panic_rtt_target as _;

#[rtic::app(device = hal::pac, peripherals = true, dispatchers = [UART0])]
mod app {
    use atsamx7x_hal as hal;
    use embedded_storage::nor_flash::NorFlash;
    use embedded_storage::nor_flash::ReadNorFlash;
    use hal::efc::*;
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        let efc = ctx.device.EFC;
        rprintln!("0x{:x}", efc.eefc_fsr.read().bits());
        rprintln!("0x{:x}", efc.eefc_fmr.read().bits());
        let mut efc = Efc::new(efc, VddioLevel::V3);
        let wdt = ctx.device.WDT;
        wdt.mr.modify(|_, w| w.wddis().set_bit());
        let mut a: [u8; 4] = [0; 4];
        for i in 0..32768 {
            efc.read(i * 4, &mut a).unwrap();
            let a_int: u32 = u32::from_be_bytes(a);
            rprintln!("Word 0x{:x}: 0x{:x}", i, a_int);
        }

        let array: [u8; 512] = [0xad; 512];
        for _j in 0..10 {
            efc.write(SECTOR_SIZE as u32 * 8, &array).unwrap();
            efc.write(SECTOR_SIZE as u32 * 8 + PAGE_SIZE as u32, &array)
                .unwrap();
            efc.write(SECTOR_SIZE as u32 * 8 + 2 * PAGE_SIZE as u32, &array)
                .unwrap();
            efc.write(SECTOR_SIZE as u32 * 8 + 3 * PAGE_SIZE as u32, &array)
                .unwrap();
            for i in 8..12 {
                efc.read(SECTOR_SIZE as u32 * 8 + i * 4, &mut a).unwrap();
                let a_int: u32 = u32::from_be_bytes(a);
                rprintln!("Word 0x{:x}: 0x{:x}", i, a_int);
            }
            rprintln!("Erasing Flash Sector...");
            efc.erase(SECTOR_SIZE as u32 * 8, SECTOR_SIZE as u32 * 9)
                .unwrap();
            rprintln!("Flash Sector Erased");
            for i in 8..12 {
                efc.read(SECTOR_SIZE as u32 * 8 + i * 4, &mut a).unwrap();
                let a_int: u32 = u32::from_be_bytes(a);
                rprintln!("Word 0x{:x}: 0x{:x}", i, a_int);
            }
        }


        (Shared {}, Local {}, init::Monotonics())
    }
}
