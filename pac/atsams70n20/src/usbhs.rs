#[doc = r" Register block"]
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
    _reserved0: [u8; 220usize],
    #[doc = "0x100 - Device Endpoint Configuration Register (n = 0) 0"]
    pub usbhs_deveptcfg: [USBHS_DEVEPTCFG; 10],
    _reserved1: [u8; 8usize],
    #[doc = "0x130 - Device Endpoint Status Register (n = 0) 0"]
    pub usbhs_deveptisr: [USBHS_DEVEPTISR; 10],
    _reserved2: [u8; 8usize],
    #[doc = "0x160 - Device Endpoint Clear Register (n = 0) 0"]
    pub usbhs_devepticr: [USBHS_DEVEPTICR; 10],
    _reserved3: [u8; 8usize],
    #[doc = "0x190 - Device Endpoint Set Register (n = 0) 0"]
    pub usbhs_deveptifr: [USBHS_DEVEPTIFR; 10],
    _reserved4: [u8; 8usize],
    #[doc = "0x1c0 - Device Endpoint Mask Register (n = 0) 0"]
    pub usbhs_deveptimr: [USBHS_DEVEPTIMR; 10],
    _reserved5: [u8; 8usize],
    #[doc = "0x1f0 - Device Endpoint Enable Register (n = 0) 0"]
    pub usbhs_deveptier: [USBHS_DEVEPTIER; 10],
    _reserved6: [u8; 8usize],
    #[doc = "0x220 - Device Endpoint Disable Register (n = 0) 0"]
    pub usbhs_deveptidr: [USBHS_DEVEPTIDR; 10],
    _reserved7: [u8; 200usize],
    #[doc = "0x310 - Device DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_devdma: [USBHS_DEVDMA; 7],
    _reserved8: [u8; 128usize],
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
    _reserved9: [u8; 208usize],
    #[doc = "0x500 - Host Pipe Configuration Register (n = 0) 0"]
    pub usbhs_hstpipcfg: [USBHS_HSTPIPCFG; 10],
    _reserved10: [u8; 8usize],
    #[doc = "0x530 - Host Pipe Status Register (n = 0) 0"]
    pub usbhs_hstpipisr: [USBHS_HSTPIPISR; 10],
    _reserved11: [u8; 8usize],
    #[doc = "0x560 - Host Pipe Clear Register (n = 0) 0"]
    pub usbhs_hstpipicr: [USBHS_HSTPIPICR; 10],
    _reserved12: [u8; 8usize],
    #[doc = "0x590 - Host Pipe Set Register (n = 0) 0"]
    pub usbhs_hstpipifr: [USBHS_HSTPIPIFR; 10],
    _reserved13: [u8; 8usize],
    #[doc = "0x5c0 - Host Pipe Mask Register (n = 0) 0"]
    pub usbhs_hstpipimr: [USBHS_HSTPIPIMR; 10],
    _reserved14: [u8; 8usize],
    #[doc = "0x5f0 - Host Pipe Enable Register (n = 0) 0"]
    pub usbhs_hstpipier: [USBHS_HSTPIPIER; 10],
    _reserved15: [u8; 8usize],
    #[doc = "0x620 - Host Pipe Disable Register (n = 0) 0"]
    pub usbhs_hstpipidr: [USBHS_HSTPIPIDR; 10],
    _reserved16: [u8; 8usize],
    #[doc = "0x650 - Host Pipe IN Request Register (n = 0) 0"]
    pub usbhs_hstpipinrq: [USBHS_HSTPIPINRQ; 10],
    _reserved17: [u8; 8usize],
    #[doc = "0x680 - Host Pipe Error Register (n = 0) 0"]
    pub usbhs_hstpiperr: [USBHS_HSTPIPERR; 10],
    _reserved18: [u8; 104usize],
    #[doc = "0x710 - Host DMA Channel Next Descriptor Address Register (n = 1)"]
    pub usbhs_hstdma: [USBHS_HSTDMA; 7],
    _reserved19: [u8; 128usize],
    #[doc = "0x800 - General Control Register"]
    pub usbhs_ctrl: USBHS_CTRL,
    #[doc = "0x804 - General Status Register"]
    pub usbhs_sr: USBHS_SR,
    #[doc = "0x808 - General Status Clear Register"]
    pub usbhs_scr: USBHS_SCR,
    #[doc = "0x80c - General Status Set Register"]
    pub usbhs_sfr: USBHS_SFR,
}
#[doc = r" Register block"]
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
#[doc = r" Register block"]
#[doc = "Device DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_devdma;
#[doc = r" Register block"]
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
#[doc = r" Register block"]
#[doc = "Host DMA Channel Next Descriptor Address Register (n = 1)"]
pub mod usbhs_hstdma;
#[doc = "Device General Control Register"]
pub struct USBHS_DEVCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device General Control Register"]
pub mod usbhs_devctrl;
#[doc = "Device Global Interrupt Status Register"]
pub struct USBHS_DEVISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Status Register"]
pub mod usbhs_devisr;
#[doc = "Device Global Interrupt Clear Register"]
pub struct USBHS_DEVICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Clear Register"]
pub mod usbhs_devicr;
#[doc = "Device Global Interrupt Set Register"]
pub struct USBHS_DEVIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Set Register"]
pub mod usbhs_devifr;
#[doc = "Device Global Interrupt Mask Register"]
pub struct USBHS_DEVIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Mask Register"]
pub mod usbhs_devimr;
#[doc = "Device Global Interrupt Disable Register"]
pub struct USBHS_DEVIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Disable Register"]
pub mod usbhs_devidr;
#[doc = "Device Global Interrupt Enable Register"]
pub struct USBHS_DEVIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Global Interrupt Enable Register"]
pub mod usbhs_devier;
#[doc = "Device Endpoint Register"]
pub struct USBHS_DEVEPT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Register"]
pub mod usbhs_devept;
#[doc = "Device Frame Number Register"]
pub struct USBHS_DEVFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Frame Number Register"]
pub mod usbhs_devfnum;
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub struct USBHS_DEVEPTCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Configuration Register (n = 0) 0"]
pub mod usbhs_deveptcfg;
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub struct USBHS_DEVEPTISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Status Register (n = 0) 0"]
pub mod usbhs_deveptisr;
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub struct USBHS_DEVEPTICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Clear Register (n = 0) 0"]
pub mod usbhs_devepticr;
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub struct USBHS_DEVEPTIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Set Register (n = 0) 0"]
pub mod usbhs_deveptifr;
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub struct USBHS_DEVEPTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Mask Register (n = 0) 0"]
pub mod usbhs_deveptimr;
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub struct USBHS_DEVEPTIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Enable Register (n = 0) 0"]
pub mod usbhs_deveptier;
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub struct USBHS_DEVEPTIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device Endpoint Disable Register (n = 0) 0"]
pub mod usbhs_deveptidr;
#[doc = "Host General Control Register"]
pub struct USBHS_HSTCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host General Control Register"]
pub mod usbhs_hstctrl;
#[doc = "Host Global Interrupt Status Register"]
pub struct USBHS_HSTISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Status Register"]
pub mod usbhs_hstisr;
#[doc = "Host Global Interrupt Clear Register"]
pub struct USBHS_HSTICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Clear Register"]
pub mod usbhs_hsticr;
#[doc = "Host Global Interrupt Set Register"]
pub struct USBHS_HSTIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Set Register"]
pub mod usbhs_hstifr;
#[doc = "Host Global Interrupt Mask Register"]
pub struct USBHS_HSTIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Mask Register"]
pub mod usbhs_hstimr;
#[doc = "Host Global Interrupt Disable Register"]
pub struct USBHS_HSTIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Disable Register"]
pub mod usbhs_hstidr;
#[doc = "Host Global Interrupt Enable Register"]
pub struct USBHS_HSTIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Global Interrupt Enable Register"]
pub mod usbhs_hstier;
#[doc = "Host Pipe Register"]
pub struct USBHS_HSTPIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Register"]
pub mod usbhs_hstpip;
#[doc = "Host Frame Number Register"]
pub struct USBHS_HSTFNUM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Frame Number Register"]
pub mod usbhs_hstfnum;
#[doc = "Host Address 1 Register"]
pub struct USBHS_HSTADDR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Address 1 Register"]
pub mod usbhs_hstaddr1;
#[doc = "Host Address 2 Register"]
pub struct USBHS_HSTADDR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Address 2 Register"]
pub mod usbhs_hstaddr2;
#[doc = "Host Address 3 Register"]
pub struct USBHS_HSTADDR3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Address 3 Register"]
pub mod usbhs_hstaddr3;
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub struct USBHS_HSTPIPCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Configuration Register (n = 0) 0"]
pub mod usbhs_hstpipcfg;
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub struct USBHS_HSTPIPISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Status Register (n = 0) 0"]
pub mod usbhs_hstpipisr;
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub struct USBHS_HSTPIPICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Clear Register (n = 0) 0"]
pub mod usbhs_hstpipicr;
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub struct USBHS_HSTPIPIFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Set Register (n = 0) 0"]
pub mod usbhs_hstpipifr;
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub struct USBHS_HSTPIPIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Mask Register (n = 0) 0"]
pub mod usbhs_hstpipimr;
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub struct USBHS_HSTPIPIER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Enable Register (n = 0) 0"]
pub mod usbhs_hstpipier;
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub struct USBHS_HSTPIPIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Disable Register (n = 0) 0"]
pub mod usbhs_hstpipidr;
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub struct USBHS_HSTPIPINRQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe IN Request Register (n = 0) 0"]
pub mod usbhs_hstpipinrq;
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub struct USBHS_HSTPIPERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Host Pipe Error Register (n = 0) 0"]
pub mod usbhs_hstpiperr;
#[doc = "General Control Register"]
pub struct USBHS_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Control Register"]
pub mod usbhs_ctrl;
#[doc = "General Status Register"]
pub struct USBHS_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Register"]
pub mod usbhs_sr;
#[doc = "General Status Clear Register"]
pub struct USBHS_SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Clear Register"]
pub mod usbhs_scr;
#[doc = "General Status Set Register"]
pub struct USBHS_SFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Status Set Register"]
pub mod usbhs_sfr;
