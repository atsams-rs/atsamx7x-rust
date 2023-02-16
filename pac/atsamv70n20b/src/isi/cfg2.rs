#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM_VSIZE` reader - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub type IM_VSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IM_VSIZE` writer - Vertical Size of the Image Sensor \\[0..2047\\]"]
pub type IM_VSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u16, u16, 11, O>;
#[doc = "Field `GS_MODE` reader - Grayscale Pixel Format Mode"]
pub type GS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `GS_MODE` writer - Grayscale Pixel Format Mode"]
pub type GS_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `RGB_MODE` reader - RGB Input Mode"]
pub type RGB_MODE_R = crate::BitReader<bool>;
#[doc = "Field `RGB_MODE` writer - RGB Input Mode"]
pub type RGB_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `GRAYSCALE` reader - Grayscale Mode Format Enable"]
pub type GRAYSCALE_R = crate::BitReader<bool>;
#[doc = "Field `GRAYSCALE` writer - Grayscale Mode Format Enable"]
pub type GRAYSCALE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `RGB_SWAP` reader - RGB Format Swap Mode"]
pub type RGB_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `RGB_SWAP` writer - RGB Format Swap Mode"]
pub type RGB_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `COL_SPACE` reader - Color Space for the Image Data"]
pub type COL_SPACE_R = crate::BitReader<bool>;
#[doc = "Field `COL_SPACE` writer - Color Space for the Image Data"]
pub type COL_SPACE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, bool, O>;
#[doc = "Field `IM_HSIZE` reader - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub type IM_HSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IM_HSIZE` writer - Horizontal Size of the Image Sensor \\[0..2047\\]"]
pub type IM_HSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG2_SPEC, u16, u16, 11, O>;
#[doc = "Field `YCC_SWAP` reader - YCrCb Format Swap Mode"]
pub type YCC_SWAP_R = crate::FieldReader<u8, YCC_SWAPSELECT_A>;
#[doc = "YCrCb Format Swap Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YCC_SWAPSELECT_A {
    #[doc = "0: Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    MODE1 = 1,
    #[doc = "2: Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    MODE3 = 3,
}
impl From<YCC_SWAPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: YCC_SWAPSELECT_A) -> Self {
        variant as _
    }
}
impl YCC_SWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YCC_SWAPSELECT_A {
        match self.bits {
            0 => YCC_SWAPSELECT_A::DEFAULT,
            1 => YCC_SWAPSELECT_A::MODE1,
            2 => YCC_SWAPSELECT_A::MODE2,
            3 => YCC_SWAPSELECT_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == YCC_SWAPSELECT_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == YCC_SWAPSELECT_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == YCC_SWAPSELECT_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == YCC_SWAPSELECT_A::MODE3
    }
}
#[doc = "Field `YCC_SWAP` writer - YCrCb Format Swap Mode"]
pub type YCC_SWAP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, YCC_SWAPSELECT_A, 2, O>;
impl<'a, const O: u8> YCC_SWAP_W<'a, O> {
    #[doc = "Byte 0 Cb(i)Byte 1 Y(i)Byte 2 Cr(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(YCC_SWAPSELECT_A::DEFAULT)
    }
    #[doc = "Byte 0 Cr(i)Byte 1 Y(i)Byte 2 Cb(i)Byte 3 Y(i+1)"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(YCC_SWAPSELECT_A::MODE1)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cb(i)Byte 2 Y(i+1)Byte 3 Cr(i)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(YCC_SWAPSELECT_A::MODE2)
    }
    #[doc = "Byte 0 Y(i)Byte 1 Cr(i)Byte 2 Y(i+1)Byte 3 Cb(i)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(YCC_SWAPSELECT_A::MODE3)
    }
}
#[doc = "Field `RGB_CFG` reader - RGB Pixel Mapping Configuration"]
pub type RGB_CFG_R = crate::FieldReader<u8, RGB_CFGSELECT_A>;
#[doc = "RGB Pixel Mapping Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RGB_CFGSELECT_A {
    #[doc = "0: Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    DEFAULT = 0,
    #[doc = "1: Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    MODE1 = 1,
    #[doc = "2: Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    MODE2 = 2,
    #[doc = "3: Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    MODE3 = 3,
}
impl From<RGB_CFGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_CFGSELECT_A) -> Self {
        variant as _
    }
}
impl RGB_CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_CFGSELECT_A {
        match self.bits {
            0 => RGB_CFGSELECT_A::DEFAULT,
            1 => RGB_CFGSELECT_A::MODE1,
            2 => RGB_CFGSELECT_A::MODE2,
            3 => RGB_CFGSELECT_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RGB_CFGSELECT_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == RGB_CFGSELECT_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == RGB_CFGSELECT_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == RGB_CFGSELECT_A::MODE3
    }
}
#[doc = "Field `RGB_CFG` writer - RGB Pixel Mapping Configuration"]
pub type RGB_CFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, RGB_CFGSELECT_A, 2, O>;
impl<'a, const O: u8> RGB_CFG_W<'a, O> {
    #[doc = "Byte 0 R/G(MSB)Byte 1 G(LSB)/BByte 2 R/G(MSB)Byte 3 G(LSB)/B"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RGB_CFGSELECT_A::DEFAULT)
    }
    #[doc = "Byte 0 B/G(MSB)Byte 1 G(LSB)/RByte 2 B/G(MSB)Byte 3 G(LSB)/R"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(RGB_CFGSELECT_A::MODE1)
    }
    #[doc = "Byte 0 G(LSB)/RByte 1 B/G(MSB)Byte 2 G(LSB)/RByte 3 B/G(MSB)"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(RGB_CFGSELECT_A::MODE2)
    }
    #[doc = "Byte 0 G(LSB)/BByte 1 R/G(MSB)Byte 2 G(LSB)/BByte 3 R/G(MSB)"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(RGB_CFGSELECT_A::MODE3)
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
        GS_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&self) -> RGB_MODE_R {
        RGB_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&self) -> GRAYSCALE_R {
        GRAYSCALE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&self) -> RGB_SWAP_R {
        RGB_SWAP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&self) -> COL_SPACE_R {
        COL_SPACE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&self) -> IM_HSIZE_R {
        IM_HSIZE_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&self) -> YCC_SWAP_R {
        YCC_SWAP_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&self) -> RGB_CFG_R {
        RGB_CFG_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Vertical Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_vsize(&mut self) -> IM_VSIZE_W<0> {
        IM_VSIZE_W::new(self)
    }
    #[doc = "Bit 11 - Grayscale Pixel Format Mode"]
    #[inline(always)]
    pub fn gs_mode(&mut self) -> GS_MODE_W<11> {
        GS_MODE_W::new(self)
    }
    #[doc = "Bit 12 - RGB Input Mode"]
    #[inline(always)]
    pub fn rgb_mode(&mut self) -> RGB_MODE_W<12> {
        RGB_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Grayscale Mode Format Enable"]
    #[inline(always)]
    pub fn grayscale(&mut self) -> GRAYSCALE_W<13> {
        GRAYSCALE_W::new(self)
    }
    #[doc = "Bit 14 - RGB Format Swap Mode"]
    #[inline(always)]
    pub fn rgb_swap(&mut self) -> RGB_SWAP_W<14> {
        RGB_SWAP_W::new(self)
    }
    #[doc = "Bit 15 - Color Space for the Image Data"]
    #[inline(always)]
    pub fn col_space(&mut self) -> COL_SPACE_W<15> {
        COL_SPACE_W::new(self)
    }
    #[doc = "Bits 16:26 - Horizontal Size of the Image Sensor \\[0..2047\\]"]
    #[inline(always)]
    pub fn im_hsize(&mut self) -> IM_HSIZE_W<16> {
        IM_HSIZE_W::new(self)
    }
    #[doc = "Bits 28:29 - YCrCb Format Swap Mode"]
    #[inline(always)]
    pub fn ycc_swap(&mut self) -> YCC_SWAP_W<28> {
        YCC_SWAP_W::new(self)
    }
    #[doc = "Bits 30:31 - RGB Pixel Mapping Configuration"]
    #[inline(always)]
    pub fn rgb_cfg(&mut self) -> RGB_CFG_W<30> {
        RGB_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
