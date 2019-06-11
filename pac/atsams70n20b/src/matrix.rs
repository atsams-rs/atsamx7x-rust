#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    pub matrix_mcfg: [MATRIX_MCFG; 13],
    _reserved0: [u8; 12usize],
    #[doc = "0x40 - Slave Configuration Register 0"]
    pub matrix_scfg: [MATRIX_SCFG; 9],
    _reserved1: [u8; 28usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pr: [MATRIX_PR; 9],
    _reserved2: [u8; 56usize],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: MATRIX_MRCR,
    _reserved3: [u8; 16usize],
    #[doc = "0x114 - System I/O Configuration Register"]
    pub ccfg_sysio: CCFG_SYSIO,
    #[doc = "0x118 - Peripheral Clock Configuration Register"]
    pub ccfg_pccr: CCFG_PCCR,
    #[doc = "0x11c - Dynamic Clock Gating Register"]
    pub ccfg_dynckg: CCFG_DYNCKG,
    _reserved4: [u8; 4usize],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved5: [u8; 188usize],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub matrix_pras: self::matrix_pr::MATRIX_PRAS,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub matrix_prbs: self::matrix_pr::MATRIX_PRBS,
}
#[doc = r" Register block"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "Master Configuration Register 0"]
pub struct MATRIX_MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Configuration Register 0"]
pub mod matrix_mcfg;
#[doc = "Slave Configuration Register 0"]
pub struct MATRIX_SCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Configuration Register 0"]
pub mod matrix_scfg;
#[doc = "Master Remap Control Register"]
pub struct MATRIX_MRCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "System I/O Configuration Register"]
pub struct CCFG_SYSIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System I/O Configuration Register"]
pub mod ccfg_sysio;
#[doc = "Peripheral Clock Configuration Register"]
pub struct CCFG_PCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral Clock Configuration Register"]
pub mod ccfg_pccr;
#[doc = "Dynamic Clock Gating Register"]
pub struct CCFG_DYNCKG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dynamic Clock Gating Register"]
pub mod ccfg_dynckg;
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub struct CCFG_SMCNFCS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "Write Protection Mode Register"]
pub struct MATRIX_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "Write Protection Status Register"]
pub struct MATRIX_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
