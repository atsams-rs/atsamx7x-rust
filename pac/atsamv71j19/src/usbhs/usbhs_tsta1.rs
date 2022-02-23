#[doc = "Register `USBHS_TSTA1` reader"]
pub struct R(crate::R<USBHS_TSTA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_TSTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_TSTA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_TSTA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_TSTA1` writer"]
pub struct W(crate::W<USBHS_TSTA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_TSTA1_SPEC>;
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
impl From<crate::W<USBHS_TSTA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_TSTA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CounterA` reader - Counter A"]
pub struct COUNTERA_R(crate::FieldReader<u16, u16>);
impl COUNTERA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        COUNTERA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTERA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CounterA` writer - Counter A"]
pub struct COUNTERA_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTERA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
#[doc = "Field `LoadCntA` reader - Load CounterA"]
pub struct LOADCNTA_R(crate::FieldReader<bool, bool>);
impl LOADCNTA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOADCNTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOADCNTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoadCntA` writer - Load CounterA"]
pub struct LOADCNTA_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADCNTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CounterB` reader - Counter B"]
pub struct COUNTERB_R(crate::FieldReader<u8, u8>);
impl COUNTERB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COUNTERB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTERB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CounterB` writer - Counter B"]
pub struct COUNTERB_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTERB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `LoadCntB` reader - Load CounterB"]
pub struct LOADCNTB_R(crate::FieldReader<bool, bool>);
impl LOADCNTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOADCNTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOADCNTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoadCntB` writer - Load CounterB"]
pub struct LOADCNTB_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADCNTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `SOFCntMa1` reader - SOF Counter Max"]
pub struct SOFCNTMA1_R(crate::FieldReader<u8, u8>);
impl SOFCNTMA1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SOFCNTMA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFCNTMA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFCntMa1` writer - SOF Counter Max"]
pub struct SOFCNTMA1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFCNTMA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `LoadSOFCnt` reader - Load SOF Counter"]
pub struct LOADSOFCNT_R(crate::FieldReader<bool, bool>);
impl LOADSOFCNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOADSOFCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOADSOFCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoadSOFCnt` writer - Load SOF Counter"]
pub struct LOADSOFCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOADSOFCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Counter A"]
    #[inline(always)]
    pub fn counter_a(&self) -> COUNTERA_R {
        COUNTERA_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Load CounterA"]
    #[inline(always)]
    pub fn load_cnt_a(&self) -> LOADCNTA_R {
        LOADCNTA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - Counter B"]
    #[inline(always)]
    pub fn counter_b(&self) -> COUNTERB_R {
        COUNTERB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Load CounterB"]
    #[inline(always)]
    pub fn load_cnt_b(&self) -> LOADCNTB_R {
        LOADCNTB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - SOF Counter Max"]
    #[inline(always)]
    pub fn sofcnt_ma1(&self) -> SOFCNTMA1_R {
        SOFCNTMA1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Load SOF Counter"]
    #[inline(always)]
    pub fn load_sofcnt(&self) -> LOADSOFCNT_R {
        LOADSOFCNT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Counter A"]
    #[inline(always)]
    pub fn counter_a(&mut self) -> COUNTERA_W {
        COUNTERA_W { w: self }
    }
    #[doc = "Bit 15 - Load CounterA"]
    #[inline(always)]
    pub fn load_cnt_a(&mut self) -> LOADCNTA_W {
        LOADCNTA_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter B"]
    #[inline(always)]
    pub fn counter_b(&mut self) -> COUNTERB_W {
        COUNTERB_W { w: self }
    }
    #[doc = "Bit 23 - Load CounterB"]
    #[inline(always)]
    pub fn load_cnt_b(&mut self) -> LOADCNTB_W {
        LOADCNTB_W { w: self }
    }
    #[doc = "Bits 24:30 - SOF Counter Max"]
    #[inline(always)]
    pub fn sofcnt_ma1(&mut self) -> SOFCNTMA1_W {
        SOFCNTMA1_W { w: self }
    }
    #[doc = "Bit 31 - Load SOF Counter"]
    #[inline(always)]
    pub fn load_sofcnt(&mut self) -> LOADSOFCNT_W {
        LOADSOFCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Test A1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_tsta1](index.html) module"]
pub struct USBHS_TSTA1_SPEC;
impl crate::RegisterSpec for USBHS_TSTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_tsta1::R](R) reader structure"]
impl crate::Readable for USBHS_TSTA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_tsta1::W](W) writer structure"]
impl crate::Writable for USBHS_TSTA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_TSTA1 to value 0"]
impl crate::Resettable for USBHS_TSTA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
