#[doc = "Channel Interrupt Enable Register (chid = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cie](xdmac_cie) module"]
pub type XDMAC_CIE = crate::Reg<u32, _XDMAC_CIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CIE;
#[doc = "`write(|w| ..)` method takes [xdmac_cie::W](xdmac_cie::W) writer structure"]
impl crate::Writable for XDMAC_CIE {}
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod xdmac_cie;
#[doc = "Channel Interrupt Disable Register (chid = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cid](xdmac_cid) module"]
pub type XDMAC_CID = crate::Reg<u32, _XDMAC_CID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CID;
#[doc = "`write(|w| ..)` method takes [xdmac_cid::W](xdmac_cid::W) writer structure"]
impl crate::Writable for XDMAC_CID {}
#[doc = "Channel Interrupt Disable Register (chid = 0)"]
pub mod xdmac_cid;
#[doc = "Channel Interrupt Mask Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cim](xdmac_cim) module"]
pub type XDMAC_CIM = crate::Reg<u32, _XDMAC_CIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CIM;
#[doc = "`read()` method returns [xdmac_cim::R](xdmac_cim::R) reader structure"]
impl crate::Readable for XDMAC_CIM {}
#[doc = "Channel Interrupt Mask Register (chid = 0)"]
pub mod xdmac_cim;
#[doc = "Channel Interrupt Status Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cis](xdmac_cis) module"]
pub type XDMAC_CIS = crate::Reg<u32, _XDMAC_CIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CIS;
#[doc = "`read()` method returns [xdmac_cis::R](xdmac_cis::R) reader structure"]
impl crate::Readable for XDMAC_CIS {}
#[doc = "Channel Interrupt Status Register (chid = 0)"]
pub mod xdmac_cis;
#[doc = "Channel Source Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_csa](xdmac_csa) module"]
pub type XDMAC_CSA = crate::Reg<u32, _XDMAC_CSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CSA;
#[doc = "`read()` method returns [xdmac_csa::R](xdmac_csa::R) reader structure"]
impl crate::Readable for XDMAC_CSA {}
#[doc = "`write(|w| ..)` method takes [xdmac_csa::W](xdmac_csa::W) writer structure"]
impl crate::Writable for XDMAC_CSA {}
#[doc = "Channel Source Address Register (chid = 0)"]
pub mod xdmac_csa;
#[doc = "Channel Destination Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cda](xdmac_cda) module"]
pub type XDMAC_CDA = crate::Reg<u32, _XDMAC_CDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CDA;
#[doc = "`read()` method returns [xdmac_cda::R](xdmac_cda::R) reader structure"]
impl crate::Readable for XDMAC_CDA {}
#[doc = "`write(|w| ..)` method takes [xdmac_cda::W](xdmac_cda::W) writer structure"]
impl crate::Writable for XDMAC_CDA {}
#[doc = "Channel Destination Address Register (chid = 0)"]
pub mod xdmac_cda;
#[doc = "Channel Next Descriptor Address Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cnda](xdmac_cnda) module"]
pub type XDMAC_CNDA = crate::Reg<u32, _XDMAC_CNDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CNDA;
#[doc = "`read()` method returns [xdmac_cnda::R](xdmac_cnda::R) reader structure"]
impl crate::Readable for XDMAC_CNDA {}
#[doc = "`write(|w| ..)` method takes [xdmac_cnda::W](xdmac_cnda::W) writer structure"]
impl crate::Writable for XDMAC_CNDA {}
#[doc = "Channel Next Descriptor Address Register (chid = 0)"]
pub mod xdmac_cnda;
#[doc = "Channel Next Descriptor Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cndc](xdmac_cndc) module"]
pub type XDMAC_CNDC = crate::Reg<u32, _XDMAC_CNDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CNDC;
#[doc = "`read()` method returns [xdmac_cndc::R](xdmac_cndc::R) reader structure"]
impl crate::Readable for XDMAC_CNDC {}
#[doc = "`write(|w| ..)` method takes [xdmac_cndc::W](xdmac_cndc::W) writer structure"]
impl crate::Writable for XDMAC_CNDC {}
#[doc = "Channel Next Descriptor Control Register (chid = 0)"]
pub mod xdmac_cndc;
#[doc = "Channel Microblock Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cubc](xdmac_cubc) module"]
pub type XDMAC_CUBC = crate::Reg<u32, _XDMAC_CUBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CUBC;
#[doc = "`read()` method returns [xdmac_cubc::R](xdmac_cubc::R) reader structure"]
impl crate::Readable for XDMAC_CUBC {}
#[doc = "`write(|w| ..)` method takes [xdmac_cubc::W](xdmac_cubc::W) writer structure"]
impl crate::Writable for XDMAC_CUBC {}
#[doc = "Channel Microblock Control Register (chid = 0)"]
pub mod xdmac_cubc;
#[doc = "Channel Block Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cbc](xdmac_cbc) module"]
pub type XDMAC_CBC = crate::Reg<u32, _XDMAC_CBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CBC;
#[doc = "`read()` method returns [xdmac_cbc::R](xdmac_cbc::R) reader structure"]
impl crate::Readable for XDMAC_CBC {}
#[doc = "`write(|w| ..)` method takes [xdmac_cbc::W](xdmac_cbc::W) writer structure"]
impl crate::Writable for XDMAC_CBC {}
#[doc = "Channel Block Control Register (chid = 0)"]
pub mod xdmac_cbc;
#[doc = "Channel Configuration Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cc](xdmac_cc) module"]
pub type XDMAC_CC = crate::Reg<u32, _XDMAC_CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CC;
#[doc = "`read()` method returns [xdmac_cc::R](xdmac_cc::R) reader structure"]
impl crate::Readable for XDMAC_CC {}
#[doc = "`write(|w| ..)` method takes [xdmac_cc::W](xdmac_cc::W) writer structure"]
impl crate::Writable for XDMAC_CC {}
#[doc = "Channel Configuration Register (chid = 0)"]
pub mod xdmac_cc;
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cds_msp](xdmac_cds_msp) module"]
pub type XDMAC_CDS_MSP = crate::Reg<u32, _XDMAC_CDS_MSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CDS_MSP;
#[doc = "`read()` method returns [xdmac_cds_msp::R](xdmac_cds_msp::R) reader structure"]
impl crate::Readable for XDMAC_CDS_MSP {}
#[doc = "`write(|w| ..)` method takes [xdmac_cds_msp::W](xdmac_cds_msp::W) writer structure"]
impl crate::Writable for XDMAC_CDS_MSP {}
#[doc = "Channel Data Stride Memory Set Pattern (chid = 0)"]
pub mod xdmac_cds_msp;
#[doc = "Channel Source Microblock Stride (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_csus](xdmac_csus) module"]
pub type XDMAC_CSUS = crate::Reg<u32, _XDMAC_CSUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CSUS;
#[doc = "`read()` method returns [xdmac_csus::R](xdmac_csus::R) reader structure"]
impl crate::Readable for XDMAC_CSUS {}
#[doc = "`write(|w| ..)` method takes [xdmac_csus::W](xdmac_csus::W) writer structure"]
impl crate::Writable for XDMAC_CSUS {}
#[doc = "Channel Source Microblock Stride (chid = 0)"]
pub mod xdmac_csus;
#[doc = "Channel Destination Microblock Stride (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_cdus](xdmac_cdus) module"]
pub type XDMAC_CDUS = crate::Reg<u32, _XDMAC_CDUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_CDUS;
#[doc = "`read()` method returns [xdmac_cdus::R](xdmac_cdus::R) reader structure"]
impl crate::Readable for XDMAC_CDUS {}
#[doc = "`write(|w| ..)` method takes [xdmac_cdus::W](xdmac_cdus::W) writer structure"]
impl crate::Writable for XDMAC_CDUS {}
#[doc = "Channel Destination Microblock Stride (chid = 0)"]
pub mod xdmac_cdus;
