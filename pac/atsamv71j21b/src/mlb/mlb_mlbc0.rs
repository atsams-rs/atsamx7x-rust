#[doc = "Register `MLB_MLBC0` reader"]
pub struct R(crate::R<MLB_MLBC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_MLBC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_MLBC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_MLBC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_MLBC0` writer"]
pub struct W(crate::W<MLB_MLBC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_MLBC0_SPEC>;
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
impl From<crate::W<MLB_MLBC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_MLBC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MLBEN` reader - MediaLB Enable"]
pub struct MLBEN_R(crate::FieldReader<bool, bool>);
impl MLBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MLBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MLBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MLBEN` writer - MediaLB Enable"]
pub struct MLBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MLBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "MLBCLK (MediaLB clock) Speed Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MLBCLK_A {
    #[doc = "0: 256xFs (for MLBPEN = 0)"]
    _256_FS = 0,
    #[doc = "1: 512xFs (for MLBPEN = 0)"]
    _512_FS = 1,
    #[doc = "2: 1024xFs (for MLBPEN = 0)"]
    _1024_FS = 2,
}
impl From<MLBCLK_A> for u8 {
    #[inline(always)]
    fn from(variant: MLBCLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MLBCLK` reader - MLBCLK (MediaLB clock) Speed Select"]
pub struct MLBCLK_R(crate::FieldReader<u8, MLBCLK_A>);
impl MLBCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MLBCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MLBCLK_A> {
        match self.bits {
            0 => Some(MLBCLK_A::_256_FS),
            1 => Some(MLBCLK_A::_512_FS),
            2 => Some(MLBCLK_A::_1024_FS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_256_FS`"]
    #[inline(always)]
    pub fn is_256_fs(&self) -> bool {
        **self == MLBCLK_A::_256_FS
    }
    #[doc = "Checks if the value of the field is `_512_FS`"]
    #[inline(always)]
    pub fn is_512_fs(&self) -> bool {
        **self == MLBCLK_A::_512_FS
    }
    #[doc = "Checks if the value of the field is `_1024_FS`"]
    #[inline(always)]
    pub fn is_1024_fs(&self) -> bool {
        **self == MLBCLK_A::_1024_FS
    }
}
impl core::ops::Deref for MLBCLK_R {
    type Target = crate::FieldReader<u8, MLBCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MLBCLK` writer - MLBCLK (MediaLB clock) Speed Select"]
pub struct MLBCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MLBCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MLBCLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _256_fs(self) -> &'a mut W {
        self.variant(MLBCLK_A::_256_FS)
    }
    #[doc = "512xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _512_fs(self) -> &'a mut W {
        self.variant(MLBCLK_A::_512_FS)
    }
    #[doc = "1024xFs (for MLBPEN = 0)"]
    #[inline(always)]
    pub fn _1024_fs(self) -> &'a mut W {
        self.variant(MLBCLK_A::_1024_FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `ZERO` reader - Must be Written to 0"]
pub struct ZERO_R(crate::FieldReader<bool, bool>);
impl ZERO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ZERO` writer - Must be Written to 0"]
pub struct ZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> ZERO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MLBLK` reader - MediaLB Lock Status (read-only)"]
pub struct MLBLK_R(crate::FieldReader<bool, bool>);
impl MLBLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MLBLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MLBLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MLBLK` writer - MediaLB Lock Status (read-only)"]
pub struct MLBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> MLBLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ASYRETRY` reader - Asynchronous Tx Packet Retry"]
pub struct ASYRETRY_R(crate::FieldReader<bool, bool>);
impl ASYRETRY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ASYRETRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASYRETRY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASYRETRY` writer - Asynchronous Tx Packet Retry"]
pub struct ASYRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYRETRY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CTLRETRY` reader - Control Tx Packet Retry"]
pub struct CTLRETRY_R(crate::FieldReader<bool, bool>);
impl CTLRETRY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTLRETRY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTLRETRY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLRETRY` writer - Control Tx Packet Retry"]
pub struct CTLRETRY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLRETRY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "The number of frames per sub-buffer for synchronous channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCNT_A {
    #[doc = "0: 1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    _1_FRAME = 0,
    #[doc = "1: 2 frames per sub-buffer"]
    _2_FRAMES = 1,
    #[doc = "2: 4 frames per sub-buffer"]
    _4_FRAMES = 2,
    #[doc = "3: 8 frames per sub-buffer"]
    _8_FRAMES = 3,
    #[doc = "4: 16 frames per sub-buffer"]
    _16_FRAMES = 4,
    #[doc = "5: 32 frames per sub-buffer"]
    _32_FRAMES = 5,
    #[doc = "6: 64 frames per sub-buffer"]
    _64_FRAMES = 6,
}
impl From<FCNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FCNT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCNT` reader - The number of frames per sub-buffer for synchronous channels"]
pub struct FCNT_R(crate::FieldReader<u8, FCNT_A>);
impl FCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCNT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FCNT_A> {
        match self.bits {
            0 => Some(FCNT_A::_1_FRAME),
            1 => Some(FCNT_A::_2_FRAMES),
            2 => Some(FCNT_A::_4_FRAMES),
            3 => Some(FCNT_A::_8_FRAMES),
            4 => Some(FCNT_A::_16_FRAMES),
            5 => Some(FCNT_A::_32_FRAMES),
            6 => Some(FCNT_A::_64_FRAMES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_FRAME`"]
    #[inline(always)]
    pub fn is_1_frame(&self) -> bool {
        **self == FCNT_A::_1_FRAME
    }
    #[doc = "Checks if the value of the field is `_2_FRAMES`"]
    #[inline(always)]
    pub fn is_2_frames(&self) -> bool {
        **self == FCNT_A::_2_FRAMES
    }
    #[doc = "Checks if the value of the field is `_4_FRAMES`"]
    #[inline(always)]
    pub fn is_4_frames(&self) -> bool {
        **self == FCNT_A::_4_FRAMES
    }
    #[doc = "Checks if the value of the field is `_8_FRAMES`"]
    #[inline(always)]
    pub fn is_8_frames(&self) -> bool {
        **self == FCNT_A::_8_FRAMES
    }
    #[doc = "Checks if the value of the field is `_16_FRAMES`"]
    #[inline(always)]
    pub fn is_16_frames(&self) -> bool {
        **self == FCNT_A::_16_FRAMES
    }
    #[doc = "Checks if the value of the field is `_32_FRAMES`"]
    #[inline(always)]
    pub fn is_32_frames(&self) -> bool {
        **self == FCNT_A::_32_FRAMES
    }
    #[doc = "Checks if the value of the field is `_64_FRAMES`"]
    #[inline(always)]
    pub fn is_64_frames(&self) -> bool {
        **self == FCNT_A::_64_FRAMES
    }
}
impl core::ops::Deref for FCNT_R {
    type Target = crate::FieldReader<u8, FCNT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCNT` writer - The number of frames per sub-buffer for synchronous channels"]
pub struct FCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCNT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 frame per sub-buffer (Operation is the same as Standard mode.)"]
    #[inline(always)]
    pub fn _1_frame(self) -> &'a mut W {
        self.variant(FCNT_A::_1_FRAME)
    }
    #[doc = "2 frames per sub-buffer"]
    #[inline(always)]
    pub fn _2_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_2_FRAMES)
    }
    #[doc = "4 frames per sub-buffer"]
    #[inline(always)]
    pub fn _4_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_4_FRAMES)
    }
    #[doc = "8 frames per sub-buffer"]
    #[inline(always)]
    pub fn _8_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_8_FRAMES)
    }
    #[doc = "16 frames per sub-buffer"]
    #[inline(always)]
    pub fn _16_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_16_FRAMES)
    }
    #[doc = "32 frames per sub-buffer"]
    #[inline(always)]
    pub fn _32_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_32_FRAMES)
    }
    #[doc = "64 frames per sub-buffer"]
    #[inline(always)]
    pub fn _64_frames(self) -> &'a mut W {
        self.variant(FCNT_A::_64_FRAMES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&self) -> MLBEN_R {
        MLBEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - MLBCLK (MediaLB clock) Speed Select"]
    #[inline(always)]
    pub fn mlbclk(&self) -> MLBCLK_R {
        MLBCLK_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&self) -> MLBLK_R {
        MLBLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&self) -> ASYRETRY_R {
        ASYRETRY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&self) -> CTLRETRY_R {
        CTLRETRY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&self) -> FCNT_R {
        FCNT_R::new(((self.bits >> 15) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - MediaLB Enable"]
    #[inline(always)]
    pub fn mlben(&mut self) -> MLBEN_W {
        MLBEN_W { w: self }
    }
    #[doc = "Bits 2:4 - MLBCLK (MediaLB clock) Speed Select"]
    #[inline(always)]
    pub fn mlbclk(&mut self) -> MLBCLK_W {
        MLBCLK_W { w: self }
    }
    #[doc = "Bit 5 - Must be Written to 0"]
    #[inline(always)]
    pub fn zero(&mut self) -> ZERO_W {
        ZERO_W { w: self }
    }
    #[doc = "Bit 7 - MediaLB Lock Status (read-only)"]
    #[inline(always)]
    pub fn mlblk(&mut self) -> MLBLK_W {
        MLBLK_W { w: self }
    }
    #[doc = "Bit 12 - Asynchronous Tx Packet Retry"]
    #[inline(always)]
    pub fn asyretry(&mut self) -> ASYRETRY_W {
        ASYRETRY_W { w: self }
    }
    #[doc = "Bit 14 - Control Tx Packet Retry"]
    #[inline(always)]
    pub fn ctlretry(&mut self) -> CTLRETRY_W {
        CTLRETRY_W { w: self }
    }
    #[doc = "Bits 15:17 - The number of frames per sub-buffer for synchronous channels"]
    #[inline(always)]
    pub fn fcnt(&mut self) -> FCNT_W {
        FCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_mlbc0](index.html) module"]
pub struct MLB_MLBC0_SPEC;
impl crate::RegisterSpec for MLB_MLBC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_mlbc0::R](R) reader structure"]
impl crate::Readable for MLB_MLBC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_mlbc0::W](W) writer structure"]
impl crate::Writable for MLB_MLBC0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_MLBC0 to value 0"]
impl crate::Resettable for MLB_MLBC0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
