#[doc = "Register `USBHS_TSTA2` reader"]
pub struct R(crate::R<USBHS_TSTA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBHS_TSTA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBHS_TSTA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBHS_TSTA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBHS_TSTA2` writer"]
pub struct W(crate::W<USBHS_TSTA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBHS_TSTA2_SPEC>;
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
impl From<crate::W<USBHS_TSTA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBHS_TSTA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FullDetachEn` reader - Full Detach Enable"]
pub struct FULLDETACHEN_R(crate::FieldReader<bool, bool>);
impl FULLDETACHEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FULLDETACHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULLDETACHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FullDetachEn` writer - Full Detach Enable"]
pub struct FULLDETACHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDETACHEN_W<'a> {
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
#[doc = "Field `HSSerialMode` reader - HS Serial Mode"]
pub struct HSSERIALMODE_R(crate::FieldReader<bool, bool>);
impl HSSERIALMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSSERIALMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSSERIALMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSSerialMode` writer - HS Serial Mode"]
pub struct HSSERIALMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSSERIALMODE_W<'a> {
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
#[doc = "Field `LoopBackMode` reader - Loop-back Mode"]
pub struct LOOPBACKMODE_R(crate::FieldReader<bool, bool>);
impl LOOPBACKMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACKMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACKMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LoopBackMode` writer - Loop-back Mode"]
pub struct LOOPBACKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACKMODE_W<'a> {
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
#[doc = "Field `DisableGatedClock` reader - Disable Gated Clock"]
pub struct DISABLEGATEDCLOCK_R(crate::FieldReader<bool, bool>);
impl DISABLEGATEDCLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLEGATEDCLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLEGATEDCLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DisableGatedClock` writer - Disable Gated Clock"]
pub struct DISABLEGATEDCLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLEGATEDCLOCK_W<'a> {
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
#[doc = "Field `ForceSuspendMTo1` reader - Force SuspendM to 1"]
pub struct FORCESUSPENDMTO1_R(crate::FieldReader<bool, bool>);
impl FORCESUSPENDMTO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCESUSPENDMTO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCESUSPENDMTO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ForceSuspendMTo1` writer - Force SuspendM to 1"]
pub struct FORCESUSPENDMTO1_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCESUSPENDMTO1_W<'a> {
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
#[doc = "Field `ByPassDpll` reader - Bypass DPLL"]
pub struct BYPASSDPLL_R(crate::FieldReader<bool, bool>);
impl BYPASSDPLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYPASSDPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASSDPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ByPassDpll` writer - Bypass DPLL"]
pub struct BYPASSDPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASSDPLL_W<'a> {
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
#[doc = "Field `HostHSDisconnectDisable` reader - Host HS Disconnect Disable"]
pub struct HOSTHSDISCONNECTDISABLE_R(crate::FieldReader<bool, bool>);
impl HOSTHSDISCONNECTDISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOSTHSDISCONNECTDISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOSTHSDISCONNECTDISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HostHSDisconnectDisable` writer - Host HS Disconnect Disable"]
pub struct HOSTHSDISCONNECTDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOSTHSDISCONNECTDISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ForceHSRst_50ms` reader - Force HS Reset to 50 ms"]
pub struct FORCEHSRST_50MS_R(crate::FieldReader<bool, bool>);
impl FORCEHSRST_50MS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCEHSRST_50MS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCEHSRST_50MS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ForceHSRst_50ms` writer - Force HS Reset to 50 ms"]
pub struct FORCEHSRST_50MS_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEHSRST_50MS_W<'a> {
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
#[doc = "Field `RemovePUWhenTX` reader - Remove Pull-up When TX"]
pub struct REMOVEPUWHENTX_R(crate::FieldReader<bool, bool>);
impl REMOVEPUWHENTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REMOVEPUWHENTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMOVEPUWHENTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RemovePUWhenTX` writer - Remove Pull-up When TX"]
pub struct REMOVEPUWHENTX_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVEPUWHENTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Full Detach Enable"]
    #[inline(always)]
    pub fn full_detach_en(&self) -> FULLDETACHEN_R {
        FULLDETACHEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HS Serial Mode"]
    #[inline(always)]
    pub fn hsserial_mode(&self) -> HSSERIALMODE_R {
        HSSERIALMODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Loop-back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&self) -> LOOPBACKMODE_R {
        LOOPBACKMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable Gated Clock"]
    #[inline(always)]
    pub fn disable_gated_clock(&self) -> DISABLEGATEDCLOCK_R {
        DISABLEGATEDCLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force SuspendM to 1"]
    #[inline(always)]
    pub fn force_suspend_mto1(&self) -> FORCESUSPENDMTO1_R {
        FORCESUSPENDMTO1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bypass DPLL"]
    #[inline(always)]
    pub fn by_pass_dpll(&self) -> BYPASSDPLL_R {
        BYPASSDPLL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host HS Disconnect Disable"]
    #[inline(always)]
    pub fn host_hsdisconnect_disable(&self) -> HOSTHSDISCONNECTDISABLE_R {
        HOSTHSDISCONNECTDISABLE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force HS Reset to 50 ms"]
    #[inline(always)]
    pub fn force_hsrst_50ms(&self) -> FORCEHSRST_50MS_R {
        FORCEHSRST_50MS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Remove Pull-up When TX"]
    #[inline(always)]
    pub fn remove_puwhen_tx(&self) -> REMOVEPUWHENTX_R {
        REMOVEPUWHENTX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Full Detach Enable"]
    #[inline(always)]
    pub fn full_detach_en(&mut self) -> FULLDETACHEN_W {
        FULLDETACHEN_W { w: self }
    }
    #[doc = "Bit 1 - HS Serial Mode"]
    #[inline(always)]
    pub fn hsserial_mode(&mut self) -> HSSERIALMODE_W {
        HSSERIALMODE_W { w: self }
    }
    #[doc = "Bit 2 - Loop-back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&mut self) -> LOOPBACKMODE_W {
        LOOPBACKMODE_W { w: self }
    }
    #[doc = "Bit 3 - Disable Gated Clock"]
    #[inline(always)]
    pub fn disable_gated_clock(&mut self) -> DISABLEGATEDCLOCK_W {
        DISABLEGATEDCLOCK_W { w: self }
    }
    #[doc = "Bit 4 - Force SuspendM to 1"]
    #[inline(always)]
    pub fn force_suspend_mto1(&mut self) -> FORCESUSPENDMTO1_W {
        FORCESUSPENDMTO1_W { w: self }
    }
    #[doc = "Bit 5 - Bypass DPLL"]
    #[inline(always)]
    pub fn by_pass_dpll(&mut self) -> BYPASSDPLL_W {
        BYPASSDPLL_W { w: self }
    }
    #[doc = "Bit 6 - Host HS Disconnect Disable"]
    #[inline(always)]
    pub fn host_hsdisconnect_disable(&mut self) -> HOSTHSDISCONNECTDISABLE_W {
        HOSTHSDISCONNECTDISABLE_W { w: self }
    }
    #[doc = "Bit 7 - Force HS Reset to 50 ms"]
    #[inline(always)]
    pub fn force_hsrst_50ms(&mut self) -> FORCEHSRST_50MS_W {
        FORCEHSRST_50MS_W { w: self }
    }
    #[doc = "Bit 9 - Remove Pull-up When TX"]
    #[inline(always)]
    pub fn remove_puwhen_tx(&mut self) -> REMOVEPUWHENTX_W {
        REMOVEPUWHENTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Test A2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbhs_tsta2](index.html) module"]
pub struct USBHS_TSTA2_SPEC;
impl crate::RegisterSpec for USBHS_TSTA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbhs_tsta2::R](R) reader structure"]
impl crate::Readable for USBHS_TSTA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbhs_tsta2::W](W) writer structure"]
impl crate::Writable for USBHS_TSTA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBHS_TSTA2 to value 0"]
impl crate::Resettable for USBHS_TSTA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
