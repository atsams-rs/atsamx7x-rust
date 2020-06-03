#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstdmanxtdsc](usbhs_hstdmanxtdsc) module"]
pub type USBHS_HSTDMANXTDSC = crate::Reg<u32, _USBHS_HSTDMANXTDSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTDMANXTDSC;
#[doc = "`read()` method returns [usbhs_hstdmanxtdsc::R](usbhs_hstdmanxtdsc::R) reader structure"]
impl crate::Readable for USBHS_HSTDMANXTDSC {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstdmanxtdsc::W](usbhs_hstdmanxtdsc::W) writer structure"]
impl crate::Writable for USBHS_HSTDMANXTDSC {}
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_hstdmanxtdsc;
#[doc = "Host DMA Channel Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstdmaaddress](usbhs_hstdmaaddress) module"]
pub type USBHS_HSTDMAADDRESS = crate::Reg<u32, _USBHS_HSTDMAADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTDMAADDRESS;
#[doc = "`read()` method returns [usbhs_hstdmaaddress::R](usbhs_hstdmaaddress::R) reader structure"]
impl crate::Readable for USBHS_HSTDMAADDRESS {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstdmaaddress::W](usbhs_hstdmaaddress::W) writer structure"]
impl crate::Writable for USBHS_HSTDMAADDRESS {}
#[doc = "Host DMA Channel Address Register (n = 1)"]
pub mod usbhs_hstdmaaddress;
#[doc = "Host DMA Channel Control Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstdmacontrol](usbhs_hstdmacontrol) module"]
pub type USBHS_HSTDMACONTROL = crate::Reg<u32, _USBHS_HSTDMACONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTDMACONTROL;
#[doc = "`read()` method returns [usbhs_hstdmacontrol::R](usbhs_hstdmacontrol::R) reader structure"]
impl crate::Readable for USBHS_HSTDMACONTROL {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstdmacontrol::W](usbhs_hstdmacontrol::W) writer structure"]
impl crate::Writable for USBHS_HSTDMACONTROL {}
#[doc = "Host DMA Channel Control Register (n = 1)"]
pub mod usbhs_hstdmacontrol;
#[doc = "Host DMA Channel Status Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstdmastatus](usbhs_hstdmastatus) module"]
pub type USBHS_HSTDMASTATUS = crate::Reg<u32, _USBHS_HSTDMASTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTDMASTATUS;
#[doc = "`read()` method returns [usbhs_hstdmastatus::R](usbhs_hstdmastatus::R) reader structure"]
impl crate::Readable for USBHS_HSTDMASTATUS {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstdmastatus::W](usbhs_hstdmastatus::W) writer structure"]
impl crate::Writable for USBHS_HSTDMASTATUS {}
#[doc = "Host DMA Channel Status Register (n = 1)"]
pub mod usbhs_hstdmastatus;
