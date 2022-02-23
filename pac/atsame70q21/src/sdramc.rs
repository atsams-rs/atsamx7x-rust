#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    pub sdramc_mr: crate::Reg<sdramc_mr::SDRAMC_MR_SPEC>,
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    pub sdramc_tr: crate::Reg<sdramc_tr::SDRAMC_TR_SPEC>,
    #[doc = "0x08 - SDRAMC Configuration Register"]
    pub sdramc_cr: crate::Reg<sdramc_cr::SDRAMC_CR_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - SDRAMC Low Power Register"]
    pub sdramc_lpr: crate::Reg<sdramc_lpr::SDRAMC_LPR_SPEC>,
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    pub sdramc_ier: crate::Reg<sdramc_ier::SDRAMC_IER_SPEC>,
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    pub sdramc_idr: crate::Reg<sdramc_idr::SDRAMC_IDR_SPEC>,
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    pub sdramc_imr: crate::Reg<sdramc_imr::SDRAMC_IMR_SPEC>,
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    pub sdramc_isr: crate::Reg<sdramc_isr::SDRAMC_ISR_SPEC>,
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    pub sdramc_mdr: crate::Reg<sdramc_mdr::SDRAMC_MDR_SPEC>,
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    pub sdramc_cfr1: crate::Reg<sdramc_cfr1::SDRAMC_CFR1_SPEC>,
    #[doc = "0x2c - SDRAMC OCMS Register"]
    pub sdramc_ocms: crate::Reg<sdramc_ocms::SDRAMC_OCMS_SPEC>,
    #[doc = "0x30 - SDRAMC OCMS KEY1 Register"]
    pub sdramc_ocms_key1: crate::Reg<sdramc_ocms_key1::SDRAMC_OCMS_KEY1_SPEC>,
    #[doc = "0x34 - SDRAMC OCMS KEY2 Register"]
    pub sdramc_ocms_key2: crate::Reg<sdramc_ocms_key2::SDRAMC_OCMS_KEY2_SPEC>,
}
#[doc = "SDRAMC_MR register accessor: an alias for `Reg<SDRAMC_MR_SPEC>`"]
pub type SDRAMC_MR = crate::Reg<sdramc_mr::SDRAMC_MR_SPEC>;
#[doc = "SDRAMC Mode Register"]
pub mod sdramc_mr;
#[doc = "SDRAMC_TR register accessor: an alias for `Reg<SDRAMC_TR_SPEC>`"]
pub type SDRAMC_TR = crate::Reg<sdramc_tr::SDRAMC_TR_SPEC>;
#[doc = "SDRAMC Refresh Timer Register"]
pub mod sdramc_tr;
#[doc = "SDRAMC_CR register accessor: an alias for `Reg<SDRAMC_CR_SPEC>`"]
pub type SDRAMC_CR = crate::Reg<sdramc_cr::SDRAMC_CR_SPEC>;
#[doc = "SDRAMC Configuration Register"]
pub mod sdramc_cr;
#[doc = "SDRAMC_LPR register accessor: an alias for `Reg<SDRAMC_LPR_SPEC>`"]
pub type SDRAMC_LPR = crate::Reg<sdramc_lpr::SDRAMC_LPR_SPEC>;
#[doc = "SDRAMC Low Power Register"]
pub mod sdramc_lpr;
#[doc = "SDRAMC_IER register accessor: an alias for `Reg<SDRAMC_IER_SPEC>`"]
pub type SDRAMC_IER = crate::Reg<sdramc_ier::SDRAMC_IER_SPEC>;
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod sdramc_ier;
#[doc = "SDRAMC_IDR register accessor: an alias for `Reg<SDRAMC_IDR_SPEC>`"]
pub type SDRAMC_IDR = crate::Reg<sdramc_idr::SDRAMC_IDR_SPEC>;
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod sdramc_idr;
#[doc = "SDRAMC_IMR register accessor: an alias for `Reg<SDRAMC_IMR_SPEC>`"]
pub type SDRAMC_IMR = crate::Reg<sdramc_imr::SDRAMC_IMR_SPEC>;
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod sdramc_imr;
#[doc = "SDRAMC_ISR register accessor: an alias for `Reg<SDRAMC_ISR_SPEC>`"]
pub type SDRAMC_ISR = crate::Reg<sdramc_isr::SDRAMC_ISR_SPEC>;
#[doc = "SDRAMC Interrupt Status Register"]
pub mod sdramc_isr;
#[doc = "SDRAMC_MDR register accessor: an alias for `Reg<SDRAMC_MDR_SPEC>`"]
pub type SDRAMC_MDR = crate::Reg<sdramc_mdr::SDRAMC_MDR_SPEC>;
#[doc = "SDRAMC Memory Device Register"]
pub mod sdramc_mdr;
#[doc = "SDRAMC_CFR1 register accessor: an alias for `Reg<SDRAMC_CFR1_SPEC>`"]
pub type SDRAMC_CFR1 = crate::Reg<sdramc_cfr1::SDRAMC_CFR1_SPEC>;
#[doc = "SDRAMC Configuration Register 1"]
pub mod sdramc_cfr1;
#[doc = "SDRAMC_OCMS register accessor: an alias for `Reg<SDRAMC_OCMS_SPEC>`"]
pub type SDRAMC_OCMS = crate::Reg<sdramc_ocms::SDRAMC_OCMS_SPEC>;
#[doc = "SDRAMC OCMS Register"]
pub mod sdramc_ocms;
#[doc = "SDRAMC_OCMS_KEY1 register accessor: an alias for `Reg<SDRAMC_OCMS_KEY1_SPEC>`"]
pub type SDRAMC_OCMS_KEY1 = crate::Reg<sdramc_ocms_key1::SDRAMC_OCMS_KEY1_SPEC>;
#[doc = "SDRAMC OCMS KEY1 Register"]
pub mod sdramc_ocms_key1;
#[doc = "SDRAMC_OCMS_KEY2 register accessor: an alias for `Reg<SDRAMC_OCMS_KEY2_SPEC>`"]
pub type SDRAMC_OCMS_KEY2 = crate::Reg<sdramc_ocms_key2::SDRAMC_OCMS_KEY2_SPEC>;
#[doc = "SDRAMC OCMS KEY2 Register"]
pub mod sdramc_ocms_key2;
