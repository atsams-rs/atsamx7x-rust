#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x30 - Master Configuration Register 0"]
    pub matrix_mcfg: [crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>; 12],
    _reserved1: [u8; 0x10],
    #[doc = "0x40..0x64 - Slave Configuration Register 0"]
    pub matrix_scfg: [crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>; 9],
    _reserved2: [u8; 0x1c],
    #[doc = "0x80..0xc8 - Priority Register A for Slave 0"]
    pub matrix_pr: [MATRIX_PR; 9],
    _reserved3: [u8; 0x38],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>,
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    pub ccfg_sysio: crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>,
    _reserved7: [u8; 0xbc],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>,
    _reserved9: [u8; 0x10],
    #[doc = "0x1fc - Version Register"]
    pub matrix_version: crate::Reg<matrix_version::MATRIX_VERSION_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub matrix_pras: crate::Reg<self::matrix_pr::matrix_pras::MATRIX_PRAS_SPEC>,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub matrix_prbs: crate::Reg<self::matrix_pr::matrix_prbs::MATRIX_PRBS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "MATRIX_MCFG register accessor: an alias for `Reg<MATRIX_MCFG_SPEC>`"]
pub type MATRIX_MCFG = crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>;
#[doc = "Master Configuration Register 0"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG register accessor: an alias for `Reg<MATRIX_SCFG_SPEC>`"]
pub type MATRIX_SCFG = crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>;
#[doc = "Slave Configuration Register 0"]
pub mod matrix_scfg;
#[doc = "MATRIX_MRCR register accessor: an alias for `Reg<MATRIX_MRCR_SPEC>`"]
pub type MATRIX_MRCR = crate::Reg<matrix_mrcr::MATRIX_MRCR_SPEC>;
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "CCFG_CAN0 register accessor: an alias for `Reg<CCFG_CAN0_SPEC>`"]
pub type CCFG_CAN0 = crate::Reg<ccfg_can0::CCFG_CAN0_SPEC>;
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "CCFG_SYSIO register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_SMCNFCS register accessor: an alias for `Reg<CCFG_SMCNFCS_SPEC>`"]
pub type CCFG_SMCNFCS = crate::Reg<ccfg_smcnfcs::CCFG_SMCNFCS_SPEC>;
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "MATRIX_WPMR register accessor: an alias for `Reg<MATRIX_WPMR_SPEC>`"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR register accessor: an alias for `Reg<MATRIX_WPSR_SPEC>`"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
#[doc = "MATRIX_VERSION register accessor: an alias for `Reg<MATRIX_VERSION_SPEC>`"]
pub type MATRIX_VERSION = crate::Reg<matrix_version::MATRIX_VERSION_SPEC>;
#[doc = "Version Register"]
pub mod matrix_version;
