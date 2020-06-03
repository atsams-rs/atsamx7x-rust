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
#[doc = "AFEC Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cr](afec_cr) module"]
pub type AFEC_CR = crate::Reg<u32, _AFEC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CR;
#[doc = "`write(|w| ..)` method takes [afec_cr::W](afec_cr::W) writer structure"]
impl crate::Writable for AFEC_CR {}
#[doc = "AFEC Control Register"]
pub mod afec_cr;
#[doc = "AFEC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_mr](afec_mr) module"]
pub type AFEC_MR = crate::Reg<u32, _AFEC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_MR;
#[doc = "`read()` method returns [afec_mr::R](afec_mr::R) reader structure"]
impl crate::Readable for AFEC_MR {}
#[doc = "`write(|w| ..)` method takes [afec_mr::W](afec_mr::W) writer structure"]
impl crate::Writable for AFEC_MR {}
#[doc = "AFEC Mode Register"]
pub mod afec_mr;
#[doc = "AFEC Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_emr](afec_emr) module"]
pub type AFEC_EMR = crate::Reg<u32, _AFEC_EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_EMR;
#[doc = "`read()` method returns [afec_emr::R](afec_emr::R) reader structure"]
impl crate::Readable for AFEC_EMR {}
#[doc = "`write(|w| ..)` method takes [afec_emr::W](afec_emr::W) writer structure"]
impl crate::Writable for AFEC_EMR {}
#[doc = "AFEC Extended Mode Register"]
pub mod afec_emr;
#[doc = "AFEC Channel Sequence 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_seq1r](afec_seq1r) module"]
pub type AFEC_SEQ1R = crate::Reg<u32, _AFEC_SEQ1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_SEQ1R;
#[doc = "`read()` method returns [afec_seq1r::R](afec_seq1r::R) reader structure"]
impl crate::Readable for AFEC_SEQ1R {}
#[doc = "`write(|w| ..)` method takes [afec_seq1r::W](afec_seq1r::W) writer structure"]
impl crate::Writable for AFEC_SEQ1R {}
#[doc = "AFEC Channel Sequence 1 Register"]
pub mod afec_seq1r;
#[doc = "AFEC Channel Sequence 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_seq2r](afec_seq2r) module"]
pub type AFEC_SEQ2R = crate::Reg<u32, _AFEC_SEQ2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_SEQ2R;
#[doc = "`read()` method returns [afec_seq2r::R](afec_seq2r::R) reader structure"]
impl crate::Readable for AFEC_SEQ2R {}
#[doc = "`write(|w| ..)` method takes [afec_seq2r::W](afec_seq2r::W) writer structure"]
impl crate::Writable for AFEC_SEQ2R {}
#[doc = "AFEC Channel Sequence 2 Register"]
pub mod afec_seq2r;
#[doc = "AFEC Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cher](afec_cher) module"]
pub type AFEC_CHER = crate::Reg<u32, _AFEC_CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CHER;
#[doc = "`write(|w| ..)` method takes [afec_cher::W](afec_cher::W) writer structure"]
impl crate::Writable for AFEC_CHER {}
#[doc = "AFEC Channel Enable Register"]
pub mod afec_cher;
#[doc = "AFEC Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_chdr](afec_chdr) module"]
pub type AFEC_CHDR = crate::Reg<u32, _AFEC_CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CHDR;
#[doc = "`write(|w| ..)` method takes [afec_chdr::W](afec_chdr::W) writer structure"]
impl crate::Writable for AFEC_CHDR {}
#[doc = "AFEC Channel Disable Register"]
pub mod afec_chdr;
#[doc = "AFEC Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_chsr](afec_chsr) module"]
pub type AFEC_CHSR = crate::Reg<u32, _AFEC_CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CHSR;
#[doc = "`read()` method returns [afec_chsr::R](afec_chsr::R) reader structure"]
impl crate::Readable for AFEC_CHSR {}
#[doc = "AFEC Channel Status Register"]
pub mod afec_chsr;
#[doc = "AFEC Last Converted Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_lcdr](afec_lcdr) module"]
pub type AFEC_LCDR = crate::Reg<u32, _AFEC_LCDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_LCDR;
#[doc = "`read()` method returns [afec_lcdr::R](afec_lcdr::R) reader structure"]
impl crate::Readable for AFEC_LCDR {}
#[doc = "AFEC Last Converted Data Register"]
pub mod afec_lcdr;
#[doc = "AFEC Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_ier](afec_ier) module"]
pub type AFEC_IER = crate::Reg<u32, _AFEC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_IER;
#[doc = "`write(|w| ..)` method takes [afec_ier::W](afec_ier::W) writer structure"]
impl crate::Writable for AFEC_IER {}
#[doc = "AFEC Interrupt Enable Register"]
pub mod afec_ier;
#[doc = "AFEC Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_idr](afec_idr) module"]
pub type AFEC_IDR = crate::Reg<u32, _AFEC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_IDR;
#[doc = "`write(|w| ..)` method takes [afec_idr::W](afec_idr::W) writer structure"]
impl crate::Writable for AFEC_IDR {}
#[doc = "AFEC Interrupt Disable Register"]
pub mod afec_idr;
#[doc = "AFEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_imr](afec_imr) module"]
pub type AFEC_IMR = crate::Reg<u32, _AFEC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_IMR;
#[doc = "`read()` method returns [afec_imr::R](afec_imr::R) reader structure"]
impl crate::Readable for AFEC_IMR {}
#[doc = "AFEC Interrupt Mask Register"]
pub mod afec_imr;
#[doc = "AFEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_isr](afec_isr) module"]
pub type AFEC_ISR = crate::Reg<u32, _AFEC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_ISR;
#[doc = "`read()` method returns [afec_isr::R](afec_isr::R) reader structure"]
impl crate::Readable for AFEC_ISR {}
#[doc = "AFEC Interrupt Status Register"]
pub mod afec_isr;
#[doc = "AFEC Overrun Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_over](afec_over) module"]
pub type AFEC_OVER = crate::Reg<u32, _AFEC_OVER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_OVER;
#[doc = "`read()` method returns [afec_over::R](afec_over::R) reader structure"]
impl crate::Readable for AFEC_OVER {}
#[doc = "AFEC Overrun Status Register"]
pub mod afec_over;
#[doc = "AFEC Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cwr](afec_cwr) module"]
pub type AFEC_CWR = crate::Reg<u32, _AFEC_CWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CWR;
#[doc = "`read()` method returns [afec_cwr::R](afec_cwr::R) reader structure"]
impl crate::Readable for AFEC_CWR {}
#[doc = "`write(|w| ..)` method takes [afec_cwr::W](afec_cwr::W) writer structure"]
impl crate::Writable for AFEC_CWR {}
#[doc = "AFEC Compare Window Register"]
pub mod afec_cwr;
#[doc = "AFEC Channel Gain Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cgr](afec_cgr) module"]
pub type AFEC_CGR = crate::Reg<u32, _AFEC_CGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CGR;
#[doc = "`read()` method returns [afec_cgr::R](afec_cgr::R) reader structure"]
impl crate::Readable for AFEC_CGR {}
#[doc = "`write(|w| ..)` method takes [afec_cgr::W](afec_cgr::W) writer structure"]
impl crate::Writable for AFEC_CGR {}
#[doc = "AFEC Channel Gain Register"]
pub mod afec_cgr;
#[doc = "AFEC Channel Differential Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_diffr](afec_diffr) module"]
pub type AFEC_DIFFR = crate::Reg<u32, _AFEC_DIFFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_DIFFR;
#[doc = "`read()` method returns [afec_diffr::R](afec_diffr::R) reader structure"]
impl crate::Readable for AFEC_DIFFR {}
#[doc = "`write(|w| ..)` method takes [afec_diffr::W](afec_diffr::W) writer structure"]
impl crate::Writable for AFEC_DIFFR {}
#[doc = "AFEC Channel Differential Register"]
pub mod afec_diffr;
#[doc = "AFEC Channel Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cselr](afec_cselr) module"]
pub type AFEC_CSELR = crate::Reg<u32, _AFEC_CSELR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CSELR;
#[doc = "`read()` method returns [afec_cselr::R](afec_cselr::R) reader structure"]
impl crate::Readable for AFEC_CSELR {}
#[doc = "`write(|w| ..)` method takes [afec_cselr::W](afec_cselr::W) writer structure"]
impl crate::Writable for AFEC_CSELR {}
#[doc = "AFEC Channel Selection Register"]
pub mod afec_cselr;
#[doc = "AFEC Channel Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cdr](afec_cdr) module"]
pub type AFEC_CDR = crate::Reg<u32, _AFEC_CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CDR;
#[doc = "`read()` method returns [afec_cdr::R](afec_cdr::R) reader structure"]
impl crate::Readable for AFEC_CDR {}
#[doc = "AFEC Channel Data Register"]
pub mod afec_cdr;
#[doc = "AFEC Channel Offset Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cocr](afec_cocr) module"]
pub type AFEC_COCR = crate::Reg<u32, _AFEC_COCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_COCR;
#[doc = "`read()` method returns [afec_cocr::R](afec_cocr::R) reader structure"]
impl crate::Readable for AFEC_COCR {}
#[doc = "`write(|w| ..)` method takes [afec_cocr::W](afec_cocr::W) writer structure"]
impl crate::Writable for AFEC_COCR {}
#[doc = "AFEC Channel Offset Compensation Register"]
pub mod afec_cocr;
#[doc = "AFEC Temperature Sensor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_tempmr](afec_tempmr) module"]
pub type AFEC_TEMPMR = crate::Reg<u32, _AFEC_TEMPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_TEMPMR;
#[doc = "`read()` method returns [afec_tempmr::R](afec_tempmr::R) reader structure"]
impl crate::Readable for AFEC_TEMPMR {}
#[doc = "`write(|w| ..)` method takes [afec_tempmr::W](afec_tempmr::W) writer structure"]
impl crate::Writable for AFEC_TEMPMR {}
#[doc = "AFEC Temperature Sensor Mode Register"]
pub mod afec_tempmr;
#[doc = "AFEC Temperature Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_tempcwr](afec_tempcwr) module"]
pub type AFEC_TEMPCWR = crate::Reg<u32, _AFEC_TEMPCWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_TEMPCWR;
#[doc = "`read()` method returns [afec_tempcwr::R](afec_tempcwr::R) reader structure"]
impl crate::Readable for AFEC_TEMPCWR {}
#[doc = "`write(|w| ..)` method takes [afec_tempcwr::W](afec_tempcwr::W) writer structure"]
impl crate::Writable for AFEC_TEMPCWR {}
#[doc = "AFEC Temperature Compare Window Register"]
pub mod afec_tempcwr;
#[doc = "AFEC Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_acr](afec_acr) module"]
pub type AFEC_ACR = crate::Reg<u32, _AFEC_ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_ACR;
#[doc = "`read()` method returns [afec_acr::R](afec_acr::R) reader structure"]
impl crate::Readable for AFEC_ACR {}
#[doc = "`write(|w| ..)` method takes [afec_acr::W](afec_acr::W) writer structure"]
impl crate::Writable for AFEC_ACR {}
#[doc = "AFEC Analog Control Register"]
pub mod afec_acr;
#[doc = "AFEC Sample & Hold Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_shmr](afec_shmr) module"]
pub type AFEC_SHMR = crate::Reg<u32, _AFEC_SHMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_SHMR;
#[doc = "`read()` method returns [afec_shmr::R](afec_shmr::R) reader structure"]
impl crate::Readable for AFEC_SHMR {}
#[doc = "`write(|w| ..)` method takes [afec_shmr::W](afec_shmr::W) writer structure"]
impl crate::Writable for AFEC_SHMR {}
#[doc = "AFEC Sample & Hold Mode Register"]
pub mod afec_shmr;
#[doc = "AFEC Correction Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cosr](afec_cosr) module"]
pub type AFEC_COSR = crate::Reg<u32, _AFEC_COSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_COSR;
#[doc = "`read()` method returns [afec_cosr::R](afec_cosr::R) reader structure"]
impl crate::Readable for AFEC_COSR {}
#[doc = "`write(|w| ..)` method takes [afec_cosr::W](afec_cosr::W) writer structure"]
impl crate::Writable for AFEC_COSR {}
#[doc = "AFEC Correction Select Register"]
pub mod afec_cosr;
#[doc = "AFEC Correction Values Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cvr](afec_cvr) module"]
pub type AFEC_CVR = crate::Reg<u32, _AFEC_CVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CVR;
#[doc = "`read()` method returns [afec_cvr::R](afec_cvr::R) reader structure"]
impl crate::Readable for AFEC_CVR {}
#[doc = "`write(|w| ..)` method takes [afec_cvr::W](afec_cvr::W) writer structure"]
impl crate::Writable for AFEC_CVR {}
#[doc = "AFEC Correction Values Register"]
pub mod afec_cvr;
#[doc = "AFEC Channel Error Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_cecr](afec_cecr) module"]
pub type AFEC_CECR = crate::Reg<u32, _AFEC_CECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_CECR;
#[doc = "`read()` method returns [afec_cecr::R](afec_cecr::R) reader structure"]
impl crate::Readable for AFEC_CECR {}
#[doc = "`write(|w| ..)` method takes [afec_cecr::W](afec_cecr::W) writer structure"]
impl crate::Writable for AFEC_CECR {}
#[doc = "AFEC Channel Error Correction Register"]
pub mod afec_cecr;
#[doc = "AFEC Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_wpmr](afec_wpmr) module"]
pub type AFEC_WPMR = crate::Reg<u32, _AFEC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_WPMR;
#[doc = "`read()` method returns [afec_wpmr::R](afec_wpmr::R) reader structure"]
impl crate::Readable for AFEC_WPMR {}
#[doc = "`write(|w| ..)` method takes [afec_wpmr::W](afec_wpmr::W) writer structure"]
impl crate::Writable for AFEC_WPMR {}
#[doc = "AFEC Write Protection Mode Register"]
pub mod afec_wpmr;
#[doc = "AFEC Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afec_wpsr](afec_wpsr) module"]
pub type AFEC_WPSR = crate::Reg<u32, _AFEC_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFEC_WPSR;
#[doc = "`read()` method returns [afec_wpsr::R](afec_wpsr::R) reader structure"]
impl crate::Readable for AFEC_WPSR {}
#[doc = "AFEC Write Protection Status Register"]
pub mod afec_wpsr;
