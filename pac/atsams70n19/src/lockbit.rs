#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub word0: crate::Reg<word0::WORD0_SPEC>,
}
#[doc = "WORD0 register accessor: an alias for `Reg<WORD0_SPEC>`"]
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
#[doc = "Lock Bits Word 0"]
pub mod word0;
