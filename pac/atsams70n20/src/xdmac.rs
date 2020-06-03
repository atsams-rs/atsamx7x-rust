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
    #[doc = "0x50 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid0: XDMAC_CHID,
    _reserved18: [u8; 8usize],
    #[doc = "0x90 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid1: XDMAC_CHID,
    _reserved19: [u8; 8usize],
    #[doc = "0xd0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid2: XDMAC_CHID,
    _reserved20: [u8; 8usize],
    #[doc = "0x110 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid3: XDMAC_CHID,
    _reserved21: [u8; 8usize],
    #[doc = "0x150 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid4: XDMAC_CHID,
    _reserved22: [u8; 8usize],
    #[doc = "0x190 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid5: XDMAC_CHID,
    _reserved23: [u8; 8usize],
    #[doc = "0x1d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid6: XDMAC_CHID,
    _reserved24: [u8; 8usize],
    #[doc = "0x210 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid7: XDMAC_CHID,
    _reserved25: [u8; 8usize],
    #[doc = "0x250 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid8: XDMAC_CHID,
    _reserved26: [u8; 8usize],
    #[doc = "0x290 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid9: XDMAC_CHID,
    _reserved27: [u8; 8usize],
    #[doc = "0x2d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid10: XDMAC_CHID,
    _reserved28: [u8; 8usize],
    #[doc = "0x310 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid11: XDMAC_CHID,
    _reserved29: [u8; 8usize],
    #[doc = "0x350 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid12: XDMAC_CHID,
    _reserved30: [u8; 8usize],
    #[doc = "0x390 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid13: XDMAC_CHID,
    _reserved31: [u8; 8usize],
    #[doc = "0x3d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid14: XDMAC_CHID,
    _reserved32: [u8; 8usize],
    #[doc = "0x410 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid15: XDMAC_CHID,
    _reserved33: [u8; 8usize],
    #[doc = "0x450 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid16: XDMAC_CHID,
    _reserved34: [u8; 8usize],
    #[doc = "0x490 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid17: XDMAC_CHID,
    _reserved35: [u8; 8usize],
    #[doc = "0x4d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid18: XDMAC_CHID,
    _reserved36: [u8; 8usize],
    #[doc = "0x510 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid19: XDMAC_CHID,
    _reserved37: [u8; 8usize],
    #[doc = "0x550 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid20: XDMAC_CHID,
    _reserved38: [u8; 8usize],
    #[doc = "0x590 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid21: XDMAC_CHID,
    _reserved39: [u8; 8usize],
    #[doc = "0x5d0 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid22: XDMAC_CHID,
    _reserved40: [u8; 8usize],
    #[doc = "0x610 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_chid23: XDMAC_CHID,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct XDMAC_CHID {
    #[doc = "0x00 - Channel Interrupt Enable Register (chid = 0)"]
    pub xdmac_cie: self::xdmac_chid::XDMAC_CIE,
    #[doc = "0x04 - Channel Interrupt Disable Register (chid = 0)"]
    pub xdmac_cid: self::xdmac_chid::XDMAC_CID,
    #[doc = "0x08 - Channel Interrupt Mask Register (chid = 0)"]
    pub xdmac_cim: self::xdmac_chid::XDMAC_CIM,
    #[doc = "0x0c - Channel Interrupt Status Register (chid = 0)"]
    pub xdmac_cis: self::xdmac_chid::XDMAC_CIS,
    #[doc = "0x10 - Channel Source Address Register (chid = 0)"]
    pub xdmac_csa: self::xdmac_chid::XDMAC_CSA,
    #[doc = "0x14 - Channel Destination Address Register (chid = 0)"]
    pub xdmac_cda: self::xdmac_chid::XDMAC_CDA,
    #[doc = "0x18 - Channel Next Descriptor Address Register (chid = 0)"]
    pub xdmac_cnda: self::xdmac_chid::XDMAC_CNDA,
    #[doc = "0x1c - Channel Next Descriptor Control Register (chid = 0)"]
    pub xdmac_cndc: self::xdmac_chid::XDMAC_CNDC,
    #[doc = "0x20 - Channel Microblock Control Register (chid = 0)"]
    pub xdmac_cubc: self::xdmac_chid::XDMAC_CUBC,
    #[doc = "0x24 - Channel Block Control Register (chid = 0)"]
    pub xdmac_cbc: self::xdmac_chid::XDMAC_CBC,
    #[doc = "0x28 - Channel Configuration Register (chid = 0)"]
    pub xdmac_cc: self::xdmac_chid::XDMAC_CC,
    #[doc = "0x2c - Channel Data Stride Memory Set Pattern (chid = 0)"]
    pub xdmac_cds_msp: self::xdmac_chid::XDMAC_CDS_MSP,
    #[doc = "0x30 - Channel Source Microblock Stride (chid = 0)"]
    pub xdmac_csus: self::xdmac_chid::XDMAC_CSUS,
    #[doc = "0x34 - Channel Destination Microblock Stride (chid = 0)"]
    pub xdmac_cdus: self::xdmac_chid::XDMAC_CDUS,
}
#[doc = r"Register block"]
#[doc = "Channel Interrupt Enable Register (chid = 0)"]
pub mod xdmac_chid;
#[doc = "Global Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gtype](xdmac_gtype) module"]
pub type XDMAC_GTYPE = crate::Reg<u32, _XDMAC_GTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GTYPE;
#[doc = "`read()` method returns [xdmac_gtype::R](xdmac_gtype::R) reader structure"]
impl crate::Readable for XDMAC_GTYPE {}
#[doc = "Global Type Register"]
pub mod xdmac_gtype;
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gcfg](xdmac_gcfg) module"]
pub type XDMAC_GCFG = crate::Reg<u32, _XDMAC_GCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GCFG;
#[doc = "`read()` method returns [xdmac_gcfg::R](xdmac_gcfg::R) reader structure"]
impl crate::Readable for XDMAC_GCFG {}
#[doc = "`write(|w| ..)` method takes [xdmac_gcfg::W](xdmac_gcfg::W) writer structure"]
impl crate::Writable for XDMAC_GCFG {}
#[doc = "Global Configuration Register"]
pub mod xdmac_gcfg;
#[doc = "Global Weighted Arbiter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gwac](xdmac_gwac) module"]
pub type XDMAC_GWAC = crate::Reg<u32, _XDMAC_GWAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GWAC;
#[doc = "`read()` method returns [xdmac_gwac::R](xdmac_gwac::R) reader structure"]
impl crate::Readable for XDMAC_GWAC {}
#[doc = "`write(|w| ..)` method takes [xdmac_gwac::W](xdmac_gwac::W) writer structure"]
impl crate::Writable for XDMAC_GWAC {}
#[doc = "Global Weighted Arbiter Configuration Register"]
pub mod xdmac_gwac;
#[doc = "Global Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gie](xdmac_gie) module"]
pub type XDMAC_GIE = crate::Reg<u32, _XDMAC_GIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GIE;
#[doc = "`write(|w| ..)` method takes [xdmac_gie::W](xdmac_gie::W) writer structure"]
impl crate::Writable for XDMAC_GIE {}
#[doc = "Global Interrupt Enable Register"]
pub mod xdmac_gie;
#[doc = "Global Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gid](xdmac_gid) module"]
pub type XDMAC_GID = crate::Reg<u32, _XDMAC_GID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GID;
#[doc = "`write(|w| ..)` method takes [xdmac_gid::W](xdmac_gid::W) writer structure"]
impl crate::Writable for XDMAC_GID {}
#[doc = "Global Interrupt Disable Register"]
pub mod xdmac_gid;
#[doc = "Global Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gim](xdmac_gim) module"]
pub type XDMAC_GIM = crate::Reg<u32, _XDMAC_GIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GIM;
#[doc = "`read()` method returns [xdmac_gim::R](xdmac_gim::R) reader structure"]
impl crate::Readable for XDMAC_GIM {}
#[doc = "Global Interrupt Mask Register"]
pub mod xdmac_gim;
#[doc = "Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gis](xdmac_gis) module"]
pub type XDMAC_GIS = crate::Reg<u32, _XDMAC_GIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GIS;
#[doc = "`read()` method returns [xdmac_gis::R](xdmac_gis::R) reader structure"]
impl crate::Readable for XDMAC_GIS {}
#[doc = "Global Interrupt Status Register"]
pub mod xdmac_gis;
#[doc = "Global Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_ge](xdmac_ge) module"]
pub type XDMAC_GE = crate::Reg<u32, _XDMAC_GE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GE;
#[doc = "`write(|w| ..)` method takes [xdmac_ge::W](xdmac_ge::W) writer structure"]
impl crate::Writable for XDMAC_GE {}
#[doc = "Global Channel Enable Register"]
pub mod xdmac_ge;
#[doc = "Global Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gd](xdmac_gd) module"]
pub type XDMAC_GD = crate::Reg<u32, _XDMAC_GD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GD;
#[doc = "`write(|w| ..)` method takes [xdmac_gd::W](xdmac_gd::W) writer structure"]
impl crate::Writable for XDMAC_GD {}
#[doc = "Global Channel Disable Register"]
pub mod xdmac_gd;
#[doc = "Global Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gs](xdmac_gs) module"]
pub type XDMAC_GS = crate::Reg<u32, _XDMAC_GS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GS;
#[doc = "`read()` method returns [xdmac_gs::R](xdmac_gs::R) reader structure"]
impl crate::Readable for XDMAC_GS {}
#[doc = "Global Channel Status Register"]
pub mod xdmac_gs;
#[doc = "Global Channel Read Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_grs](xdmac_grs) module"]
pub type XDMAC_GRS = crate::Reg<u32, _XDMAC_GRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GRS;
#[doc = "`read()` method returns [xdmac_grs::R](xdmac_grs::R) reader structure"]
impl crate::Readable for XDMAC_GRS {}
#[doc = "`write(|w| ..)` method takes [xdmac_grs::W](xdmac_grs::W) writer structure"]
impl crate::Writable for XDMAC_GRS {}
#[doc = "Global Channel Read Suspend Register"]
pub mod xdmac_grs;
#[doc = "Global Channel Write Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gws](xdmac_gws) module"]
pub type XDMAC_GWS = crate::Reg<u32, _XDMAC_GWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GWS;
#[doc = "`read()` method returns [xdmac_gws::R](xdmac_gws::R) reader structure"]
impl crate::Readable for XDMAC_GWS {}
#[doc = "`write(|w| ..)` method takes [xdmac_gws::W](xdmac_gws::W) writer structure"]
impl crate::Writable for XDMAC_GWS {}
#[doc = "Global Channel Write Suspend Register"]
pub mod xdmac_gws;
#[doc = "Global Channel Read Write Suspend Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_grws](xdmac_grws) module"]
pub type XDMAC_GRWS = crate::Reg<u32, _XDMAC_GRWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GRWS;
#[doc = "`write(|w| ..)` method takes [xdmac_grws::W](xdmac_grws::W) writer structure"]
impl crate::Writable for XDMAC_GRWS {}
#[doc = "Global Channel Read Write Suspend Register"]
pub mod xdmac_grws;
#[doc = "Global Channel Read Write Resume Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_grwr](xdmac_grwr) module"]
pub type XDMAC_GRWR = crate::Reg<u32, _XDMAC_GRWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GRWR;
#[doc = "`write(|w| ..)` method takes [xdmac_grwr::W](xdmac_grwr::W) writer structure"]
impl crate::Writable for XDMAC_GRWR {}
#[doc = "Global Channel Read Write Resume Register"]
pub mod xdmac_grwr;
#[doc = "Global Channel Software Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gswr](xdmac_gswr) module"]
pub type XDMAC_GSWR = crate::Reg<u32, _XDMAC_GSWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GSWR;
#[doc = "`write(|w| ..)` method takes [xdmac_gswr::W](xdmac_gswr::W) writer structure"]
impl crate::Writable for XDMAC_GSWR {}
#[doc = "Global Channel Software Request Register"]
pub mod xdmac_gswr;
#[doc = "Global Channel Software Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gsws](xdmac_gsws) module"]
pub type XDMAC_GSWS = crate::Reg<u32, _XDMAC_GSWS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GSWS;
#[doc = "`read()` method returns [xdmac_gsws::R](xdmac_gsws::R) reader structure"]
impl crate::Readable for XDMAC_GSWS {}
#[doc = "Global Channel Software Request Status Register"]
pub mod xdmac_gsws;
#[doc = "Global Channel Software Flush Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gswf](xdmac_gswf) module"]
pub type XDMAC_GSWF = crate::Reg<u32, _XDMAC_GSWF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XDMAC_GSWF;
#[doc = "`write(|w| ..)` method takes [xdmac_gswf::W](xdmac_gswf::W) writer structure"]
impl crate::Writable for XDMAC_GSWF {}
#[doc = "Global Channel Software Flush Request Register"]
pub mod xdmac_gswf;
