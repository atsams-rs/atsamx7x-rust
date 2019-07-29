#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global Type Register"]
    pub xdmac_gtype: XDMAC_GTYPE,
    #[doc = "0x04 - Global Configuration Register"]
    pub xdmac_gcfg: XDMAC_GCFG,
    #[doc = "0x08 - Global Weighted Arbiter Configuration Register"]
    pub xdmac_gwac: XDMAC_GWAC,
    #[doc = "0x0c - Global Interrupt Enable Register"]
    pub xdmac_gie: XDMAC_GIE,
    #[doc = "0x10 - Global Interrupt Disable Register"]
    pub xdmac_gid: XDMAC_GID,
    #[doc = "0x14 - Global Interrupt Mask Register"]
    pub xdmac_gim: XDMAC_GIM,
    #[doc = "0x18 - Global Interrupt Status Register"]
    pub xdmac_gis: XDMAC_GIS,
    #[doc = "0x1c - Global Channel Enable Register"]
    pub xdmac_ge: XDMAC_GE,
    #[doc = "0x20 - Global Channel Disable Register"]
    pub xdmac_gd: XDMAC_GD,
    #[doc = "0x24 - Global Channel Status Register"]
    pub xdmac_gs: XDMAC_GS,
    #[doc = "0x28 - Global Channel Read Suspend Register"]
    pub xdmac_grs: XDMAC_GRS,
    #[doc = "0x2c - Global Channel Write Suspend Register"]
    pub xdmac_gws: XDMAC_GWS,
    #[doc = "0x30 - Global Channel Read Write Suspend Register"]
    pub xdmac_grws: XDMAC_GRWS,
    #[doc = "0x34 - Global Channel Read Write Resume Register"]
    pub xdmac_grwr: XDMAC_GRWR,
    #[doc = "0x38 - Global Channel Software Request Register"]
    pub xdmac_gswr: XDMAC_GSWR,
    #[doc = "0x3c - Global Channel Software Request Status Register"]
    pub xdmac_gsws: XDMAC_GSWS,
    #[doc = "0x40 - Global Channel Software Flush Request Register"]
    pub xdmac_gswf: XDMAC_GSWF,
    _reserved17: [u8; 12usize],
    #[doc = "0x50 - Channel Interrupt Enable Register"]
    pub xdmac_chid0: XDMAC_CHID,
    _reserved18: [u8; 8usize],
    #[doc = "0x90 - Channel Interrupt Enable Register"]
    pub xdmac_chid1: XDMAC_CHID,
    _reserved19: [u8; 8usize],
    #[doc = "0xd0 - Channel Interrupt Enable Register"]
    pub xdmac_chid2: XDMAC_CHID,
    _reserved20: [u8; 8usize],
    #[doc = "0x110 - Channel Interrupt Enable Register"]
    pub xdmac_chid3: XDMAC_CHID,
    _reserved21: [u8; 8usize],
    #[doc = "0x150 - Channel Interrupt Enable Register"]
    pub xdmac_chid4: XDMAC_CHID,
    _reserved22: [u8; 8usize],
    #[doc = "0x190 - Channel Interrupt Enable Register"]
    pub xdmac_chid5: XDMAC_CHID,
    _reserved23: [u8; 8usize],
    #[doc = "0x1d0 - Channel Interrupt Enable Register"]
    pub xdmac_chid6: XDMAC_CHID,
    _reserved24: [u8; 8usize],
    #[doc = "0x210 - Channel Interrupt Enable Register"]
    pub xdmac_chid7: XDMAC_CHID,
    _reserved25: [u8; 8usize],
    #[doc = "0x250 - Channel Interrupt Enable Register"]
    pub xdmac_chid8: XDMAC_CHID,
    _reserved26: [u8; 8usize],
    #[doc = "0x290 - Channel Interrupt Enable Register"]
    pub xdmac_chid9: XDMAC_CHID,
    _reserved27: [u8; 8usize],
    #[doc = "0x2d0 - Channel Interrupt Enable Register"]
    pub xdmac_chid10: XDMAC_CHID,
    _reserved28: [u8; 8usize],
    #[doc = "0x310 - Channel Interrupt Enable Register"]
    pub xdmac_chid11: XDMAC_CHID,
    _reserved29: [u8; 8usize],
    #[doc = "0x350 - Channel Interrupt Enable Register"]
    pub xdmac_chid12: XDMAC_CHID,
    _reserved30: [u8; 8usize],
    #[doc = "0x390 - Channel Interrupt Enable Register"]
    pub xdmac_chid13: XDMAC_CHID,
    _reserved31: [u8; 8usize],
    #[doc = "0x3d0 - Channel Interrupt Enable Register"]
    pub xdmac_chid14: XDMAC_CHID,
    _reserved32: [u8; 8usize],
    #[doc = "0x410 - Channel Interrupt Enable Register"]
    pub xdmac_chid15: XDMAC_CHID,
    _reserved33: [u8; 8usize],
    #[doc = "0x450 - Channel Interrupt Enable Register"]
    pub xdmac_chid16: XDMAC_CHID,
    _reserved34: [u8; 8usize],
    #[doc = "0x490 - Channel Interrupt Enable Register"]
    pub xdmac_chid17: XDMAC_CHID,
    _reserved35: [u8; 8usize],
    #[doc = "0x4d0 - Channel Interrupt Enable Register"]
    pub xdmac_chid18: XDMAC_CHID,
    _reserved36: [u8; 8usize],
    #[doc = "0x510 - Channel Interrupt Enable Register"]
    pub xdmac_chid19: XDMAC_CHID,
    _reserved37: [u8; 8usize],
    #[doc = "0x550 - Channel Interrupt Enable Register"]
    pub xdmac_chid20: XDMAC_CHID,
    _reserved38: [u8; 8usize],
    #[doc = "0x590 - Channel Interrupt Enable Register"]
    pub xdmac_chid21: XDMAC_CHID,
    _reserved39: [u8; 8usize],
    #[doc = "0x5d0 - Channel Interrupt Enable Register"]
    pub xdmac_chid22: XDMAC_CHID,
    _reserved40: [u8; 8usize],
    #[doc = "0x610 - Channel Interrupt Enable Register"]
    pub xdmac_chid23: XDMAC_CHID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct XDMAC_CHID {
    #[doc = "0x00 - Channel Interrupt Enable Register"]
    pub xdmac_cie: self::xdmac_chid::XDMAC_CIE,
    #[doc = "0x04 - Channel Interrupt Disable Register"]
    pub xdmac_cid: self::xdmac_chid::XDMAC_CID,
    #[doc = "0x08 - Channel Interrupt Mask Register"]
    pub xdmac_cim: self::xdmac_chid::XDMAC_CIM,
    #[doc = "0x0c - Channel Interrupt Status Register"]
    pub xdmac_cis: self::xdmac_chid::XDMAC_CIS,
    #[doc = "0x10 - Channel Source Address Register"]
    pub xdmac_csa: self::xdmac_chid::XDMAC_CSA,
    #[doc = "0x14 - Channel Destination Address Register"]
    pub xdmac_cda: self::xdmac_chid::XDMAC_CDA,
    #[doc = "0x18 - Channel Next Descriptor Address Register"]
    pub xdmac_cnda: self::xdmac_chid::XDMAC_CNDA,
    #[doc = "0x1c - Channel Next Descriptor Control Register"]
    pub xdmac_cndc: self::xdmac_chid::XDMAC_CNDC,
    #[doc = "0x20 - Channel Microblock Control Register"]
    pub xdmac_cubc: self::xdmac_chid::XDMAC_CUBC,
    #[doc = "0x24 - Channel Block Control Register"]
    pub xdmac_cbc: self::xdmac_chid::XDMAC_CBC,
    #[doc = "0x28 - Channel Configuration Register"]
    pub xdmac_cc: self::xdmac_chid::XDMAC_CC,
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern"]
    pub xdmac_cds_msp: self::xdmac_chid::XDMAC_CDS_MSP,
    #[doc = "0x30 - Channel Source Microblock Stride"]
    pub xdmac_csus: self::xdmac_chid::XDMAC_CSUS,
    #[doc = "0x34 - Channel Destination Microblock Stride"]
    pub xdmac_cdus: self::xdmac_chid::XDMAC_CDUS,
}
#[doc = r"Register block"]
#[doc = "Channel Interrupt Enable Register"]
pub mod xdmac_chid;
#[doc = "Global Type Register"]
pub struct XDMAC_GTYPE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Type Register"]
pub mod xdmac_gtype;
#[doc = "Global Configuration Register"]
pub struct XDMAC_GCFG {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Configuration Register"]
pub mod xdmac_gcfg;
#[doc = "Global Weighted Arbiter Configuration Register"]
pub struct XDMAC_GWAC {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod xdmac_gwac;
#[doc = "Global Interrupt Enable Register"]
pub struct XDMAC_GIE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Interrupt Enable Register"]
pub mod xdmac_gie;
#[doc = "Global Interrupt Disable Register"]
pub struct XDMAC_GID {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Interrupt Disable Register"]
pub mod xdmac_gid;
#[doc = "Global Interrupt Mask Register"]
pub struct XDMAC_GIM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Interrupt Mask Register"]
pub mod xdmac_gim;
#[doc = "Global Interrupt Status Register"]
pub struct XDMAC_GIS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Interrupt Status Register"]
pub mod xdmac_gis;
#[doc = "Global Channel Enable Register"]
pub struct XDMAC_GE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Enable Register"]
pub mod xdmac_ge;
#[doc = "Global Channel Disable Register"]
pub struct XDMAC_GD {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Disable Register"]
pub mod xdmac_gd;
#[doc = "Global Channel Status Register"]
pub struct XDMAC_GS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Status Register"]
pub mod xdmac_gs;
#[doc = "Global Channel Read Suspend Register"]
pub struct XDMAC_GRS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Read Suspend Register"]
pub mod xdmac_grs;
#[doc = "Global Channel Write Suspend Register"]
pub struct XDMAC_GWS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Write Suspend Register"]
pub mod xdmac_gws;
#[doc = "Global Channel Read Write Suspend Register"]
pub struct XDMAC_GRWS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Read Write Suspend Register"]
pub mod xdmac_grws;
#[doc = "Global Channel Read Write Resume Register"]
pub struct XDMAC_GRWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Read Write Resume Register"]
pub mod xdmac_grwr;
#[doc = "Global Channel Software Request Register"]
pub struct XDMAC_GSWR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Software Request Register"]
pub mod xdmac_gswr;
#[doc = "Global Channel Software Request Status Register"]
pub struct XDMAC_GSWS {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Software Request Status Register"]
pub mod xdmac_gsws;
#[doc = "Global Channel Software Flush Request Register"]
pub struct XDMAC_GSWF {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Global Channel Software Flush Request Register"]
pub mod xdmac_gswf;
