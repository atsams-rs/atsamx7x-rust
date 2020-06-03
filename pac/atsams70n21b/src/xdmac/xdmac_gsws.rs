#[doc = "Reader of register XDMAC_GSWS"]
pub type R = crate::R<u32, super::XDMAC_GSWS>;
#[doc = "Reader of field `SWRS0`"]
pub type SWRS0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS1`"]
pub type SWRS1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS2`"]
pub type SWRS2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS3`"]
pub type SWRS3_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS4`"]
pub type SWRS4_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS5`"]
pub type SWRS5_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS6`"]
pub type SWRS6_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS7`"]
pub type SWRS7_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS8`"]
pub type SWRS8_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS9`"]
pub type SWRS9_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS10`"]
pub type SWRS10_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS11`"]
pub type SWRS11_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS12`"]
pub type SWRS12_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS13`"]
pub type SWRS13_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS14`"]
pub type SWRS14_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS15`"]
pub type SWRS15_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS16`"]
pub type SWRS16_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS17`"]
pub type SWRS17_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS18`"]
pub type SWRS18_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS19`"]
pub type SWRS19_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS20`"]
pub type SWRS20_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS21`"]
pub type SWRS21_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS22`"]
pub type SWRS22_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWRS23`"]
pub type SWRS23_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs0(&self) -> SWRS0_R {
        SWRS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs1(&self) -> SWRS1_R {
        SWRS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs2(&self) -> SWRS2_R {
        SWRS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs3(&self) -> SWRS3_R {
        SWRS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs4(&self) -> SWRS4_R {
        SWRS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs5(&self) -> SWRS5_R {
        SWRS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs6(&self) -> SWRS6_R {
        SWRS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs7(&self) -> SWRS7_R {
        SWRS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs8(&self) -> SWRS8_R {
        SWRS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs9(&self) -> SWRS9_R {
        SWRS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs10(&self) -> SWRS10_R {
        SWRS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs11(&self) -> SWRS11_R {
        SWRS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs12(&self) -> SWRS12_R {
        SWRS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs13(&self) -> SWRS13_R {
        SWRS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs14(&self) -> SWRS14_R {
        SWRS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs15(&self) -> SWRS15_R {
        SWRS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs16(&self) -> SWRS16_R {
        SWRS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs17(&self) -> SWRS17_R {
        SWRS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs18(&self) -> SWRS18_R {
        SWRS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs19(&self) -> SWRS19_R {
        SWRS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs20(&self) -> SWRS20_R {
        SWRS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs21(&self) -> SWRS21_R {
        SWRS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs22(&self) -> SWRS22_R {
        SWRS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Status Bit"]
    #[inline(always)]
    pub fn swrs23(&self) -> SWRS23_R {
        SWRS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
