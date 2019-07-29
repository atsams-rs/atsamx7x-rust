#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub lockbit_word0: LOCKBIT_WORD0,
    #[doc = "0x04 - Lock Bits Word 1"]
    pub lockbit_word1: LOCKBIT_WORD1,
    #[doc = "0x08 - Lock Bits Word 2"]
    pub lockbit_word2: LOCKBIT_WORD2,
    #[doc = "0x0c - Lock Bits Word 3"]
    pub lockbit_word3: LOCKBIT_WORD3,
}
#[doc = "Lock Bits Word 0"]
pub struct LOCKBIT_WORD0 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Lock Bits Word 0"]
pub mod lockbit_word0;
#[doc = "Lock Bits Word 1"]
pub struct LOCKBIT_WORD1 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Lock Bits Word 1"]
pub mod lockbit_word1;
#[doc = "Lock Bits Word 2"]
pub struct LOCKBIT_WORD2 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Lock Bits Word 2"]
pub mod lockbit_word2;
#[doc = "Lock Bits Word 3"]
pub struct LOCKBIT_WORD3 {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Lock Bits Word 3"]
pub mod lockbit_word3;
