#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x30 - Master Configuration Register 0"]
    pub mcfg: [crate::Reg<mcfg::MCFG_SPEC>; 12],
    _reserved1: [u8; 0x10],
    #[doc = "0x40..0x64 - Slave Configuration Register 0"]
    pub scfg: [crate::Reg<scfg::SCFG_SPEC>; 9],
    _reserved2: [u8; 0x1c],
    #[doc = "0x80..0xc8 - Priority Register A for Slave 0"]
    pub matrix_pr: [MATRIX_PR; 9],
    _reserved3: [u8; 0x38],
    #[doc = "0x100 - Master Remap Control Register"]
    pub mrcr: crate::Reg<mrcr::MRCR_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>,
    #[doc = "0x114 - System I/O Configuration Register"]
    pub ccfg_sysio: crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub pras: crate::Reg<self::matrix_pr::pras::PRAS_SPEC>,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub prbs: crate::Reg<self::matrix_pr::prbs::PRBS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "MCFG register accessor: an alias for `Reg<MCFG_SPEC>`"]
pub type MCFG = crate::Reg<mcfg::MCFG_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod mcfg;
#[doc = "SCFG register accessor: an alias for `Reg<SCFG_SPEC>`"]
pub type SCFG = crate::Reg<scfg::SCFG_SPEC>;
#[doc = "Slave Configuration Register 0"]
pub mod scfg;
#[doc = "MRCR register accessor: an alias for `Reg<MRCR_SPEC>`"]
pub type MRCR = crate::Reg<mrcr::MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "CCFG_CAN0 register accessor: an alias for `Reg<CCFG_CAN0_SPEC>`"]
pub type CCFG_CAN0 = crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>;
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "CCFG_SYSIO register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_SMCNFCS register accessor: an alias for `Reg<CCFG_SMCNFCS_SPEC>`"]
pub type CCFG_SMCNFCS = crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>;
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
