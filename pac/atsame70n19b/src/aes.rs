#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub aes_cr: AES_CR,
    #[doc = "0x04 - Mode Register"]
    pub aes_mr: AES_MR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub aes_ier: AES_IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub aes_idr: AES_IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub aes_imr: AES_IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub aes_isr: AES_ISR,
    #[doc = "0x20 - Key Word Register"]
    pub aes_keywr: [AES_KEYWR; 8],
    #[doc = "0x40 - Input Data Register"]
    pub aes_idatar: [AES_IDATAR; 4],
    #[doc = "0x50 - Output Data Register"]
    pub aes_odatar: [AES_ODATAR; 4],
    #[doc = "0x60 - Initialization Vector Register"]
    pub aes_ivr: [AES_IVR; 4],
    #[doc = "0x70 - Additional Authenticated Data Length Register"]
    pub aes_aadlenr: AES_AADLENR,
    #[doc = "0x74 - Plaintext/Ciphertext Length Register"]
    pub aes_clenr: AES_CLENR,
    #[doc = "0x78 - GCM Intermediate Hash Word Register"]
    pub aes_ghashr: [AES_GHASHR; 4],
    #[doc = "0x88 - GCM Authentication Tag Word Register"]
    pub aes_tagr: [AES_TAGR; 4],
    #[doc = "0x98 - GCM Encryption Counter Value Register"]
    pub aes_ctrr: AES_CTRR,
    #[doc = "0x9c - GCM H Word Register"]
    pub aes_gcmhr: [AES_GCMHR; 4],
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_cr](aes_cr) module"]
pub type AES_CR = crate::Reg<u32, _AES_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_CR;
#[doc = "`write(|w| ..)` method takes [aes_cr::W](aes_cr::W) writer structure"]
impl crate::Writable for AES_CR {}
#[doc = "Control Register"]
pub mod aes_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_mr](aes_mr) module"]
pub type AES_MR = crate::Reg<u32, _AES_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_MR;
#[doc = "`read()` method returns [aes_mr::R](aes_mr::R) reader structure"]
impl crate::Readable for AES_MR {}
#[doc = "`write(|w| ..)` method takes [aes_mr::W](aes_mr::W) writer structure"]
impl crate::Writable for AES_MR {}
#[doc = "Mode Register"]
pub mod aes_mr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ier](aes_ier) module"]
pub type AES_IER = crate::Reg<u32, _AES_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IER;
#[doc = "`write(|w| ..)` method takes [aes_ier::W](aes_ier::W) writer structure"]
impl crate::Writable for AES_IER {}
#[doc = "Interrupt Enable Register"]
pub mod aes_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_idr](aes_idr) module"]
pub type AES_IDR = crate::Reg<u32, _AES_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IDR;
#[doc = "`write(|w| ..)` method takes [aes_idr::W](aes_idr::W) writer structure"]
impl crate::Writable for AES_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod aes_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_imr](aes_imr) module"]
pub type AES_IMR = crate::Reg<u32, _AES_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IMR;
#[doc = "`read()` method returns [aes_imr::R](aes_imr::R) reader structure"]
impl crate::Readable for AES_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod aes_imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_isr](aes_isr) module"]
pub type AES_ISR = crate::Reg<u32, _AES_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_ISR;
#[doc = "`read()` method returns [aes_isr::R](aes_isr::R) reader structure"]
impl crate::Readable for AES_ISR {}
#[doc = "Interrupt Status Register"]
pub mod aes_isr;
#[doc = "Key Word Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_keywr](aes_keywr) module"]
pub type AES_KEYWR = crate::Reg<u32, _AES_KEYWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_KEYWR;
#[doc = "`write(|w| ..)` method takes [aes_keywr::W](aes_keywr::W) writer structure"]
impl crate::Writable for AES_KEYWR {}
#[doc = "Key Word Register"]
pub mod aes_keywr;
#[doc = "Input Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_idatar](aes_idatar) module"]
pub type AES_IDATAR = crate::Reg<u32, _AES_IDATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IDATAR;
#[doc = "`write(|w| ..)` method takes [aes_idatar::W](aes_idatar::W) writer structure"]
impl crate::Writable for AES_IDATAR {}
#[doc = "Input Data Register"]
pub mod aes_idatar;
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_odatar](aes_odatar) module"]
pub type AES_ODATAR = crate::Reg<u32, _AES_ODATAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_ODATAR;
#[doc = "`read()` method returns [aes_odatar::R](aes_odatar::R) reader structure"]
impl crate::Readable for AES_ODATAR {}
#[doc = "Output Data Register"]
pub mod aes_odatar;
#[doc = "Initialization Vector Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ivr](aes_ivr) module"]
pub type AES_IVR = crate::Reg<u32, _AES_IVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_IVR;
#[doc = "`write(|w| ..)` method takes [aes_ivr::W](aes_ivr::W) writer structure"]
impl crate::Writable for AES_IVR {}
#[doc = "Initialization Vector Register"]
pub mod aes_ivr;
#[doc = "Additional Authenticated Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aadlenr](aes_aadlenr) module"]
pub type AES_AADLENR = crate::Reg<u32, _AES_AADLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_AADLENR;
#[doc = "`read()` method returns [aes_aadlenr::R](aes_aadlenr::R) reader structure"]
impl crate::Readable for AES_AADLENR {}
#[doc = "`write(|w| ..)` method takes [aes_aadlenr::W](aes_aadlenr::W) writer structure"]
impl crate::Writable for AES_AADLENR {}
#[doc = "Additional Authenticated Data Length Register"]
pub mod aes_aadlenr;
#[doc = "Plaintext/Ciphertext Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_clenr](aes_clenr) module"]
pub type AES_CLENR = crate::Reg<u32, _AES_CLENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_CLENR;
#[doc = "`read()` method returns [aes_clenr::R](aes_clenr::R) reader structure"]
impl crate::Readable for AES_CLENR {}
#[doc = "`write(|w| ..)` method takes [aes_clenr::W](aes_clenr::W) writer structure"]
impl crate::Writable for AES_CLENR {}
#[doc = "Plaintext/Ciphertext Length Register"]
pub mod aes_clenr;
#[doc = "GCM Intermediate Hash Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ghashr](aes_ghashr) module"]
pub type AES_GHASHR = crate::Reg<u32, _AES_GHASHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_GHASHR;
#[doc = "`read()` method returns [aes_ghashr::R](aes_ghashr::R) reader structure"]
impl crate::Readable for AES_GHASHR {}
#[doc = "`write(|w| ..)` method takes [aes_ghashr::W](aes_ghashr::W) writer structure"]
impl crate::Writable for AES_GHASHR {}
#[doc = "GCM Intermediate Hash Word Register"]
pub mod aes_ghashr;
#[doc = "GCM Authentication Tag Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_tagr](aes_tagr) module"]
pub type AES_TAGR = crate::Reg<u32, _AES_TAGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_TAGR;
#[doc = "`read()` method returns [aes_tagr::R](aes_tagr::R) reader structure"]
impl crate::Readable for AES_TAGR {}
#[doc = "GCM Authentication Tag Word Register"]
pub mod aes_tagr;
#[doc = "GCM Encryption Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ctrr](aes_ctrr) module"]
pub type AES_CTRR = crate::Reg<u32, _AES_CTRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_CTRR;
#[doc = "`read()` method returns [aes_ctrr::R](aes_ctrr::R) reader structure"]
impl crate::Readable for AES_CTRR {}
#[doc = "GCM Encryption Counter Value Register"]
pub mod aes_ctrr;
#[doc = "GCM H Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_gcmhr](aes_gcmhr) module"]
pub type AES_GCMHR = crate::Reg<u32, _AES_GCMHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AES_GCMHR;
#[doc = "`read()` method returns [aes_gcmhr::R](aes_gcmhr::R) reader structure"]
impl crate::Readable for AES_GCMHR {}
#[doc = "`write(|w| ..)` method takes [aes_gcmhr::W](aes_gcmhr::W) writer structure"]
impl crate::Writable for AES_GCMHR {}
#[doc = "GCM H Word Register"]
pub mod aes_gcmhr;
