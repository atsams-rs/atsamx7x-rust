#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub lockbit_word0: crate::Reg<lockbit_word0::LOCKBIT_WORD0_SPEC>,
}
#[doc = "LOCKBIT_WORD0 register accessor: an alias for `Reg<LOCKBIT_WORD0_SPEC>`"]
pub type LOCKBIT_WORD0 = crate::Reg<lockbit_word0::LOCKBIT_WORD0_SPEC>;
#[doc = "Lock Bits Word 0"]
pub mod lockbit_word0;
