#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFEC Control Register"]
    pub afec_cr: AFEC_CR,
    #[doc = "0x04 - AFEC Mode Register"]
    pub afec_mr: AFEC_MR,
    #[doc = "0x08 - AFEC Extended Mode Register"]
    pub afec_emr: AFEC_EMR,
    #[doc = "0x0c - AFEC Channel Sequence 1 Register"]
    pub afec_seq1r: AFEC_SEQ1R,
    #[doc = "0x10 - AFEC Channel Sequence 2 Register"]
    pub afec_seq2r: AFEC_SEQ2R,
    #[doc = "0x14 - AFEC Channel Enable Register"]
    pub afec_cher: AFEC_CHER,
    #[doc = "0x18 - AFEC Channel Disable Register"]
    pub afec_chdr: AFEC_CHDR,
    #[doc = "0x1c - AFEC Channel Status Register"]
    pub afec_chsr: AFEC_CHSR,
    #[doc = "0x20 - AFEC Last Converted Data Register"]
    pub afec_lcdr: AFEC_LCDR,
    #[doc = "0x24 - AFEC Interrupt Enable Register"]
    pub afec_ier: AFEC_IER,
    #[doc = "0x28 - AFEC Interrupt Disable Register"]
    pub afec_idr: AFEC_IDR,
    #[doc = "0x2c - AFEC Interrupt Mask Register"]
    pub afec_imr: AFEC_IMR,
    #[doc = "0x30 - AFEC Interrupt Status Register"]
    pub afec_isr: AFEC_ISR,
    _reserved13: [u8; 24usize],
    #[doc = "0x4c - AFEC Overrun Status Register"]
    pub afec_over: AFEC_OVER,
    #[doc = "0x50 - AFEC Compare Window Register"]
    pub afec_cwr: AFEC_CWR,
    #[doc = "0x54 - AFEC Channel Gain Register"]
    pub afec_cgr: AFEC_CGR,
    _reserved16: [u8; 8usize],
    #[doc = "0x60 - AFEC Channel Differential Register"]
    pub afec_diffr: AFEC_DIFFR,
    #[doc = "0x64 - AFEC Channel Selection Register"]
    pub afec_cselr: AFEC_CSELR,
    #[doc = "0x68 - AFEC Channel Data Register"]
    pub afec_cdr: AFEC_CDR,
    #[doc = "0x6c - AFEC Channel Offset Compensation Register"]
    pub afec_cocr: AFEC_COCR,
    #[doc = "0x70 - AFEC Temperature Sensor Mode Register"]
    pub afec_tempmr: AFEC_TEMPMR,
    #[doc = "0x74 - AFEC Temperature Compare Window Register"]
    pub afec_tempcwr: AFEC_TEMPCWR,
    _reserved22: [u8; 28usize],
    #[doc = "0x94 - AFEC Analog Control Register"]
    pub afec_acr: AFEC_ACR,
    _reserved23: [u8; 8usize],
    #[doc = "0xa0 - AFEC Sample & Hold Mode Register"]
    pub afec_shmr: AFEC_SHMR,
    _reserved24: [u8; 44usize],
    #[doc = "0xd0 - AFEC Correction Select Register"]
    pub afec_cosr: AFEC_COSR,
    #[doc = "0xd4 - AFEC Correction Values Register"]
    pub afec_cvr: AFEC_CVR,
    #[doc = "0xd8 - AFEC Channel Error Correction Register"]
    pub afec_cecr: AFEC_CECR,
    _reserved27: [u8; 8usize],
    #[doc = "0xe4 - AFEC Write Protection Mode Register"]
    pub afec_wpmr: AFEC_WPMR,
    #[doc = "0xe8 - AFEC Write Protection Status Register"]
    pub afec_wpsr: AFEC_WPSR,
}
#[doc = "AFEC Control Register"]
pub struct AFEC_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Control Register"]
pub mod afec_cr;
#[doc = "AFEC Mode Register"]
pub struct AFEC_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Mode Register"]
pub mod afec_mr;
#[doc = "AFEC Extended Mode Register"]
pub struct AFEC_EMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Extended Mode Register"]
pub mod afec_emr;
#[doc = "AFEC Channel Sequence 1 Register"]
pub struct AFEC_SEQ1R {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Sequence 1 Register"]
pub mod afec_seq1r;
#[doc = "AFEC Channel Sequence 2 Register"]
pub struct AFEC_SEQ2R {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Sequence 2 Register"]
pub mod afec_seq2r;
#[doc = "AFEC Channel Enable Register"]
pub struct AFEC_CHER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Enable Register"]
pub mod afec_cher;
#[doc = "AFEC Channel Disable Register"]
pub struct AFEC_CHDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Disable Register"]
pub mod afec_chdr;
#[doc = "AFEC Channel Status Register"]
pub struct AFEC_CHSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Status Register"]
pub mod afec_chsr;
#[doc = "AFEC Last Converted Data Register"]
pub struct AFEC_LCDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Last Converted Data Register"]
pub mod afec_lcdr;
#[doc = "AFEC Interrupt Enable Register"]
pub struct AFEC_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Interrupt Enable Register"]
pub mod afec_ier;
#[doc = "AFEC Interrupt Disable Register"]
pub struct AFEC_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Interrupt Disable Register"]
pub mod afec_idr;
#[doc = "AFEC Interrupt Mask Register"]
pub struct AFEC_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Interrupt Mask Register"]
pub mod afec_imr;
#[doc = "AFEC Interrupt Status Register"]
pub struct AFEC_ISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Interrupt Status Register"]
pub mod afec_isr;
#[doc = "AFEC Overrun Status Register"]
pub struct AFEC_OVER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Overrun Status Register"]
pub mod afec_over;
#[doc = "AFEC Compare Window Register"]
pub struct AFEC_CWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Compare Window Register"]
pub mod afec_cwr;
#[doc = "AFEC Channel Gain Register"]
pub struct AFEC_CGR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Gain Register"]
pub mod afec_cgr;
#[doc = "AFEC Channel Differential Register"]
pub struct AFEC_DIFFR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Differential Register"]
pub mod afec_diffr;
#[doc = "AFEC Channel Selection Register"]
pub struct AFEC_CSELR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Selection Register"]
pub mod afec_cselr;
#[doc = "AFEC Channel Data Register"]
pub struct AFEC_CDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Data Register"]
pub mod afec_cdr;
#[doc = "AFEC Channel Offset Compensation Register"]
pub struct AFEC_COCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Offset Compensation Register"]
pub mod afec_cocr;
#[doc = "AFEC Temperature Sensor Mode Register"]
pub struct AFEC_TEMPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Temperature Sensor Mode Register"]
pub mod afec_tempmr;
#[doc = "AFEC Temperature Compare Window Register"]
pub struct AFEC_TEMPCWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Temperature Compare Window Register"]
pub mod afec_tempcwr;
#[doc = "AFEC Analog Control Register"]
pub struct AFEC_ACR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Analog Control Register"]
pub mod afec_acr;
#[doc = "AFEC Sample & Hold Mode Register"]
pub struct AFEC_SHMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Sample & Hold Mode Register"]
pub mod afec_shmr;
#[doc = "AFEC Correction Select Register"]
pub struct AFEC_COSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Correction Select Register"]
pub mod afec_cosr;
#[doc = "AFEC Correction Values Register"]
pub struct AFEC_CVR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Correction Values Register"]
pub mod afec_cvr;
#[doc = "AFEC Channel Error Correction Register"]
pub struct AFEC_CECR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Channel Error Correction Register"]
pub mod afec_cecr;
#[doc = "AFEC Write Protection Mode Register"]
pub struct AFEC_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Write Protection Mode Register"]
pub mod afec_wpmr;
#[doc = "AFEC Write Protection Status Register"]
pub struct AFEC_WPSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "AFEC Write Protection Status Register"]
pub mod afec_wpsr;
