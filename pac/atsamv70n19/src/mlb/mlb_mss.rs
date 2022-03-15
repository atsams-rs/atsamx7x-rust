#[doc = "Register `MLB_MSS` reader"]
pub struct R(crate::R<MLB_MSS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_MSS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_MSS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_MSS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_MSS` writer"]
pub struct W(crate::W<MLB_MSS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_MSS_SPEC>;
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
impl From<crate::W<MLB_MSS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_MSS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTSYSCMD` reader - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct RSTSYSCMD_R(crate::FieldReader<bool, bool>);
impl RSTSYSCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTSYSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTSYSCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTSYSCMD` writer - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct RSTSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTSYSCMD_W<'a> {
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
#[doc = "Field `LKSYSCMD` reader - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct LKSYSCMD_R(crate::FieldReader<bool, bool>);
impl LKSYSCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LKSYSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LKSYSCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LKSYSCMD` writer - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct LKSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> LKSYSCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ULKSYSCMD` reader - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct ULKSYSCMD_R(crate::FieldReader<bool, bool>);
impl ULKSYSCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULKSYSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULKSYSCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULKSYSCMD` writer - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct ULKSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ULKSYSCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CSSYSCMD` reader - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct CSSYSCMD_R(crate::FieldReader<bool, bool>);
impl CSSYSCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CSSYSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSYSCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSYSCMD` writer - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct CSSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSYSCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SWSYSCMD` reader - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct SWSYSCMD_R(crate::FieldReader<bool, bool>);
impl SWSYSCMD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWSYSCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWSYSCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWSYSCMD` writer - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
pub struct SWSYSCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWSYSCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SERVREQ` reader - Service Request Enabled"]
pub struct SERVREQ_R(crate::FieldReader<bool, bool>);
impl SERVREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SERVREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SERVREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SERVREQ` writer - Service Request Enabled"]
pub struct SERVREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SERVREQ_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&self) -> RSTSYSCMD_R {
        RSTSYSCMD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&self) -> LKSYSCMD_R {
        LKSYSCMD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&self) -> ULKSYSCMD_R {
        ULKSYSCMD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&self) -> CSSYSCMD_R {
        CSSYSCMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&self) -> SWSYSCMD_R {
        SWSYSCMD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&self) -> SERVREQ_R {
        SERVREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn rstsyscmd(&mut self) -> RSTSYSCMD_W {
        RSTSYSCMD_W { w: self }
    }
    #[doc = "Bit 1 - Network Lock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lksyscmd(&mut self) -> LKSYSCMD_W {
        LKSYSCMD_W { w: self }
    }
    #[doc = "Bit 2 - Network Unlock System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn ulksyscmd(&mut self) -> ULKSYSCMD_W {
        ULKSYSCMD_W { w: self }
    }
    #[doc = "Bit 3 - Channel Scan System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn cssyscmd(&mut self) -> CSSYSCMD_W {
        CSSYSCMD_W { w: self }
    }
    #[doc = "Bit 4 - Software System Command Detected in the System Quadlet (cleared by writing a 0)"]
    #[inline(always)]
    pub fn swsyscmd(&mut self) -> SWSYSCMD_W {
        SWSYSCMD_W { w: self }
    }
    #[doc = "Bit 5 - Service Request Enabled"]
    #[inline(always)]
    pub fn servreq(&mut self) -> SERVREQ_W {
        SERVREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_mss](index.html) module"]
pub struct MLB_MSS_SPEC;
impl crate::RegisterSpec for MLB_MSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_mss::R](R) reader structure"]
impl crate::Readable for MLB_MSS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_mss::W](W) writer structure"]
impl crate::Writable for MLB_MSS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_MSS to value 0"]
impl crate::Resettable for MLB_MSS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
