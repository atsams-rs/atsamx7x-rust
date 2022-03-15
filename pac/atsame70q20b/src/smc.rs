#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - SMC Setup Register"]
    pub smc_cs_number: [SMC_CS_NUMBER; 4],
    _reserved1: [u8; 0x40],
    #[doc = "0x80 - SMC Off-Chip Memory Scrambling Register"]
    pub smc_ocms: crate::Reg<smc_ocms::SMC_OCMS_SPEC>,
    #[doc = "0x84 - SMC Off-Chip Memory Scrambling KEY1 Register"]
    pub smc_key1: crate::Reg<smc_key1::SMC_KEY1_SPEC>,
    #[doc = "0x88 - SMC Off-Chip Memory Scrambling KEY2 Register"]
    pub smc_key2: crate::Reg<smc_key2::SMC_KEY2_SPEC>,
    _reserved4: [u8; 0x58],
    #[doc = "0xe4 - SMC Write Protection Mode Register"]
    pub smc_wpmr: crate::Reg<smc_wpmr::SMC_WPMR_SPEC>,
    #[doc = "0xe8 - SMC Write Protection Status Register"]
    pub smc_wpsr: crate::Reg<smc_wpsr::SMC_WPSR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SMC_CS_NUMBER {
    #[doc = "0x00 - SMC Setup Register"]
    pub smc_setup: crate::Reg<self::smc_cs_number::smc_setup::SMC_SETUP_SPEC>,
    #[doc = "0x04 - SMC Pulse Register"]
    pub smc_pulse: crate::Reg<self::smc_cs_number::smc_pulse::SMC_PULSE_SPEC>,
    #[doc = "0x08 - SMC Cycle Register"]
    pub smc_cycle: crate::Reg<self::smc_cs_number::smc_cycle::SMC_CYCLE_SPEC>,
    #[doc = "0x0c - SMC Mode Register"]
    pub smc_mode: crate::Reg<self::smc_cs_number::smc_mode::SMC_MODE_SPEC>,
}
#[doc = r"Register block"]
#[doc = "SMC Setup Register"]
pub mod smc_cs_number;
#[doc = "SMC_OCMS register accessor: an alias for `Reg<SMC_OCMS_SPEC>`"]
pub type SMC_OCMS = crate::Reg<smc_ocms::SMC_OCMS_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling Register"]
pub mod smc_ocms;
#[doc = "SMC_KEY1 register accessor: an alias for `Reg<SMC_KEY1_SPEC>`"]
pub type SMC_KEY1 = crate::Reg<smc_key1::SMC_KEY1_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling KEY1 Register"]
pub mod smc_key1;
#[doc = "SMC_KEY2 register accessor: an alias for `Reg<SMC_KEY2_SPEC>`"]
pub type SMC_KEY2 = crate::Reg<smc_key2::SMC_KEY2_SPEC>;
#[doc = "SMC Off-Chip Memory Scrambling KEY2 Register"]
pub mod smc_key2;
#[doc = "SMC_WPMR register accessor: an alias for `Reg<SMC_WPMR_SPEC>`"]
pub type SMC_WPMR = crate::Reg<smc_wpmr::SMC_WPMR_SPEC>;
#[doc = "SMC Write Protection Mode Register"]
pub mod smc_wpmr;
#[doc = "SMC_WPSR register accessor: an alias for `Reg<SMC_WPSR_SPEC>`"]
pub type SMC_WPSR = crate::Reg<smc_wpsr::SMC_WPSR_SPEC>;
#[doc = "SMC Write Protection Status Register"]
pub mod smc_wpsr;
