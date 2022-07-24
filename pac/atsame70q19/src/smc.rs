#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - SMC Setup Register (CS_number = 0)"]
    pub smc_cs_number: [SMC_CS_NUMBER; 4],
    _reserved1: [u8; 0x40],
    #[doc = "0x80 - SMC OCMS MODE Register"]
    pub ocms: crate::Reg<ocms::OCMS_SPEC>,
    #[doc = "0x84 - SMC OCMS KEY1 Register"]
    pub key1: crate::Reg<key1::KEY1_SPEC>,
    #[doc = "0x88 - SMC OCMS KEY2 Register"]
    pub key2: crate::Reg<key2::KEY2_SPEC>,
    _reserved4: [u8; 0x58],
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register (CS_number = 0)"]
    pub setup: crate::Reg<self::smc_cs_number::setup::SETUP_SPEC>,
    #[doc = "0x04 - SMC Pulse Register (CS_number = 0)"]
    pub pulse: crate::Reg<self::smc_cs_number::pulse::PULSE_SPEC>,
    #[doc = "0x08 - SMC Cycle Register (CS_number = 0)"]
    pub cycle: crate::Reg<self::smc_cs_number::cycle::CYCLE_SPEC>,
    #[doc = "0x0c - SMC MODE Register (CS_number = 0)"]
    pub mode: crate::Reg<self::smc_cs_number::mode::MODE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SMC Setup Register (CS_number = 0)"]
pub mod smc_cs_number;
#[doc = "OCMS register accessor: an alias for `Reg<OCMS_SPEC>`"]
pub type OCMS = crate::Reg<ocms::OCMS_SPEC>;
#[doc = "SMC OCMS MODE Register"]
pub mod ocms;
#[doc = "KEY1 register accessor: an alias for `Reg<KEY1_SPEC>`"]
pub type KEY1 = crate::Reg<key1::KEY1_SPEC>;
#[doc = "SMC OCMS KEY1 Register"]
pub mod key1;
#[doc = "KEY2 register accessor: an alias for `Reg<KEY2_SPEC>`"]
pub type KEY2 = crate::Reg<key2::KEY2_SPEC>;
#[doc = "SMC OCMS KEY2 Register"]
pub mod key2;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "SMC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "SMC Write Protection Status Register"]
pub mod wpsr;
