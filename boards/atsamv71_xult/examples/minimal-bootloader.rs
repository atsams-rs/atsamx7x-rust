#![no_std]
#![no_main]
use cortex_m::asm::bootload;
use atsamx7x_hal as hal;
use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {

    unsafe {
        bootload( 0x00420000 as *const u32);
    }

}
