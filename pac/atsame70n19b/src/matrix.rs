#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Master Configuration Register 0"]
    pub matrix_mcfg: [MATRIX_MCFG; 13],
    _reserved1: [u8; 12usize],
    #[doc = "0x40 - Slave Configuration Register 0"]
    pub matrix_scfg: [MATRIX_SCFG; 9],
    _reserved2: [u8; 28usize],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pr: [MATRIX_PR; 9],
    _reserved3: [u8; 56usize],
    #[doc = "0x100 - Master Remap Control Register"]
    pub matrix_mrcr: MATRIX_MRCR,
    _reserved4: [u8; 12usize],
    #[doc = "0x110 - CAN0 Configuration Register"]
    pub ccfg_can0: CCFG_CAN0,
    #[doc = "0x114 - System I/O and CAN1 Configuration Register"]
    pub ccfg_sysio: CCFG_SYSIO,
    #[doc = "0x118 - Peripheral Clock Configuration Register"]
    pub ccfg_pccr: CCFG_PCCR,
    #[doc = "0x11c - Dynamic Clock Gating Register"]
    pub ccfg_dynckg: CCFG_DYNCKG,
    _reserved8: [u8; 4usize],
    #[doc = "0x124 - SMC NAND Flash Chip Select Configuration Register"]
    pub ccfg_smcnfcs: CCFG_SMCNFCS,
    _reserved9: [u8; 188usize],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: MATRIX_WPMR,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: MATRIX_WPSR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct MATRIX_PR {
    #[doc = "0x00 - Priority Register A for Slave 0"]
    pub matrix_pras: self::matrix_pr::MATRIX_PRAS,
    #[doc = "0x04 - Priority Register B for Slave 0"]
    pub matrix_prbs: self::matrix_pr::MATRIX_PRBS,
}
#[doc = r"Register block"]
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pr;
#[doc = "Master Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](matrix_mcfg) module"]
pub type MATRIX_MCFG = crate::Reg<u32, _MATRIX_MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_MCFG;
#[doc = "`read()` method returns [matrix_mcfg::R](matrix_mcfg::R) reader structure"]
impl crate::Readable for MATRIX_MCFG {}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](matrix_mcfg::W) writer structure"]
impl crate::Writable for MATRIX_MCFG {}
#[doc = "Master Configuration Register 0"]
pub mod matrix_mcfg;
#[doc = "Slave Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_scfg](matrix_scfg) module"]
pub type MATRIX_SCFG = crate::Reg<u32, _MATRIX_SCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_SCFG;
#[doc = "`read()` method returns [matrix_scfg::R](matrix_scfg::R) reader structure"]
impl crate::Readable for MATRIX_SCFG {}
#[doc = "`write(|w| ..)` method takes [matrix_scfg::W](matrix_scfg::W) writer structure"]
impl crate::Writable for MATRIX_SCFG {}
#[doc = "Slave Configuration Register 0"]
pub mod matrix_scfg;
#[doc = "Master Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mrcr](matrix_mrcr) module"]
pub type MATRIX_MRCR = crate::Reg<u32, _MATRIX_MRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_MRCR;
#[doc = "`read()` method returns [matrix_mrcr::R](matrix_mrcr::R) reader structure"]
impl crate::Readable for MATRIX_MRCR {}
#[doc = "`write(|w| ..)` method takes [matrix_mrcr::W](matrix_mrcr::W) writer structure"]
impl crate::Writable for MATRIX_MRCR {}
#[doc = "Master Remap Control Register"]
pub mod matrix_mrcr;
#[doc = "CAN0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_can0](ccfg_can0) module"]
pub type CCFG_CAN0 = crate::Reg<u32, _CCFG_CAN0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_CAN0;
#[doc = "`read()` method returns [ccfg_can0::R](ccfg_can0::R) reader structure"]
impl crate::Readable for CCFG_CAN0 {}
#[doc = "`write(|w| ..)` method takes [ccfg_can0::W](ccfg_can0::W) writer structure"]
impl crate::Writable for CCFG_CAN0 {}
#[doc = "CAN0 Configuration Register"]
pub mod ccfg_can0;
#[doc = "System I/O and CAN1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](ccfg_sysio) module"]
pub type CCFG_SYSIO = crate::Reg<u32, _CCFG_SYSIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SYSIO;
#[doc = "`read()` method returns [ccfg_sysio::R](ccfg_sysio::R) reader structure"]
impl crate::Readable for CCFG_SYSIO {}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](ccfg_sysio::W) writer structure"]
impl crate::Writable for CCFG_SYSIO {}
#[doc = "System I/O and CAN1 Configuration Register"]
pub mod ccfg_sysio;
#[doc = "Peripheral Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_pccr](ccfg_pccr) module"]
pub type CCFG_PCCR = crate::Reg<u32, _CCFG_PCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_PCCR;
#[doc = "`read()` method returns [ccfg_pccr::R](ccfg_pccr::R) reader structure"]
impl crate::Readable for CCFG_PCCR {}
#[doc = "`write(|w| ..)` method takes [ccfg_pccr::W](ccfg_pccr::W) writer structure"]
impl crate::Writable for CCFG_PCCR {}
#[doc = "Peripheral Clock Configuration Register"]
pub mod ccfg_pccr;
#[doc = "Dynamic Clock Gating Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_dynckg](ccfg_dynckg) module"]
pub type CCFG_DYNCKG = crate::Reg<u32, _CCFG_DYNCKG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_DYNCKG;
#[doc = "`read()` method returns [ccfg_dynckg::R](ccfg_dynckg::R) reader structure"]
impl crate::Readable for CCFG_DYNCKG {}
#[doc = "`write(|w| ..)` method takes [ccfg_dynckg::W](ccfg_dynckg::W) writer structure"]
impl crate::Writable for CCFG_DYNCKG {}
#[doc = "Dynamic Clock Gating Register"]
pub mod ccfg_dynckg;
#[doc = "SMC NAND Flash Chip Select Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_smcnfcs](ccfg_smcnfcs) module"]
pub type CCFG_SMCNFCS = crate::Reg<u32, _CCFG_SMCNFCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCFG_SMCNFCS;
#[doc = "`read()` method returns [ccfg_smcnfcs::R](ccfg_smcnfcs::R) reader structure"]
impl crate::Readable for CCFG_SMCNFCS {}
#[doc = "`write(|w| ..)` method takes [ccfg_smcnfcs::W](ccfg_smcnfcs::W) writer structure"]
impl crate::Writable for CCFG_SMCNFCS {}
#[doc = "SMC NAND Flash Chip Select Configuration Register"]
pub mod ccfg_smcnfcs;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpmr](matrix_wpmr) module"]
pub type MATRIX_WPMR = crate::Reg<u32, _MATRIX_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_WPMR;
#[doc = "`read()` method returns [matrix_wpmr::R](matrix_wpmr::R) reader structure"]
impl crate::Readable for MATRIX_WPMR {}
#[doc = "`write(|w| ..)` method takes [matrix_wpmr::W](matrix_wpmr::W) writer structure"]
impl crate::Writable for MATRIX_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpsr](matrix_wpsr) module"]
pub type MATRIX_WPSR = crate::Reg<u32, _MATRIX_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_WPSR;
#[doc = "`read()` method returns [matrix_wpsr::R](matrix_wpsr::R) reader structure"]
impl crate::Readable for MATRIX_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
