#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub word0: crate::Reg<word0::WORD0_SPEC>,
    #[doc = "0x04 - Lock Bits Word 1"]
    pub word1: crate::Reg<word1::WORD1_SPEC>,
}
#[doc = "WORD0 register accessor: an alias for `Reg<WORD0_SPEC>`"]
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
#[doc = "Lock Bits Word 0"]
pub mod word0;
#[doc = "WORD1 register accessor: an alias for `Reg<WORD1_SPEC>`"]
pub type WORD1 = crate::Reg<word1::WORD1_SPEC>;
#[doc = "Lock Bits Word 1"]
pub mod word1;
