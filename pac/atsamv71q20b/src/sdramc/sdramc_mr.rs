#[doc = "Register `SDRAMC_MR` reader"]
pub struct R(crate::R<SDRAMC_MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMC_MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMC_MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMC_MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMC_MR` writer"]
pub struct W(crate::W<SDRAMC_MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRAMC_MR_SPEC>;
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
impl From<crate::W<SDRAMC_MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRAMC_MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDRAMC Command Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    NORMAL = 0,
    #[doc = "1: The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    NOP = 1,
    #[doc = "2: The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    ALLBANKS_PRECHARGE = 2,
    #[doc = "3: The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    LOAD_MODEREG = 3,
    #[doc = "4: The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    AUTO_REFRESH = 4,
    #[doc = "5: The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    EXT_LOAD_MODEREG = 5,
    #[doc = "6: Deep Power-down mode. Enters Deep Power-down mode."]
    DEEP_POWERDOWN = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - SDRAMC Command Mode"]
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::NORMAL),
            1 => Some(MODE_A::NOP),
            2 => Some(MODE_A::ALLBANKS_PRECHARGE),
            3 => Some(MODE_A::LOAD_MODEREG),
            4 => Some(MODE_A::AUTO_REFRESH),
            5 => Some(MODE_A::EXT_LOAD_MODEREG),
            6 => Some(MODE_A::DEEP_POWERDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        **self == MODE_A::NOP
    }
    #[doc = "Checks if the value of the field is `ALLBANKS_PRECHARGE`"]
    #[inline(always)]
    pub fn is_allbanks_precharge(&self) -> bool {
        **self == MODE_A::ALLBANKS_PRECHARGE
    }
    #[doc = "Checks if the value of the field is `LOAD_MODEREG`"]
    #[inline(always)]
    pub fn is_load_modereg(&self) -> bool {
        **self == MODE_A::LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `AUTO_REFRESH`"]
    #[inline(always)]
    pub fn is_auto_refresh(&self) -> bool {
        **self == MODE_A::AUTO_REFRESH
    }
    #[doc = "Checks if the value of the field is `EXT_LOAD_MODEREG`"]
    #[inline(always)]
    pub fn is_ext_load_modereg(&self) -> bool {
        **self == MODE_A::EXT_LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_deep_powerdown(&self) -> bool {
        **self == MODE_A::DEEP_POWERDOWN
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - SDRAMC Command Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_A::NORMAL)
    }
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(MODE_A::NOP)
    }
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn allbanks_precharge(self) -> &'a mut W {
        self.variant(MODE_A::ALLBANKS_PRECHARGE)
    }
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn load_modereg(self) -> &'a mut W {
        self.variant(MODE_A::LOAD_MODEREG)
    }
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn auto_refresh(self) -> &'a mut W {
        self.variant(MODE_A::AUTO_REFRESH)
    }
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline(always)]
    pub fn ext_load_modereg(self) -> &'a mut W {
        self.variant(MODE_A::EXT_LOAD_MODEREG)
    }
    #[doc = "Deep Power-down mode. Enters Deep Power-down mode."]
    #[inline(always)]
    pub fn deep_powerdown(self) -> &'a mut W {
        self.variant(MODE_A::DEEP_POWERDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_mr](index.html) module"]
pub struct SDRAMC_MR_SPEC;
impl crate::RegisterSpec for SDRAMC_MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramc_mr::R](R) reader structure"]
impl crate::Readable for SDRAMC_MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdramc_mr::W](W) writer structure"]
impl crate::Writable for SDRAMC_MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRAMC_MR to value 0"]
impl crate::Resettable for SDRAMC_MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
