#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub lockbit_word0: LOCKBIT_WORD0,
}
#[doc = "Lock Bits Word 0"]
pub struct LOCKBIT_WORD0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Lock Bits Word 0"]
pub mod lockbit_word0;
