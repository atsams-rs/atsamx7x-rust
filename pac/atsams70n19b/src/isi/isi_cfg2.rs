#[doc = "Reader of register ISI_CFG2"]
pub type R = crate::R<u32, super::ISI_CFG2>;
#[doc = "Writer for register ISI_CFG2"]
pub type W = crate::W<u32, super::ISI_CFG2>;
#[doc = "Register ISI_CFG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_CFG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IM_VSIZE`"]
pub type IM_VSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IM_VSIZE`"]
pub struct IM_VSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_VSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `GS_MODE`"]
pub type GS_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GS_MODE`"]
pub struct GS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GS_MODE_W<'a> {
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
#[doc = "Reader of field `RGB_MODE`"]
pub type RGB_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RGB_MODE`"]
pub struct RGB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_MODE_W<'a> {
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
#[doc = "Reader of field `GRAYSCALE`"]
pub type GRAYSCALE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GRAYSCALE`"]
pub struct GRAYSCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> GRAYSCALE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RGB_SWAP`"]
pub type RGB_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RGB_SWAP`"]
pub struct RGB_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `COL_SPACE`"]
pub type COL_SPACE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COL_SPACE`"]
pub struct COL_SPACE_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_SPACE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `IM_HSIZE`"]
pub type IM_HSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IM_HSIZE`"]
pub struct IM_HSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> IM_HSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | (((value as u32) & 0x07ff) << 16);
        self.w
    }
}
#[doc = "YCrCb Format Swap Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum YCC_SWAP_A {
    #[doc = "0: Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    MODE1 = 1,
    #[doc = "2: Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    MODE3 = 3,
}
impl From<YCC_SWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: YCC_SWAP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `YCC_SWAP`"]
pub type YCC_SWAP_R = crate::R<u8, YCC_SWAP_A>;
impl YCC_SWAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YCC_SWAP_A {
        match self.bits {
            0 => YCC_SWAP_A::DEFAULT,
            1 => YCC_SWAP_A::MODE1,
            2 => YCC_SWAP_A::MODE2,
            3 => YCC_SWAP_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == YCC_SWAP_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == YCC_SWAP_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == YCC_SWAP_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == YCC_SWAP_A::MODE3
    }
}
#[doc = "Write proxy for field `YCC_SWAP`"]
pub struct YCC_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> YCC_SWAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: YCC_SWAP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::DEFAULT)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::MODE1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::MODE2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(YCC_SWAP_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "RGB Pixel Mapping Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RGB_CFG_A {
    #[doc = "0: Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    MODE1 = 1,
    #[doc = "2: Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    MODE3 = 3,
}
impl From<RGB_CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RGB_CFG`"]
pub type RGB_CFG_R = crate::R<u8, RGB_CFG_A>;
impl RGB_CFG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_CFG_A {
        match self.bits {
            0 => RGB_CFG_A::DEFAULT,
            1 => RGB_CFG_A::MODE1,
            2 => RGB_CFG_A::MODE2,
            3 => RGB_CFG_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RGB_CFG_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == RGB_CFG_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == RGB_CFG_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == RGB_CFG_A::MODE3
    }
}
#[doc = "Write proxy for field `RGB_CFG`"]
pub struct RGB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_CFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB_CFG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RGB_CFG_A::DEFAULT)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(RGB_CFG_A::MODE1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(RGB_CFG_A::MODE2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(RGB_CFG_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&self) -> IM_VSIZE_R {
        IM_VSIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&self) -> GS_MODE_R {
        GS_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&self) -> RGB_MODE_R {
        RGB_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&self) -> GRAYSCALE_R {
        GRAYSCALE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&self) -> RGB_SWAP_R {
        RGB_SWAP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&self) -> COL_SPACE_R {
        COL_SPACE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&self) -> IM_HSIZE_R {
        IM_HSIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&self) -> YCC_SWAP_R {
        YCC_SWAP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&self) -> RGB_CFG_R {
        RGB_CFG_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&mut self) -> IM_VSIZE_W {
        IM_VSIZE_W { w: self }
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&mut self) -> GS_MODE_W {
        GS_MODE_W { w: self }
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&mut self) -> RGB_MODE_W {
        RGB_MODE_W { w: self }
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&mut self) -> GRAYSCALE_W {
        GRAYSCALE_W { w: self }
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&mut self) -> RGB_SWAP_W {
        RGB_SWAP_W { w: self }
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&mut self) -> COL_SPACE_W {
        COL_SPACE_W { w: self }
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&mut self) -> IM_HSIZE_W {
        IM_HSIZE_W { w: self }
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&mut self) -> YCC_SWAP_W {
        YCC_SWAP_W { w: self }
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&mut self) -> RGB_CFG_W {
        RGB_CFG_W { w: self }
    }
}
