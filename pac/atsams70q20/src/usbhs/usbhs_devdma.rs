#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devdmanxtdsc](usbhs_devdmanxtdsc) module"]
pub type USBHS_DEVDMANXTDSC = crate::Reg<u32, _USBHS_DEVDMANXTDSC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVDMANXTDSC;
#[doc = "`read()` method returns [usbhs_devdmanxtdsc::R](usbhs_devdmanxtdsc::R) reader structure"]
impl crate::Readable for USBHS_DEVDMANXTDSC {}
#[doc = "`write(|w| ..)` method takes [usbhs_devdmanxtdsc::W](usbhs_devdmanxtdsc::W) writer structure"]
impl crate::Writable for USBHS_DEVDMANXTDSC {}
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_devdmanxtdsc;
#[doc = "Device DMA Channel Address Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devdmaaddress](usbhs_devdmaaddress) module"]
pub type USBHS_DEVDMAADDRESS = crate::Reg<u32, _USBHS_DEVDMAADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVDMAADDRESS;
#[doc = "`read()` method returns [usbhs_devdmaaddress::R](usbhs_devdmaaddress::R) reader structure"]
impl crate::Readable for USBHS_DEVDMAADDRESS {}
#[doc = "`write(|w| ..)` method takes [usbhs_devdmaaddress::W](usbhs_devdmaaddress::W) writer structure"]
impl crate::Writable for USBHS_DEVDMAADDRESS {}
#[doc = "Device DMA Channel Address Register (n = 1)"]
pub mod usbhs_devdmaaddress;
#[doc = "Device DMA Channel Control Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devdmacontrol](usbhs_devdmacontrol) module"]
pub type USBHS_DEVDMACONTROL = crate::Reg<u32, _USBHS_DEVDMACONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVDMACONTROL;
#[doc = "`read()` method returns [usbhs_devdmacontrol::R](usbhs_devdmacontrol::R) reader structure"]
impl crate::Readable for USBHS_DEVDMACONTROL {}
#[doc = "`write(|w| ..)` method takes [usbhs_devdmacontrol::W](usbhs_devdmacontrol::W) writer structure"]
impl crate::Writable for USBHS_DEVDMACONTROL {}
#[doc = "Device DMA Channel Control Register (n = 1)"]
pub mod usbhs_devdmacontrol;
#[doc = "Device DMA Channel Status Register (n = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devdmastatus](usbhs_devdmastatus) module"]
pub type USBHS_DEVDMASTATUS = crate::Reg<u32, _USBHS_DEVDMASTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVDMASTATUS;
#[doc = "`read()` method returns [usbhs_devdmastatus::R](usbhs_devdmastatus::R) reader structure"]
impl crate::Readable for USBHS_DEVDMASTATUS {}
#[doc = "`write(|w| ..)` method takes [usbhs_devdmastatus::W](usbhs_devdmastatus::W) writer structure"]
impl crate::Writable for USBHS_DEVDMASTATUS {}
#[doc = "Device DMA Channel Status Register (n = 1)"]
pub mod usbhs_devdmastatus;
