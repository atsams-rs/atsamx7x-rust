#[doc = "Reader of register ISI_CFG1"]
pub type R = crate::R<u32, super::ISI_CFG1>;
#[doc = "Writer for register ISI_CFG1"]
pub type W = crate::W<u32, super::ISI_CFG1>;
#[doc = "Register ISI_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSYNC_POL`"]
pub type HSYNC_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSYNC_POL`"]
pub struct HSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSYNC_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `VSYNC_POL`"]
pub type VSYNC_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VSYNC_POL`"]
pub struct VSYNC_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PIXCLK_POL`"]
pub type PIXCLK_POL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIXCLK_POL`"]
pub struct PIXCLK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> PIXCLK_POL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `GRAYLE`"]
pub type GRAYLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GRAYLE`"]
pub struct GRAYLE_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAYLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `EMB_SYNC`"]
pub type EMB_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EMB_SYNC`"]
pub struct EMB_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> EMB_SYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `CRC_SYNC`"]
pub type CRC_SYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC_SYNC`"]
pub struct CRC_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_SYNC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `FRATE`"]
pub type FRATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRATE`"]
pub struct FRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> FRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `DISCR`"]
pub type DISCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCR`"]
pub struct DISCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FULL`"]
pub type FULL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULL`"]
pub struct FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Threshold Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THMASK_A {
    #[doc = "0: Only 4 beats AHB burst allowed"]
    BEATS_4 = 0,
    #[doc = "1: Only 4 and 8 beats AHB burst allowed"]
    BEATS_8 = 1,
    #[doc = "2: 4, 8 and 16 beats AHB burst allowed"]
    BEATS_16 = 2,
}
impl From<THMASK_A> for u8 {
    #[inline(always)]
    fn from(variant: THMASK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THMASK`"]
pub type THMASK_R = crate::R<u8, THMASK_A>;
impl THMASK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, THMASK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(THMASK_A::BEATS_4),
            1 => Val(THMASK_A::BEATS_8),
            2 => Val(THMASK_A::BEATS_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BEATS_4`"]
    #[inline(always)]
    pub fn is_beats_4(&self) -> bool {
        *self == THMASK_A::BEATS_4
    }
    #[doc = "Checks if the value of the field is `BEATS_8`"]
    #[inline(always)]
    pub fn is_beats_8(&self) -> bool {
        *self == THMASK_A::BEATS_8
    }
    #[doc = "Checks if the value of the field is `BEATS_16`"]
    #[inline(always)]
    pub fn is_beats_16(&self) -> bool {
        *self == THMASK_A::BEATS_16
    }
}
#[doc = "Write proxy for field `THMASK`"]
pub struct THMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> THMASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THMASK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only 4 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_4(self) -> &'a mut W {
        self.variant(THMASK_A::BEATS_4)
    }
    #[doc = "Only 4 and 8 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_8(self) -> &'a mut W {
        self.variant(THMASK_A::BEATS_8)
    }
    #[doc = "4, 8 and 16 beats AHB burst allowed"]
    #[inline(always)]
    pub fn beats_16(self) -> &'a mut W {
        self.variant(THMASK_A::BEATS_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `SLD`"]
pub type SLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLD`"]
pub struct SLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SFD`"]
pub type SFD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SFD`"]
pub struct SFD_W<'a> {
    w: &'a mut W,
}
impl<'a> SFD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&self) -> HSYNC_POL_R {
        HSYNC_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&self) -> VSYNC_POL_R {
        VSYNC_POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&self) -> PIXCLK_POL_R {
        PIXCLK_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&self) -> GRAYLE_R {
        GRAYLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&self) -> EMB_SYNC_R {
        EMB_SYNC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&self) -> CRC_SYNC_R {
        CRC_SYNC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&self) -> FRATE_R {
        FRATE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&self) -> DISCR_R {
        DISCR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&self) -> THMASK_R {
        THMASK_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&self) -> SLD_R {
        SLD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hsync_pol(&mut self) -> HSYNC_POL_W {
        HSYNC_POL_W { w: self }
    }
    #[doc = "Bit 3 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vsync_pol(&mut self) -> VSYNC_POL_W {
        VSYNC_POL_W { w: self }
    }
    #[doc = "Bit 4 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pixclk_pol(&mut self) -> PIXCLK_POL_W {
        PIXCLK_POL_W { w: self }
    }
    #[doc = "Bit 5 - Grayscale Little Endian"]
    #[inline(always)]
    pub fn grayle(&mut self) -> GRAYLE_W {
        GRAYLE_W { w: self }
    }
    #[doc = "Bit 6 - Embedded Synchronization"]
    #[inline(always)]
    pub fn emb_sync(&mut self) -> EMB_SYNC_W {
        EMB_SYNC_W { w: self }
    }
    #[doc = "Bit 7 - Embedded Synchronization Correction"]
    #[inline(always)]
    pub fn crc_sync(&mut self) -> CRC_SYNC_W {
        CRC_SYNC_W { w: self }
    }
    #[doc = "Bits 8:10 - Frame Rate \\[0..7\\]"]
    #[inline(always)]
    pub fn frate(&mut self) -> FRATE_W {
        FRATE_W { w: self }
    }
    #[doc = "Bit 11 - Disable Codec Request"]
    #[inline(always)]
    pub fn discr(&mut self) -> DISCR_W {
        DISCR_W { w: self }
    }
    #[doc = "Bit 12 - Full Mode is Allowed"]
    #[inline(always)]
    pub fn full(&mut self) -> FULL_W {
        FULL_W { w: self }
    }
    #[doc = "Bits 13:14 - Threshold Mask"]
    #[inline(always)]
    pub fn thmask(&mut self) -> THMASK_W {
        THMASK_W { w: self }
    }
    #[doc = "Bits 16:23 - Start of Line Delay"]
    #[inline(always)]
    pub fn sld(&mut self) -> SLD_W {
        SLD_W { w: self }
    }
    #[doc = "Bits 24:31 - Start of Frame Delay"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W {
        SFD_W { w: self }
    }
}
