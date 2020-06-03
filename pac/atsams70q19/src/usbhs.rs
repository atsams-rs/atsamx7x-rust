#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub usbhs_devctrl: USBHS_DEVCTRL,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub usbhs_devisr: USBHS_DEVISR,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub usbhs_devicr: USBHS_DEVICR,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub usbhs_devifr: USBHS_DEVIFR,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub usbhs_devimr: USBHS_DEVIMR,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub usbhs_devidr: USBHS_DEVIDR,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub usbhs_devier: USBHS_DEVIER,
    #[doc = "0x1c - Device Endpoint Register"]
    pub usbhs_devept: USBHS_DEVEPT,
    #[doc = "0x20 - Device Frame Number Register"]
    pub usbhs_devfnum: USBHS_DEVFNUM,
    _reserved9: [u8; 220usize],
    #[doc = "0x100 - Device Endpoint Configuration Register (n = 0) 0"]
    pub usbhs_deveptcfg: [USBHS_DEVEPTCFG; 10],
    _reserved10: [u8; 8usize],
    #[doc = "0x130 - Device Endpoint Status Register (n = 0) 0"]
    pub usbhs_deveptisr: [USBHS_DEVEPTISR; 10],
    _reserved11: [u8; 8usize],
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0) 0"]
    pub usbhs_devepticr: [USBHS_DEVEPTICR; 10],
    _reserved12: [u8; 8usize],
    #[doc = "0x190 - Device Endpoint Set Register (n = 0) 0"]
    pub usbhs_deveptifr: [USBHS_DEVEPTIFR; 10],
    _reserved13: [u8; 8usize],
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0) 0"]
    pub usbhs_deveptimr: [USBHS_DEVEPTIMR; 10],
    _reserved14: [u8; 8usize],
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0) 0"]
    pub usbhs_deveptier: [USBHS_DEVEPTIER; 10],
    _reserved15: [u8; 8usize],
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0) 0"]
    pub usbhs_deveptidr: [USBHS_DEVEPTIDR; 10],
    _reserved16: [u8; 200usize],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 128usize],
    #[doc = "0x400 - Host General Control Register"]
    pub usbhs_hstctrl: USBHS_HSTCTRL,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub usbhs_hstisr: USBHS_HSTISR,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub usbhs_hsticr: USBHS_HSTICR,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub usbhs_hstifr: USBHS_HSTIFR,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub usbhs_hstimr: USBHS_HSTIMR,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub usbhs_hstidr: USBHS_HSTIDR,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub usbhs_hstier: USBHS_HSTIER,
    #[doc = "0x41c - Host Pipe Register"]
    pub usbhs_hstpip: USBHS_HSTPIP,
    #[doc = "0x420 - Host Frame Number Register"]
    pub usbhs_hstfnum: USBHS_HSTFNUM,
    #[doc = "0x424 - Host Address 1 Register"]
    pub usbhs_hstaddr1: USBHS_HSTADDR1,
    #[doc = "0x428 - Host Address 2 Register"]
    pub usbhs_hstaddr2: USBHS_HSTADDR2,
    #[doc = "0x42c - Host Address 3 Register"]
    pub usbhs_hstaddr3: USBHS_HSTADDR3,
    _reserved29: [u8; 208usize],
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0) 0"]
    pub usbhs_hstpipcfg: [USBHS_HSTPIPCFG; 10],
    _reserved30: [u8; 8usize],
    #[doc = "0x530 - Host Pipe Status Register (n = 0) 0"]
    pub usbhs_hstpipisr: [USBHS_HSTPIPISR; 10],
    _reserved31: [u8; 8usize],
    #[doc = "0x560 - Host Pipe Clear Register (n = 0) 0"]
    pub usbhs_hstpipicr: [USBHS_HSTPIPICR; 10],
    _reserved32: [u8; 8usize],
    #[doc = "0x590 - Host Pipe Set Register (n = 0) 0"]
    pub usbhs_hstpipifr: [USBHS_HSTPIPIFR; 10],
    _reserved33: [u8; 8usize],
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0) 0"]
    pub usbhs_hstpipimr: [USBHS_HSTPIPIMR; 10],
    _reserved34: [u8; 8usize],
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0) 0"]
    pub usbhs_hstpipier: [USBHS_HSTPIPIER; 10],
    _reserved35: [u8; 8usize],
    #[doc = "0x620 - Host Pipe Disable Register (n = 0) 0"]
    pub usbhs_hstpipidr: [USBHS_HSTPIPIDR; 10],
    _reserved36: [u8; 8usize],
    #[doc = "0x650 - Host Pipe IN Request Register (n = 0) 0"]
    pub usbhs_hstpipinrq: [USBHS_HSTPIPINRQ; 10],
    _reserved37: [u8; 8usize],
    #[doc = "0x680 - Host Pipe Error Register (n = 0) 0"]
    pub usbhs_hstpiperr: [USBHS_HSTPIPERR; 10],
    _reserved38: [u8; 104usize],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 128usize],
    #[doc = "0x800 - General Control Register"]
    pub usbhs_ctrl: USBHS_CTRL,
    #[doc = "0x804 - General Status Register"]
    pub usbhs_sr: USBHS_SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbhs_scr: USBHS_SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub usbhs_sfr: USBHS_SFR,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdmanxtdsc: self::usbhs_devdma::USBHS_DEVDMANXTDSC,
    #[doc = "0x04 - Device DMA Channel Address Register (n = 1)"]
    pub usbhs_devdmaaddress: self::usbhs_devdma::USBHS_DEVDMAADDRESS,
    #[doc = "0x08 - Device DMA Channel Control Register (n = 1)"]
    pub usbhs_devdmacontrol: self::usbhs_devdma::USBHS_DEVDMACONTROL,
    #[doc = "0x0c - Device DMA Channel Status Register (n = 1)"]
    pub usbhs_devdmastatus: self::usbhs_devdma::USBHS_DEVDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdmanxtdsc: self::usbhs_hstdma::USBHS_HSTDMANXTDSC,
    #[doc = "0x04 - Host DMA Channel Address Register (n = 1)"]
    pub usbhs_hstdmaaddress: self::usbhs_hstdma::USBHS_HSTDMAADDRESS,
    #[doc = "0x08 - Host DMA Channel Control Register (n = 1)"]
    pub usbhs_hstdmacontrol: self::usbhs_hstdma::USBHS_HSTDMACONTROL,
    #[doc = "0x0c - Host DMA Channel Status Register (n = 1)"]
    pub usbhs_hstdmastatus: self::usbhs_hstdma::USBHS_HSTDMASTATUS,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_hstdma;
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devctrl](usbhs_devctrl) module"]
pub type USBHS_DEVCTRL = crate::Reg<u32, _USBHS_DEVCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVCTRL;
#[doc = "`read()` method returns [usbhs_devctrl::R](usbhs_devctrl::R) reader structure"]
impl crate::Readable for USBHS_DEVCTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_devctrl::W](usbhs_devctrl::W) writer structure"]
impl crate::Writable for USBHS_DEVCTRL {}
#[doc = "Device General Control Register"]
pub mod usbhs_devctrl;
#[doc = "Device Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devisr](usbhs_devisr) module"]
pub type USBHS_DEVISR = crate::Reg<u32, _USBHS_DEVISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVISR;
#[doc = "`read()` method returns [usbhs_devisr::R](usbhs_devisr::R) reader structure"]
impl crate::Readable for USBHS_DEVISR {}
#[doc = "Device Global Interrupt Status Register"]
pub mod usbhs_devisr;
#[doc = "Device Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devicr](usbhs_devicr) module"]
pub type USBHS_DEVICR = crate::Reg<u32, _USBHS_DEVICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVICR;
#[doc = "`write(|w| ..)` method takes [usbhs_devicr::W](usbhs_devicr::W) writer structure"]
impl crate::Writable for USBHS_DEVICR {}
#[doc = "Device Global Interrupt Clear Register"]
pub mod usbhs_devicr;
#[doc = "Device Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devifr](usbhs_devifr) module"]
pub type USBHS_DEVIFR = crate::Reg<u32, _USBHS_DEVIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIFR;
#[doc = "`write(|w| ..)` method takes [usbhs_devifr::W](usbhs_devifr::W) writer structure"]
impl crate::Writable for USBHS_DEVIFR {}
#[doc = "Device Global Interrupt Set Register"]
pub mod usbhs_devifr;
#[doc = "Device Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devimr](usbhs_devimr) module"]
pub type USBHS_DEVIMR = crate::Reg<u32, _USBHS_DEVIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIMR;
#[doc = "`read()` method returns [usbhs_devimr::R](usbhs_devimr::R) reader structure"]
impl crate::Readable for USBHS_DEVIMR {}
#[doc = "Device Global Interrupt Mask Register"]
pub mod usbhs_devimr;
#[doc = "Device Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devidr](usbhs_devidr) module"]
pub type USBHS_DEVIDR = crate::Reg<u32, _USBHS_DEVIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIDR;
#[doc = "`write(|w| ..)` method takes [usbhs_devidr::W](usbhs_devidr::W) writer structure"]
impl crate::Writable for USBHS_DEVIDR {}
#[doc = "Device Global Interrupt Disable Register"]
pub mod usbhs_devidr;
#[doc = "Device Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devier](usbhs_devier) module"]
pub type USBHS_DEVIER = crate::Reg<u32, _USBHS_DEVIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVIER;
#[doc = "`write(|w| ..)` method takes [usbhs_devier::W](usbhs_devier::W) writer structure"]
impl crate::Writable for USBHS_DEVIER {}
#[doc = "Device Global Interrupt Enable Register"]
pub mod usbhs_devier;
#[doc = "Device Endpoint Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devept](usbhs_devept) module"]
pub type USBHS_DEVEPT = crate::Reg<u32, _USBHS_DEVEPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPT;
#[doc = "`read()` method returns [usbhs_devept::R](usbhs_devept::R) reader structure"]
impl crate::Readable for USBHS_DEVEPT {}
#[doc = "`write(|w| ..)` method takes [usbhs_devept::W](usbhs_devept::W) writer structure"]
impl crate::Writable for USBHS_DEVEPT {}
#[doc = "Device Endpoint Register"]
pub mod usbhs_devept;
#[doc = "Device Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devfnum](usbhs_devfnum) module"]
pub type USBHS_DEVFNUM = crate::Reg<u32, _USBHS_DEVFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVFNUM;
#[doc = "`read()` method returns [usbhs_devfnum::R](usbhs_devfnum::R) reader structure"]
impl crate::Readable for USBHS_DEVFNUM {}
#[doc = "Device Frame Number Register"]
pub mod usbhs_devfnum;
#[doc = "Device Endpoint Configuration Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptcfg](usbhs_deveptcfg) module"]
pub type USBHS_DEVEPTCFG = crate::Reg<u32, _USBHS_DEVEPTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTCFG;
#[doc = "`read()` method returns [usbhs_deveptcfg::R](usbhs_deveptcfg::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTCFG {}
#[doc = "`write(|w| ..)` method takes [usbhs_deveptcfg::W](usbhs_deveptcfg::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTCFG {}
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod usbhs_deveptcfg;
#[doc = "Device Endpoint Status Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptisr](usbhs_deveptisr) module"]
pub type USBHS_DEVEPTISR = crate::Reg<u32, _USBHS_DEVEPTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTISR;
#[doc = "`read()` method returns [usbhs_deveptisr::R](usbhs_deveptisr::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTISR {}
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod usbhs_deveptisr;
#[doc = "Device Endpoint Clear Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_devepticr](usbhs_devepticr) module"]
pub type USBHS_DEVEPTICR = crate::Reg<u32, _USBHS_DEVEPTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTICR;
#[doc = "`write(|w| ..)` method takes [usbhs_devepticr::W](usbhs_devepticr::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTICR {}
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod usbhs_devepticr;
#[doc = "Device Endpoint Set Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptifr](usbhs_deveptifr) module"]
pub type USBHS_DEVEPTIFR = crate::Reg<u32, _USBHS_DEVEPTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIFR;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptifr::W](usbhs_deveptifr::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIFR {}
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod usbhs_deveptifr;
#[doc = "Device Endpoint Mask Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptimr](usbhs_deveptimr) module"]
pub type USBHS_DEVEPTIMR = crate::Reg<u32, _USBHS_DEVEPTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIMR;
#[doc = "`read()` method returns [usbhs_deveptimr::R](usbhs_deveptimr::R) reader structure"]
impl crate::Readable for USBHS_DEVEPTIMR {}
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod usbhs_deveptimr;
#[doc = "Device Endpoint Enable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptier](usbhs_deveptier) module"]
pub type USBHS_DEVEPTIER = crate::Reg<u32, _USBHS_DEVEPTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIER;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptier::W](usbhs_deveptier::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIER {}
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod usbhs_deveptier;
#[doc = "Device Endpoint Disable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_deveptidr](usbhs_deveptidr) module"]
pub type USBHS_DEVEPTIDR = crate::Reg<u32, _USBHS_DEVEPTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_DEVEPTIDR;
#[doc = "`write(|w| ..)` method takes [usbhs_deveptidr::W](usbhs_deveptidr::W) writer structure"]
impl crate::Writable for USBHS_DEVEPTIDR {}
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod usbhs_deveptidr;
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstctrl](usbhs_hstctrl) module"]
pub type USBHS_HSTCTRL = crate::Reg<u32, _USBHS_HSTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTCTRL;
#[doc = "`read()` method returns [usbhs_hstctrl::R](usbhs_hstctrl::R) reader structure"]
impl crate::Readable for USBHS_HSTCTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstctrl::W](usbhs_hstctrl::W) writer structure"]
impl crate::Writable for USBHS_HSTCTRL {}
#[doc = "Host General Control Register"]
pub mod usbhs_hstctrl;
#[doc = "Host Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstisr](usbhs_hstisr) module"]
pub type USBHS_HSTISR = crate::Reg<u32, _USBHS_HSTISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTISR;
#[doc = "`read()` method returns [usbhs_hstisr::R](usbhs_hstisr::R) reader structure"]
impl crate::Readable for USBHS_HSTISR {}
#[doc = "Host Global Interrupt Status Register"]
pub mod usbhs_hstisr;
#[doc = "Host Global Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hsticr](usbhs_hsticr) module"]
pub type USBHS_HSTICR = crate::Reg<u32, _USBHS_HSTICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTICR;
#[doc = "`write(|w| ..)` method takes [usbhs_hsticr::W](usbhs_hsticr::W) writer structure"]
impl crate::Writable for USBHS_HSTICR {}
#[doc = "Host Global Interrupt Clear Register"]
pub mod usbhs_hsticr;
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstifr](usbhs_hstifr) module"]
pub type USBHS_HSTIFR = crate::Reg<u32, _USBHS_HSTIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIFR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstifr::W](usbhs_hstifr::W) writer structure"]
impl crate::Writable for USBHS_HSTIFR {}
#[doc = "Host Global Interrupt Set Register"]
pub mod usbhs_hstifr;
#[doc = "Host Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstimr](usbhs_hstimr) module"]
pub type USBHS_HSTIMR = crate::Reg<u32, _USBHS_HSTIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIMR;
#[doc = "`read()` method returns [usbhs_hstimr::R](usbhs_hstimr::R) reader structure"]
impl crate::Readable for USBHS_HSTIMR {}
#[doc = "Host Global Interrupt Mask Register"]
pub mod usbhs_hstimr;
#[doc = "Host Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstidr](usbhs_hstidr) module"]
pub type USBHS_HSTIDR = crate::Reg<u32, _USBHS_HSTIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIDR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstidr::W](usbhs_hstidr::W) writer structure"]
impl crate::Writable for USBHS_HSTIDR {}
#[doc = "Host Global Interrupt Disable Register"]
pub mod usbhs_hstidr;
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstier](usbhs_hstier) module"]
pub type USBHS_HSTIER = crate::Reg<u32, _USBHS_HSTIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTIER;
#[doc = "`write(|w| ..)` method takes [usbhs_hstier::W](usbhs_hstier::W) writer structure"]
impl crate::Writable for USBHS_HSTIER {}
#[doc = "Host Global Interrupt Enable Register"]
pub mod usbhs_hstier;
#[doc = "Host Pipe Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpip](usbhs_hstpip) module"]
pub type USBHS_HSTPIP = crate::Reg<u32, _USBHS_HSTPIP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIP;
#[doc = "`read()` method returns [usbhs_hstpip::R](usbhs_hstpip::R) reader structure"]
impl crate::Readable for USBHS_HSTPIP {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpip::W](usbhs_hstpip::W) writer structure"]
impl crate::Writable for USBHS_HSTPIP {}
#[doc = "Host Pipe Register"]
pub mod usbhs_hstpip;
#[doc = "Host Frame Number Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstfnum](usbhs_hstfnum) module"]
pub type USBHS_HSTFNUM = crate::Reg<u32, _USBHS_HSTFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTFNUM;
#[doc = "`read()` method returns [usbhs_hstfnum::R](usbhs_hstfnum::R) reader structure"]
impl crate::Readable for USBHS_HSTFNUM {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstfnum::W](usbhs_hstfnum::W) writer structure"]
impl crate::Writable for USBHS_HSTFNUM {}
#[doc = "Host Frame Number Register"]
pub mod usbhs_hstfnum;
#[doc = "Host Address 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr1](usbhs_hstaddr1) module"]
pub type USBHS_HSTADDR1 = crate::Reg<u32, _USBHS_HSTADDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTADDR1;
#[doc = "`read()` method returns [usbhs_hstaddr1::R](usbhs_hstaddr1::R) reader structure"]
impl crate::Readable for USBHS_HSTADDR1 {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr1::W](usbhs_hstaddr1::W) writer structure"]
impl crate::Writable for USBHS_HSTADDR1 {}
#[doc = "Host Address 1 Register"]
pub mod usbhs_hstaddr1;
#[doc = "Host Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr2](usbhs_hstaddr2) module"]
pub type USBHS_HSTADDR2 = crate::Reg<u32, _USBHS_HSTADDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTADDR2;
#[doc = "`read()` method returns [usbhs_hstaddr2::R](usbhs_hstaddr2::R) reader structure"]
impl crate::Readable for USBHS_HSTADDR2 {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr2::W](usbhs_hstaddr2::W) writer structure"]
impl crate::Writable for USBHS_HSTADDR2 {}
#[doc = "Host Address 2 Register"]
pub mod usbhs_hstaddr2;
#[doc = "Host Address 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstaddr3](usbhs_hstaddr3) module"]
pub type USBHS_HSTADDR3 = crate::Reg<u32, _USBHS_HSTADDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTADDR3;
#[doc = "`read()` method returns [usbhs_hstaddr3::R](usbhs_hstaddr3::R) reader structure"]
impl crate::Readable for USBHS_HSTADDR3 {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstaddr3::W](usbhs_hstaddr3::W) writer structure"]
impl crate::Writable for USBHS_HSTADDR3 {}
#[doc = "Host Address 3 Register"]
pub mod usbhs_hstaddr3;
#[doc = "Host Pipe Configuration Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipcfg](usbhs_hstpipcfg) module"]
pub type USBHS_HSTPIPCFG = crate::Reg<u32, _USBHS_HSTPIPCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPCFG;
#[doc = "`read()` method returns [usbhs_hstpipcfg::R](usbhs_hstpipcfg::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPCFG {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipcfg::W](usbhs_hstpipcfg::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPCFG {}
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod usbhs_hstpipcfg;
#[doc = "Host Pipe Status Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipisr](usbhs_hstpipisr) module"]
pub type USBHS_HSTPIPISR = crate::Reg<u32, _USBHS_HSTPIPISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPISR;
#[doc = "`read()` method returns [usbhs_hstpipisr::R](usbhs_hstpipisr::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPISR {}
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod usbhs_hstpipisr;
#[doc = "Host Pipe Clear Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipicr](usbhs_hstpipicr) module"]
pub type USBHS_HSTPIPICR = crate::Reg<u32, _USBHS_HSTPIPICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPICR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipicr::W](usbhs_hstpipicr::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPICR {}
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod usbhs_hstpipicr;
#[doc = "Host Pipe Set Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipifr](usbhs_hstpipifr) module"]
pub type USBHS_HSTPIPIFR = crate::Reg<u32, _USBHS_HSTPIPIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIFR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipifr::W](usbhs_hstpipifr::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIFR {}
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod usbhs_hstpipifr;
#[doc = "Host Pipe Mask Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipimr](usbhs_hstpipimr) module"]
pub type USBHS_HSTPIPIMR = crate::Reg<u32, _USBHS_HSTPIPIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIMR;
#[doc = "`read()` method returns [usbhs_hstpipimr::R](usbhs_hstpipimr::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPIMR {}
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod usbhs_hstpipimr;
#[doc = "Host Pipe Enable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipier](usbhs_hstpipier) module"]
pub type USBHS_HSTPIPIER = crate::Reg<u32, _USBHS_HSTPIPIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIER;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipier::W](usbhs_hstpipier::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIER {}
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod usbhs_hstpipier;
#[doc = "Host Pipe Disable Register (n = 0) 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipidr](usbhs_hstpipidr) module"]
pub type USBHS_HSTPIPIDR = crate::Reg<u32, _USBHS_HSTPIPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPIDR;
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipidr::W](usbhs_hstpipidr::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPIDR {}
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod usbhs_hstpipidr;
#[doc = "Host Pipe IN Request Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpipinrq](usbhs_hstpipinrq) module"]
pub type USBHS_HSTPIPINRQ = crate::Reg<u32, _USBHS_HSTPIPINRQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPINRQ;
#[doc = "`read()` method returns [usbhs_hstpipinrq::R](usbhs_hstpipinrq::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPINRQ {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpipinrq::W](usbhs_hstpipinrq::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPINRQ {}
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod usbhs_hstpipinrq;
#[doc = "Host Pipe Error Register (n = 0) 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_hstpiperr](usbhs_hstpiperr) module"]
pub type USBHS_HSTPIPERR = crate::Reg<u32, _USBHS_HSTPIPERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_HSTPIPERR;
#[doc = "`read()` method returns [usbhs_hstpiperr::R](usbhs_hstpiperr::R) reader structure"]
impl crate::Readable for USBHS_HSTPIPERR {}
#[doc = "`write(|w| ..)` method takes [usbhs_hstpiperr::W](usbhs_hstpiperr::W) writer structure"]
impl crate::Writable for USBHS_HSTPIPERR {}
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod usbhs_hstpiperr;
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_ctrl](usbhs_ctrl) module"]
pub type USBHS_CTRL = crate::Reg<u32, _USBHS_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_CTRL;
#[doc = "`read()` method returns [usbhs_ctrl::R](usbhs_ctrl::R) reader structure"]
impl crate::Readable for USBHS_CTRL {}
#[doc = "`write(|w| ..)` method takes [usbhs_ctrl::W](usbhs_ctrl::W) writer structure"]
impl crate::Writable for USBHS_CTRL {}
#[doc = "General Control Register"]
pub mod usbhs_ctrl;
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_sr](usbhs_sr) module"]
pub type USBHS_SR = crate::Reg<u32, _USBHS_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_SR;
#[doc = "`read()` method returns [usbhs_sr::R](usbhs_sr::R) reader structure"]
impl crate::Readable for USBHS_SR {}
#[doc = "General Status Register"]
pub mod usbhs_sr;
#[doc = "General Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_scr](usbhs_scr) module"]
pub type USBHS_SCR = crate::Reg<u32, _USBHS_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_SCR;
#[doc = "`write(|w| ..)` method takes [usbhs_scr::W](usbhs_scr::W) writer structure"]
impl crate::Writable for USBHS_SCR {}
#[doc = "General Status Clear Register"]
pub mod usbhs_scr;
#[doc = "General Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_sfr](usbhs_sfr) module"]
pub type USBHS_SFR = crate::Reg<u32, _USBHS_SFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBHS_SFR;
#[doc = "`write(|w| ..)` method takes [usbhs_sfr::W](usbhs_sfr::W) writer structure"]
impl crate::Writable for USBHS_SFR {}
#[doc = "General Status Set Register"]
pub mod usbhs_sfr;
