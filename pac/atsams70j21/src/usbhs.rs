#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub devctrl: crate::Reg<devctrl::DEVCTRL_SPEC>,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub devisr: crate::Reg<devisr::DEVISR_SPEC>,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub devicr: crate::Reg<devicr::DEVICR_SPEC>,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub devifr: crate::Reg<devifr::DEVIFR_SPEC>,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub devimr: crate::Reg<devimr::DEVIMR_SPEC>,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub devidr: crate::Reg<devidr::DEVIDR_SPEC>,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub devier: crate::Reg<devier::DEVIER_SPEC>,
    #[doc = "0x1c - Device Endpoint Register"]
    pub devept: crate::Reg<devept::DEVEPT_SPEC>,
    #[doc = "0x20 - Device Frame Number Register"]
    pub devfnum: crate::Reg<devfnum::DEVFNUM_SPEC>,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register (n = 0) 0"]
    pub deveptcfg: [crate::Reg<deveptcfg::DEVEPTCFG_SPEC>; 10],
    _reserved10: [u8; 0x08],
    #[doc = "0x130..0x158 - Device Endpoint Status Register (n = 0) 0"]
    pub deveptisr: [crate::Reg<deveptisr::DEVEPTISR_SPEC>; 10],
    _reserved11: [u8; 0x08],
    #[doc = "0x160..0x188 - Device Endpoint Clear Register (n = 0) 0"]
    pub devepticr: [crate::Reg<devepticr::DEVEPTICR_SPEC>; 10],
    _reserved12: [u8; 0x08],
    #[doc = "0x190..0x1b8 - Device Endpoint Set Register (n = 0) 0"]
    pub deveptifr: [crate::Reg<deveptifr::DEVEPTIFR_SPEC>; 10],
    _reserved13: [u8; 0x08],
    #[doc = "0x1c0..0x1e8 - Device Endpoint Mask Register (n = 0) 0"]
    pub deveptimr: [crate::Reg<deveptimr::DEVEPTIMR_SPEC>; 10],
    _reserved14: [u8; 0x08],
    #[doc = "0x1f0..0x218 - Device Endpoint Enable Register (n = 0) 0"]
    pub deveptier: [crate::Reg<deveptier::DEVEPTIER_SPEC>; 10],
    _reserved15: [u8; 0x08],
    #[doc = "0x220..0x248 - Device Endpoint Disable Register (n = 0) 0"]
    pub deveptidr: [crate::Reg<deveptidr::DEVEPTIDR_SPEC>; 10],
    _reserved16: [u8; 0xc8],
    #[doc = "0x310..0x380 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 0x80],
    #[doc = "0x400 - Host General Control Register"]
    pub hstctrl: crate::Reg<hstctrl::HSTCTRL_SPEC>,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub hstisr: crate::Reg<hstisr::HSTISR_SPEC>,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub hsticr: crate::Reg<hsticr::HSTICR_SPEC>,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub hstifr: crate::Reg<hstifr::HSTIFR_SPEC>,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub hstimr: crate::Reg<hstimr::HSTIMR_SPEC>,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub hstidr: crate::Reg<hstidr::HSTIDR_SPEC>,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub hstier: crate::Reg<hstier::HSTIER_SPEC>,
    #[doc = "0x41c - Host Pipe Register"]
    pub hstpip: crate::Reg<hstpip::HSTPIP_SPEC>,
    #[doc = "0x420 - Host Frame Number Register"]
    pub hstfnum: crate::Reg<hstfnum::HSTFNUM_SPEC>,
    #[doc = "0x424 - Host Address 1 Register"]
    pub hstaddr1: crate::Reg<hstaddr1::HSTADDR1_SPEC>,
    #[doc = "0x428 - Host Address 2 Register"]
    pub hstaddr2: crate::Reg<hstaddr2::HSTADDR2_SPEC>,
    #[doc = "0x42c - Host Address 3 Register"]
    pub hstaddr3: crate::Reg<hstaddr3::HSTADDR3_SPEC>,
    _reserved29: [u8; 0xd0],
    #[doc = "0x500..0x528 - Host Pipe Configuration Register (n = 0) 0"]
    pub hstpipcfg: [crate::Reg<hstpipcfg::HSTPIPCFG_SPEC>; 10],
    _reserved30: [u8; 0x08],
    #[doc = "0x530..0x558 - Host Pipe Status Register (n = 0) 0"]
    pub hstpipisr: [crate::Reg<hstpipisr::HSTPIPISR_SPEC>; 10],
    _reserved31: [u8; 0x08],
    #[doc = "0x560..0x588 - Host Pipe Clear Register (n = 0) 0"]
    pub hstpipicr: [crate::Reg<hstpipicr::HSTPIPICR_SPEC>; 10],
    _reserved32: [u8; 0x08],
    #[doc = "0x590..0x5b8 - Host Pipe Set Register (n = 0) 0"]
    pub hstpipifr: [crate::Reg<hstpipifr::HSTPIPIFR_SPEC>; 10],
    _reserved33: [u8; 0x08],
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register (n = 0) 0"]
    pub hstpipimr: [crate::Reg<hstpipimr::HSTPIPIMR_SPEC>; 10],
    _reserved34: [u8; 0x08],
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register (n = 0) 0"]
    pub hstpipier: [crate::Reg<hstpipier::HSTPIPIER_SPEC>; 10],
    _reserved35: [u8; 0x08],
    #[doc = "0x620..0x648 - Host Pipe Disable Register (n = 0) 0"]
    pub hstpipidr: [crate::Reg<hstpipidr::HSTPIPIDR_SPEC>; 10],
    _reserved36: [u8; 0x08],
    #[doc = "0x650..0x678 - Host Pipe IN Request Register (n = 0) 0"]
    pub hstpipinrq: [crate::Reg<hstpipinrq::HSTPIPINRQ_SPEC>; 10],
    _reserved37: [u8; 0x08],
    #[doc = "0x680..0x6a8 - Host Pipe Error Register (n = 0) 0"]
    pub hstpiperr: [crate::Reg<hstpiperr::HSTPIPERR_SPEC>; 10],
    _reserved38: [u8; 0x68],
    #[doc = "0x710..0x780 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 0x80],
    #[doc = "0x800 - General Control Register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x804 - General Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x808 - General Status Clear Register"]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x80c - General Status Set Register"]
    pub sfr: crate::Reg<sfr::SFR_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub devdmanxtdsc: crate::Reg<self::usbhs_devdma::devdmanxtdsc::DEVDMANXTDSC_SPEC>,
    #[doc = "0x04 - Device DMA Channel Address Register (n = 1)"]
    pub devdmaaddress: crate::Reg<self::usbhs_devdma::devdmaaddress::DEVDMAADDRESS_SPEC>,
    #[doc = "0x08 - Device DMA Channel Control Register (n = 1)"]
    pub devdmacontrol: crate::Reg<self::usbhs_devdma::devdmacontrol::DEVDMACONTROL_SPEC>,
    #[doc = "0x0c - Device DMA Channel Status Register (n = 1)"]
    pub devdmastatus: crate::Reg<self::usbhs_devdma::devdmastatus::DEVDMASTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub hstdmanxtdsc: crate::Reg<self::usbhs_hstdma::hstdmanxtdsc::HSTDMANXTDSC_SPEC>,
    #[doc = "0x04 - Host DMA Channel Address Register (n = 1)"]
    pub hstdmaaddress: crate::Reg<self::usbhs_hstdma::hstdmaaddress::HSTDMAADDRESS_SPEC>,
    #[doc = "0x08 - Host DMA Channel Control Register (n = 1)"]
    pub hstdmacontrol: crate::Reg<self::usbhs_hstdma::hstdmacontrol::HSTDMACONTROL_SPEC>,
    #[doc = "0x0c - Host DMA Channel Status Register (n = 1)"]
    pub hstdmastatus: crate::Reg<self::usbhs_hstdma::hstdmastatus::HSTDMASTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_hstdma;
#[doc = "DEVCTRL register accessor: an alias for `Reg<DEVCTRL_SPEC>`"]
pub type DEVCTRL = crate::Reg<devctrl::DEVCTRL_SPEC>;
#[doc = "Device General Control Register"]
pub mod devctrl;
#[doc = "DEVISR register accessor: an alias for `Reg<DEVISR_SPEC>`"]
pub type DEVISR = crate::Reg<devisr::DEVISR_SPEC>;
#[doc = "Device Global Interrupt Status Register"]
pub mod devisr;
#[doc = "DEVICR register accessor: an alias for `Reg<DEVICR_SPEC>`"]
pub type DEVICR = crate::Reg<devicr::DEVICR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod devicr;
#[doc = "DEVIFR register accessor: an alias for `Reg<DEVIFR_SPEC>`"]
pub type DEVIFR = crate::Reg<devifr::DEVIFR_SPEC>;
#[doc = "Device Global Interrupt Set Register"]
pub mod devifr;
#[doc = "DEVIMR register accessor: an alias for `Reg<DEVIMR_SPEC>`"]
pub type DEVIMR = crate::Reg<devimr::DEVIMR_SPEC>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod devimr;
#[doc = "DEVIDR register accessor: an alias for `Reg<DEVIDR_SPEC>`"]
pub type DEVIDR = crate::Reg<devidr::DEVIDR_SPEC>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod devidr;
#[doc = "DEVIER register accessor: an alias for `Reg<DEVIER_SPEC>`"]
pub type DEVIER = crate::Reg<devier::DEVIER_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod devier;
#[doc = "DEVEPT register accessor: an alias for `Reg<DEVEPT_SPEC>`"]
pub type DEVEPT = crate::Reg<devept::DEVEPT_SPEC>;
#[doc = "Device Endpoint Register"]
pub mod devept;
#[doc = "DEVFNUM register accessor: an alias for `Reg<DEVFNUM_SPEC>`"]
pub type DEVFNUM = crate::Reg<devfnum::DEVFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod devfnum;
#[doc = "DEVEPTCFG register accessor: an alias for `Reg<DEVEPTCFG_SPEC>`"]
pub type DEVEPTCFG = crate::Reg<deveptcfg::DEVEPTCFG_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod deveptcfg;
#[doc = "DEVEPTISR register accessor: an alias for `Reg<DEVEPTISR_SPEC>`"]
pub type DEVEPTISR = crate::Reg<deveptisr::DEVEPTISR_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod deveptisr;
#[doc = "DEVEPTICR register accessor: an alias for `Reg<DEVEPTICR_SPEC>`"]
pub type DEVEPTICR = crate::Reg<devepticr::DEVEPTICR_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod devepticr;
#[doc = "DEVEPTIFR register accessor: an alias for `Reg<DEVEPTIFR_SPEC>`"]
pub type DEVEPTIFR = crate::Reg<deveptifr::DEVEPTIFR_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod deveptifr;
#[doc = "DEVEPTIMR register accessor: an alias for `Reg<DEVEPTIMR_SPEC>`"]
pub type DEVEPTIMR = crate::Reg<deveptimr::DEVEPTIMR_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod deveptimr;
#[doc = "DEVEPTIER register accessor: an alias for `Reg<DEVEPTIER_SPEC>`"]
pub type DEVEPTIER = crate::Reg<deveptier::DEVEPTIER_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod deveptier;
#[doc = "DEVEPTIDR register accessor: an alias for `Reg<DEVEPTIDR_SPEC>`"]
pub type DEVEPTIDR = crate::Reg<deveptidr::DEVEPTIDR_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod deveptidr;
#[doc = "HSTCTRL register accessor: an alias for `Reg<HSTCTRL_SPEC>`"]
pub type HSTCTRL = crate::Reg<hstctrl::HSTCTRL_SPEC>;
#[doc = "Host General Control Register"]
pub mod hstctrl;
#[doc = "HSTISR register accessor: an alias for `Reg<HSTISR_SPEC>`"]
pub type HSTISR = crate::Reg<hstisr::HSTISR_SPEC>;
#[doc = "Host Global Interrupt Status Register"]
pub mod hstisr;
#[doc = "HSTICR register accessor: an alias for `Reg<HSTICR_SPEC>`"]
pub type HSTICR = crate::Reg<hsticr::HSTICR_SPEC>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod hsticr;
#[doc = "HSTIFR register accessor: an alias for `Reg<HSTIFR_SPEC>`"]
pub type HSTIFR = crate::Reg<hstifr::HSTIFR_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod hstifr;
#[doc = "HSTIMR register accessor: an alias for `Reg<HSTIMR_SPEC>`"]
pub type HSTIMR = crate::Reg<hstimr::HSTIMR_SPEC>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod hstimr;
#[doc = "HSTIDR register accessor: an alias for `Reg<HSTIDR_SPEC>`"]
pub type HSTIDR = crate::Reg<hstidr::HSTIDR_SPEC>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod hstidr;
#[doc = "HSTIER register accessor: an alias for `Reg<HSTIER_SPEC>`"]
pub type HSTIER = crate::Reg<hstier::HSTIER_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod hstier;
#[doc = "HSTPIP register accessor: an alias for `Reg<HSTPIP_SPEC>`"]
pub type HSTPIP = crate::Reg<hstpip::HSTPIP_SPEC>;
#[doc = "Host Pipe Register"]
pub mod hstpip;
#[doc = "HSTFNUM register accessor: an alias for `Reg<HSTFNUM_SPEC>`"]
pub type HSTFNUM = crate::Reg<hstfnum::HSTFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod hstfnum;
#[doc = "HSTADDR1 register accessor: an alias for `Reg<HSTADDR1_SPEC>`"]
pub type HSTADDR1 = crate::Reg<hstaddr1::HSTADDR1_SPEC>;
#[doc = "Host Address 1 Register"]
pub mod hstaddr1;
#[doc = "HSTADDR2 register accessor: an alias for `Reg<HSTADDR2_SPEC>`"]
pub type HSTADDR2 = crate::Reg<hstaddr2::HSTADDR2_SPEC>;
#[doc = "Host Address 2 Register"]
pub mod hstaddr2;
#[doc = "HSTADDR3 register accessor: an alias for `Reg<HSTADDR3_SPEC>`"]
pub type HSTADDR3 = crate::Reg<hstaddr3::HSTADDR3_SPEC>;
#[doc = "Host Address 3 Register"]
pub mod hstaddr3;
#[doc = "HSTPIPCFG register accessor: an alias for `Reg<HSTPIPCFG_SPEC>`"]
pub type HSTPIPCFG = crate::Reg<hstpipcfg::HSTPIPCFG_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod hstpipcfg;
#[doc = "HSTPIPISR register accessor: an alias for `Reg<HSTPIPISR_SPEC>`"]
pub type HSTPIPISR = crate::Reg<hstpipisr::HSTPIPISR_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod hstpipisr;
#[doc = "HSTPIPICR register accessor: an alias for `Reg<HSTPIPICR_SPEC>`"]
pub type HSTPIPICR = crate::Reg<hstpipicr::HSTPIPICR_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod hstpipicr;
#[doc = "HSTPIPIFR register accessor: an alias for `Reg<HSTPIPIFR_SPEC>`"]
pub type HSTPIPIFR = crate::Reg<hstpipifr::HSTPIPIFR_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod hstpipifr;
#[doc = "HSTPIPIMR register accessor: an alias for `Reg<HSTPIPIMR_SPEC>`"]
pub type HSTPIPIMR = crate::Reg<hstpipimr::HSTPIPIMR_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod hstpipimr;
#[doc = "HSTPIPIER register accessor: an alias for `Reg<HSTPIPIER_SPEC>`"]
pub type HSTPIPIER = crate::Reg<hstpipier::HSTPIPIER_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod hstpipier;
#[doc = "HSTPIPIDR register accessor: an alias for `Reg<HSTPIPIDR_SPEC>`"]
pub type HSTPIPIDR = crate::Reg<hstpipidr::HSTPIPIDR_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod hstpipidr;
#[doc = "HSTPIPINRQ register accessor: an alias for `Reg<HSTPIPINRQ_SPEC>`"]
pub type HSTPIPINRQ = crate::Reg<hstpipinrq::HSTPIPINRQ_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod hstpipinrq;
#[doc = "HSTPIPERR register accessor: an alias for `Reg<HSTPIPERR_SPEC>`"]
pub type HSTPIPERR = crate::Reg<hstpiperr::HSTPIPERR_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod hstpiperr;
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "General Control Register"]
pub mod ctrl;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "General Status Register"]
pub mod sr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod scr;
#[doc = "SFR register accessor: an alias for `Reg<SFR_SPEC>`"]
pub type SFR = crate::Reg<sfr::SFR_SPEC>;
#[doc = "General Status Set Register"]
pub mod sfr;
