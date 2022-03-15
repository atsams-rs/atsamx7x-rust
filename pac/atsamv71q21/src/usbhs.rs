#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device General Control Register"]
    pub usbhs_devctrl: crate::Reg<usbhs_devctrl::USBHS_DEVCTRL_SPEC>,
    #[doc = "0x04 - Device Global Interrupt Status Register"]
    pub usbhs_devisr: crate::Reg<usbhs_devisr::USBHS_DEVISR_SPEC>,
    #[doc = "0x08 - Device Global Interrupt Clear Register"]
    pub usbhs_devicr: crate::Reg<usbhs_devicr::USBHS_DEVICR_SPEC>,
    #[doc = "0x0c - Device Global Interrupt Set Register"]
    pub usbhs_devifr: crate::Reg<usbhs_devifr::USBHS_DEVIFR_SPEC>,
    #[doc = "0x10 - Device Global Interrupt Mask Register"]
    pub usbhs_devimr: crate::Reg<usbhs_devimr::USBHS_DEVIMR_SPEC>,
    #[doc = "0x14 - Device Global Interrupt Disable Register"]
    pub usbhs_devidr: crate::Reg<usbhs_devidr::USBHS_DEVIDR_SPEC>,
    #[doc = "0x18 - Device Global Interrupt Enable Register"]
    pub usbhs_devier: crate::Reg<usbhs_devier::USBHS_DEVIER_SPEC>,
    #[doc = "0x1c - Device Endpoint Register"]
    pub usbhs_devept: crate::Reg<usbhs_devept::USBHS_DEVEPT_SPEC>,
    #[doc = "0x20 - Device Frame Number Register"]
    pub usbhs_devfnum: crate::Reg<usbhs_devfnum::USBHS_DEVFNUM_SPEC>,
    _reserved9: [u8; 0xdc],
    #[doc = "0x100..0x128 - Device Endpoint Configuration Register (n = 0) 0"]
    pub usbhs_deveptcfg: [crate::Reg<usbhs_deveptcfg::USBHS_DEVEPTCFG_SPEC>; 10],
    _reserved10: [u8; 0x08],
    #[doc = "0x130..0x158 - Device Endpoint Status Register (n = 0) 0"]
    pub usbhs_deveptisr: [crate::Reg<usbhs_deveptisr::USBHS_DEVEPTISR_SPEC>; 10],
    _reserved11: [u8; 0x08],
    #[doc = "0x160..0x188 - Device Endpoint Clear Register (n = 0) 0"]
    pub usbhs_devepticr: [crate::Reg<usbhs_devepticr::USBHS_DEVEPTICR_SPEC>; 10],
    _reserved12: [u8; 0x08],
    #[doc = "0x190..0x1b8 - Device Endpoint Set Register (n = 0) 0"]
    pub usbhs_deveptifr: [crate::Reg<usbhs_deveptifr::USBHS_DEVEPTIFR_SPEC>; 10],
    _reserved13: [u8; 0x08],
    #[doc = "0x1c0..0x1e8 - Device Endpoint Mask Register (n = 0) 0"]
    pub usbhs_deveptimr: [crate::Reg<usbhs_deveptimr::USBHS_DEVEPTIMR_SPEC>; 10],
    _reserved14: [u8; 0x08],
    #[doc = "0x1f0..0x218 - Device Endpoint Enable Register (n = 0) 0"]
    pub usbhs_deveptier: [crate::Reg<usbhs_deveptier::USBHS_DEVEPTIER_SPEC>; 10],
    _reserved15: [u8; 0x08],
    #[doc = "0x220..0x248 - Device Endpoint Disable Register (n = 0) 0"]
    pub usbhs_deveptidr: [crate::Reg<usbhs_deveptidr::USBHS_DEVEPTIDR_SPEC>; 10],
    _reserved16: [u8; 0xc8],
    #[doc = "0x310..0x380 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved17: [u8; 0x80],
    #[doc = "0x400 - Host General Control Register"]
    pub usbhs_hstctrl: crate::Reg<usbhs_hstctrl::USBHS_HSTCTRL_SPEC>,
    #[doc = "0x404 - Host Global Interrupt Status Register"]
    pub usbhs_hstisr: crate::Reg<usbhs_hstisr::USBHS_HSTISR_SPEC>,
    #[doc = "0x408 - Host Global Interrupt Clear Register"]
    pub usbhs_hsticr: crate::Reg<usbhs_hsticr::USBHS_HSTICR_SPEC>,
    #[doc = "0x40c - Host Global Interrupt Set Register"]
    pub usbhs_hstifr: crate::Reg<usbhs_hstifr::USBHS_HSTIFR_SPEC>,
    #[doc = "0x410 - Host Global Interrupt Mask Register"]
    pub usbhs_hstimr: crate::Reg<usbhs_hstimr::USBHS_HSTIMR_SPEC>,
    #[doc = "0x414 - Host Global Interrupt Disable Register"]
    pub usbhs_hstidr: crate::Reg<usbhs_hstidr::USBHS_HSTIDR_SPEC>,
    #[doc = "0x418 - Host Global Interrupt Enable Register"]
    pub usbhs_hstier: crate::Reg<usbhs_hstier::USBHS_HSTIER_SPEC>,
    #[doc = "0x41c - Host Pipe Register"]
    pub usbhs_hstpip: crate::Reg<usbhs_hstpip::USBHS_HSTPIP_SPEC>,
    #[doc = "0x420 - Host Frame Number Register"]
    pub usbhs_hstfnum: crate::Reg<usbhs_hstfnum::USBHS_HSTFNUM_SPEC>,
    #[doc = "0x424 - Host Address 1 Register"]
    pub usbhs_hstaddr1: crate::Reg<usbhs_hstaddr1::USBHS_HSTADDR1_SPEC>,
    #[doc = "0x428 - Host Address 2 Register"]
    pub usbhs_hstaddr2: crate::Reg<usbhs_hstaddr2::USBHS_HSTADDR2_SPEC>,
    #[doc = "0x42c - Host Address 3 Register"]
    pub usbhs_hstaddr3: crate::Reg<usbhs_hstaddr3::USBHS_HSTADDR3_SPEC>,
    _reserved29: [u8; 0xd0],
    #[doc = "0x500..0x528 - Host Pipe Configuration Register (n = 0) 0"]
    pub usbhs_hstpipcfg: [crate::Reg<usbhs_hstpipcfg::USBHS_HSTPIPCFG_SPEC>; 10],
    _reserved30: [u8; 0x08],
    #[doc = "0x530..0x558 - Host Pipe Status Register (n = 0) 0"]
    pub usbhs_hstpipisr: [crate::Reg<usbhs_hstpipisr::USBHS_HSTPIPISR_SPEC>; 10],
    _reserved31: [u8; 0x08],
    #[doc = "0x560..0x588 - Host Pipe Clear Register (n = 0) 0"]
    pub usbhs_hstpipicr: [crate::Reg<usbhs_hstpipicr::USBHS_HSTPIPICR_SPEC>; 10],
    _reserved32: [u8; 0x08],
    #[doc = "0x590..0x5b8 - Host Pipe Set Register (n = 0) 0"]
    pub usbhs_hstpipifr: [crate::Reg<usbhs_hstpipifr::USBHS_HSTPIPIFR_SPEC>; 10],
    _reserved33: [u8; 0x08],
    #[doc = "0x5c0..0x5e8 - Host Pipe Mask Register (n = 0) 0"]
    pub usbhs_hstpipimr: [crate::Reg<usbhs_hstpipimr::USBHS_HSTPIPIMR_SPEC>; 10],
    _reserved34: [u8; 0x08],
    #[doc = "0x5f0..0x618 - Host Pipe Enable Register (n = 0) 0"]
    pub usbhs_hstpipier: [crate::Reg<usbhs_hstpipier::USBHS_HSTPIPIER_SPEC>; 10],
    _reserved35: [u8; 0x08],
    #[doc = "0x620..0x648 - Host Pipe Disable Register (n = 0) 0"]
    pub usbhs_hstpipidr: [crate::Reg<usbhs_hstpipidr::USBHS_HSTPIPIDR_SPEC>; 10],
    _reserved36: [u8; 0x08],
    #[doc = "0x650..0x678 - Host Pipe IN Request Register (n = 0) 0"]
    pub usbhs_hstpipinrq: [crate::Reg<usbhs_hstpipinrq::USBHS_HSTPIPINRQ_SPEC>; 10],
    _reserved37: [u8; 0x08],
    #[doc = "0x680..0x6a8 - Host Pipe Error Register (n = 0) 0"]
    pub usbhs_hstpiperr: [crate::Reg<usbhs_hstpiperr::USBHS_HSTPIPERR_SPEC>; 10],
    _reserved38: [u8; 0x68],
    #[doc = "0x710..0x780 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved39: [u8; 0x80],
    #[doc = "0x800 - General Control Register"]
    pub usbhs_ctrl: crate::Reg<usbhs_ctrl::USBHS_CTRL_SPEC>,
    #[doc = "0x804 - General Status Register"]
    pub usbhs_sr: crate::Reg<usbhs_sr::USBHS_SR_SPEC>,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbhs_scr: crate::Reg<usbhs_scr::USBHS_SCR_SPEC>,
    #[doc = "0x80c - General Status Set Register"]
    pub usbhs_sfr: crate::Reg<usbhs_sfr::USBHS_SFR_SPEC>,
    #[doc = "0x810 - General Test A1 Register"]
    pub usbhs_tsta1: crate::Reg<usbhs_tsta1::USBHS_TSTA1_SPEC>,
    #[doc = "0x814 - General Test A2 Register"]
    pub usbhs_tsta2: crate::Reg<usbhs_tsta2::USBHS_TSTA2_SPEC>,
    #[doc = "0x818 - General Version Register"]
    pub usbhs_version: crate::Reg<usbhs_version::USBHS_VERSION_SPEC>,
    _reserved46: [u8; 0x10],
    #[doc = "0x82c - General Finite State Machine Register"]
    pub usbhs_fsm: crate::Reg<usbhs_fsm::USBHS_FSM_SPEC>,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_DEVDMA {
    #[doc = "0x00 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdmanxtdsc:
        crate::Reg<self::usbhs_devdma::usbhs_devdmanxtdsc::USBHS_DEVDMANXTDSC_SPEC>,
    #[doc = "0x04 - Device DMA Channel Address Register (n = 1)"]
    pub usbhs_devdmaaddress:
        crate::Reg<self::usbhs_devdma::usbhs_devdmaaddress::USBHS_DEVDMAADDRESS_SPEC>,
    #[doc = "0x08 - Device DMA Channel Control Register (n = 1)"]
    pub usbhs_devdmacontrol:
        crate::Reg<self::usbhs_devdma::usbhs_devdmacontrol::USBHS_DEVDMACONTROL_SPEC>,
    #[doc = "0x0c - Device DMA Channel Status Register (n = 1)"]
    pub usbhs_devdmastatus:
        crate::Reg<self::usbhs_devdma::usbhs_devdmastatus::USBHS_DEVDMASTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_devdma;
#[doc = r"Register block"]
#[repr(C)]
pub struct USBHS_HSTDMA {
    #[doc = "0x00 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdmanxtdsc:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmanxtdsc::USBHS_HSTDMANXTDSC_SPEC>,
    #[doc = "0x04 - Host DMA Channel Address Register (n = 1)"]
    pub usbhs_hstdmaaddress:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmaaddress::USBHS_HSTDMAADDRESS_SPEC>,
    #[doc = "0x08 - Host DMA Channel Control Register (n = 1)"]
    pub usbhs_hstdmacontrol:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmacontrol::USBHS_HSTDMACONTROL_SPEC>,
    #[doc = "0x0c - Host DMA Channel Status Register (n = 1)"]
    pub usbhs_hstdmastatus:
        crate::Reg<self::usbhs_hstdma::usbhs_hstdmastatus::USBHS_HSTDMASTATUS_SPEC>,
}
#[doc = r"Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_hstdma;
#[doc = "USBHS_DEVCTRL register accessor: an alias for `Reg<USBHS_DEVCTRL_SPEC>`"]
pub type USBHS_DEVCTRL = crate::Reg<usbhs_devctrl::USBHS_DEVCTRL_SPEC>;
#[doc = "Device General Control Register"]
pub mod usbhs_devctrl;
#[doc = "USBHS_DEVISR register accessor: an alias for `Reg<USBHS_DEVISR_SPEC>`"]
pub type USBHS_DEVISR = crate::Reg<usbhs_devisr::USBHS_DEVISR_SPEC>;
#[doc = "Device Global Interrupt Status Register"]
pub mod usbhs_devisr;
#[doc = "USBHS_DEVICR register accessor: an alias for `Reg<USBHS_DEVICR_SPEC>`"]
pub type USBHS_DEVICR = crate::Reg<usbhs_devicr::USBHS_DEVICR_SPEC>;
#[doc = "Device Global Interrupt Clear Register"]
pub mod usbhs_devicr;
#[doc = "USBHS_DEVIFR register accessor: an alias for `Reg<USBHS_DEVIFR_SPEC>`"]
pub type USBHS_DEVIFR = crate::Reg<usbhs_devifr::USBHS_DEVIFR_SPEC>;
#[doc = "Device Global Interrupt Set Register"]
pub mod usbhs_devifr;
#[doc = "USBHS_DEVIMR register accessor: an alias for `Reg<USBHS_DEVIMR_SPEC>`"]
pub type USBHS_DEVIMR = crate::Reg<usbhs_devimr::USBHS_DEVIMR_SPEC>;
#[doc = "Device Global Interrupt Mask Register"]
pub mod usbhs_devimr;
#[doc = "USBHS_DEVIDR register accessor: an alias for `Reg<USBHS_DEVIDR_SPEC>`"]
pub type USBHS_DEVIDR = crate::Reg<usbhs_devidr::USBHS_DEVIDR_SPEC>;
#[doc = "Device Global Interrupt Disable Register"]
pub mod usbhs_devidr;
#[doc = "USBHS_DEVIER register accessor: an alias for `Reg<USBHS_DEVIER_SPEC>`"]
pub type USBHS_DEVIER = crate::Reg<usbhs_devier::USBHS_DEVIER_SPEC>;
#[doc = "Device Global Interrupt Enable Register"]
pub mod usbhs_devier;
#[doc = "USBHS_DEVEPT register accessor: an alias for `Reg<USBHS_DEVEPT_SPEC>`"]
pub type USBHS_DEVEPT = crate::Reg<usbhs_devept::USBHS_DEVEPT_SPEC>;
#[doc = "Device Endpoint Register"]
pub mod usbhs_devept;
#[doc = "USBHS_DEVFNUM register accessor: an alias for `Reg<USBHS_DEVFNUM_SPEC>`"]
pub type USBHS_DEVFNUM = crate::Reg<usbhs_devfnum::USBHS_DEVFNUM_SPEC>;
#[doc = "Device Frame Number Register"]
pub mod usbhs_devfnum;
#[doc = "USBHS_DEVEPTCFG register accessor: an alias for `Reg<USBHS_DEVEPTCFG_SPEC>`"]
pub type USBHS_DEVEPTCFG = crate::Reg<usbhs_deveptcfg::USBHS_DEVEPTCFG_SPEC>;
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod usbhs_deveptcfg;
#[doc = "USBHS_DEVEPTISR register accessor: an alias for `Reg<USBHS_DEVEPTISR_SPEC>`"]
pub type USBHS_DEVEPTISR = crate::Reg<usbhs_deveptisr::USBHS_DEVEPTISR_SPEC>;
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod usbhs_deveptisr;
#[doc = "USBHS_DEVEPTICR register accessor: an alias for `Reg<USBHS_DEVEPTICR_SPEC>`"]
pub type USBHS_DEVEPTICR = crate::Reg<usbhs_devepticr::USBHS_DEVEPTICR_SPEC>;
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod usbhs_devepticr;
#[doc = "USBHS_DEVEPTIFR register accessor: an alias for `Reg<USBHS_DEVEPTIFR_SPEC>`"]
pub type USBHS_DEVEPTIFR = crate::Reg<usbhs_deveptifr::USBHS_DEVEPTIFR_SPEC>;
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod usbhs_deveptifr;
#[doc = "USBHS_DEVEPTIMR register accessor: an alias for `Reg<USBHS_DEVEPTIMR_SPEC>`"]
pub type USBHS_DEVEPTIMR = crate::Reg<usbhs_deveptimr::USBHS_DEVEPTIMR_SPEC>;
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod usbhs_deveptimr;
#[doc = "USBHS_DEVEPTIER register accessor: an alias for `Reg<USBHS_DEVEPTIER_SPEC>`"]
pub type USBHS_DEVEPTIER = crate::Reg<usbhs_deveptier::USBHS_DEVEPTIER_SPEC>;
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod usbhs_deveptier;
#[doc = "USBHS_DEVEPTIDR register accessor: an alias for `Reg<USBHS_DEVEPTIDR_SPEC>`"]
pub type USBHS_DEVEPTIDR = crate::Reg<usbhs_deveptidr::USBHS_DEVEPTIDR_SPEC>;
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod usbhs_deveptidr;
#[doc = "USBHS_HSTCTRL register accessor: an alias for `Reg<USBHS_HSTCTRL_SPEC>`"]
pub type USBHS_HSTCTRL = crate::Reg<usbhs_hstctrl::USBHS_HSTCTRL_SPEC>;
#[doc = "Host General Control Register"]
pub mod usbhs_hstctrl;
#[doc = "USBHS_HSTISR register accessor: an alias for `Reg<USBHS_HSTISR_SPEC>`"]
pub type USBHS_HSTISR = crate::Reg<usbhs_hstisr::USBHS_HSTISR_SPEC>;
#[doc = "Host Global Interrupt Status Register"]
pub mod usbhs_hstisr;
#[doc = "USBHS_HSTICR register accessor: an alias for `Reg<USBHS_HSTICR_SPEC>`"]
pub type USBHS_HSTICR = crate::Reg<usbhs_hsticr::USBHS_HSTICR_SPEC>;
#[doc = "Host Global Interrupt Clear Register"]
pub mod usbhs_hsticr;
#[doc = "USBHS_HSTIFR register accessor: an alias for `Reg<USBHS_HSTIFR_SPEC>`"]
pub type USBHS_HSTIFR = crate::Reg<usbhs_hstifr::USBHS_HSTIFR_SPEC>;
#[doc = "Host Global Interrupt Set Register"]
pub mod usbhs_hstifr;
#[doc = "USBHS_HSTIMR register accessor: an alias for `Reg<USBHS_HSTIMR_SPEC>`"]
pub type USBHS_HSTIMR = crate::Reg<usbhs_hstimr::USBHS_HSTIMR_SPEC>;
#[doc = "Host Global Interrupt Mask Register"]
pub mod usbhs_hstimr;
#[doc = "USBHS_HSTIDR register accessor: an alias for `Reg<USBHS_HSTIDR_SPEC>`"]
pub type USBHS_HSTIDR = crate::Reg<usbhs_hstidr::USBHS_HSTIDR_SPEC>;
#[doc = "Host Global Interrupt Disable Register"]
pub mod usbhs_hstidr;
#[doc = "USBHS_HSTIER register accessor: an alias for `Reg<USBHS_HSTIER_SPEC>`"]
pub type USBHS_HSTIER = crate::Reg<usbhs_hstier::USBHS_HSTIER_SPEC>;
#[doc = "Host Global Interrupt Enable Register"]
pub mod usbhs_hstier;
#[doc = "USBHS_HSTPIP register accessor: an alias for `Reg<USBHS_HSTPIP_SPEC>`"]
pub type USBHS_HSTPIP = crate::Reg<usbhs_hstpip::USBHS_HSTPIP_SPEC>;
#[doc = "Host Pipe Register"]
pub mod usbhs_hstpip;
#[doc = "USBHS_HSTFNUM register accessor: an alias for `Reg<USBHS_HSTFNUM_SPEC>`"]
pub type USBHS_HSTFNUM = crate::Reg<usbhs_hstfnum::USBHS_HSTFNUM_SPEC>;
#[doc = "Host Frame Number Register"]
pub mod usbhs_hstfnum;
#[doc = "USBHS_HSTADDR1 register accessor: an alias for `Reg<USBHS_HSTADDR1_SPEC>`"]
pub type USBHS_HSTADDR1 = crate::Reg<usbhs_hstaddr1::USBHS_HSTADDR1_SPEC>;
#[doc = "Host Address 1 Register"]
pub mod usbhs_hstaddr1;
#[doc = "USBHS_HSTADDR2 register accessor: an alias for `Reg<USBHS_HSTADDR2_SPEC>`"]
pub type USBHS_HSTADDR2 = crate::Reg<usbhs_hstaddr2::USBHS_HSTADDR2_SPEC>;
#[doc = "Host Address 2 Register"]
pub mod usbhs_hstaddr2;
#[doc = "USBHS_HSTADDR3 register accessor: an alias for `Reg<USBHS_HSTADDR3_SPEC>`"]
pub type USBHS_HSTADDR3 = crate::Reg<usbhs_hstaddr3::USBHS_HSTADDR3_SPEC>;
#[doc = "Host Address 3 Register"]
pub mod usbhs_hstaddr3;
#[doc = "USBHS_HSTPIPCFG register accessor: an alias for `Reg<USBHS_HSTPIPCFG_SPEC>`"]
pub type USBHS_HSTPIPCFG = crate::Reg<usbhs_hstpipcfg::USBHS_HSTPIPCFG_SPEC>;
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod usbhs_hstpipcfg;
#[doc = "USBHS_HSTPIPISR register accessor: an alias for `Reg<USBHS_HSTPIPISR_SPEC>`"]
pub type USBHS_HSTPIPISR = crate::Reg<usbhs_hstpipisr::USBHS_HSTPIPISR_SPEC>;
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod usbhs_hstpipisr;
#[doc = "USBHS_HSTPIPICR register accessor: an alias for `Reg<USBHS_HSTPIPICR_SPEC>`"]
pub type USBHS_HSTPIPICR = crate::Reg<usbhs_hstpipicr::USBHS_HSTPIPICR_SPEC>;
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod usbhs_hstpipicr;
#[doc = "USBHS_HSTPIPIFR register accessor: an alias for `Reg<USBHS_HSTPIPIFR_SPEC>`"]
pub type USBHS_HSTPIPIFR = crate::Reg<usbhs_hstpipifr::USBHS_HSTPIPIFR_SPEC>;
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod usbhs_hstpipifr;
#[doc = "USBHS_HSTPIPIMR register accessor: an alias for `Reg<USBHS_HSTPIPIMR_SPEC>`"]
pub type USBHS_HSTPIPIMR = crate::Reg<usbhs_hstpipimr::USBHS_HSTPIPIMR_SPEC>;
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod usbhs_hstpipimr;
#[doc = "USBHS_HSTPIPIER register accessor: an alias for `Reg<USBHS_HSTPIPIER_SPEC>`"]
pub type USBHS_HSTPIPIER = crate::Reg<usbhs_hstpipier::USBHS_HSTPIPIER_SPEC>;
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod usbhs_hstpipier;
#[doc = "USBHS_HSTPIPIDR register accessor: an alias for `Reg<USBHS_HSTPIPIDR_SPEC>`"]
pub type USBHS_HSTPIPIDR = crate::Reg<usbhs_hstpipidr::USBHS_HSTPIPIDR_SPEC>;
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod usbhs_hstpipidr;
#[doc = "USBHS_HSTPIPINRQ register accessor: an alias for `Reg<USBHS_HSTPIPINRQ_SPEC>`"]
pub type USBHS_HSTPIPINRQ = crate::Reg<usbhs_hstpipinrq::USBHS_HSTPIPINRQ_SPEC>;
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod usbhs_hstpipinrq;
#[doc = "USBHS_HSTPIPERR register accessor: an alias for `Reg<USBHS_HSTPIPERR_SPEC>`"]
pub type USBHS_HSTPIPERR = crate::Reg<usbhs_hstpiperr::USBHS_HSTPIPERR_SPEC>;
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod usbhs_hstpiperr;
#[doc = "USBHS_CTRL register accessor: an alias for `Reg<USBHS_CTRL_SPEC>`"]
pub type USBHS_CTRL = crate::Reg<usbhs_ctrl::USBHS_CTRL_SPEC>;
#[doc = "General Control Register"]
pub mod usbhs_ctrl;
#[doc = "USBHS_SR register accessor: an alias for `Reg<USBHS_SR_SPEC>`"]
pub type USBHS_SR = crate::Reg<usbhs_sr::USBHS_SR_SPEC>;
#[doc = "General Status Register"]
pub mod usbhs_sr;
#[doc = "USBHS_SCR register accessor: an alias for `Reg<USBHS_SCR_SPEC>`"]
pub type USBHS_SCR = crate::Reg<usbhs_scr::USBHS_SCR_SPEC>;
#[doc = "General Status Clear Register"]
pub mod usbhs_scr;
#[doc = "USBHS_SFR register accessor: an alias for `Reg<USBHS_SFR_SPEC>`"]
pub type USBHS_SFR = crate::Reg<usbhs_sfr::USBHS_SFR_SPEC>;
#[doc = "General Status Set Register"]
pub mod usbhs_sfr;
#[doc = "USBHS_TSTA1 register accessor: an alias for `Reg<USBHS_TSTA1_SPEC>`"]
pub type USBHS_TSTA1 = crate::Reg<usbhs_tsta1::USBHS_TSTA1_SPEC>;
#[doc = "General Test A1 Register"]
pub mod usbhs_tsta1;
#[doc = "USBHS_TSTA2 register accessor: an alias for `Reg<USBHS_TSTA2_SPEC>`"]
pub type USBHS_TSTA2 = crate::Reg<usbhs_tsta2::USBHS_TSTA2_SPEC>;
#[doc = "General Test A2 Register"]
pub mod usbhs_tsta2;
#[doc = "USBHS_VERSION register accessor: an alias for `Reg<USBHS_VERSION_SPEC>`"]
pub type USBHS_VERSION = crate::Reg<usbhs_version::USBHS_VERSION_SPEC>;
#[doc = "General Version Register"]
pub mod usbhs_version;
#[doc = "USBHS_FSM register accessor: an alias for `Reg<USBHS_FSM_SPEC>`"]
pub type USBHS_FSM = crate::Reg<usbhs_fsm::USBHS_FSM_SPEC>;
#[doc = "General Finite State Machine Register"]
pub mod usbhs_fsm;
