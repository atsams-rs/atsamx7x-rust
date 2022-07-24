#[doc = "Register `HSTCTRL` reader"]
pub struct R(crate::R<HSTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTCTRL` writer"]
pub struct W(crate::W<HSTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTCTRL_SPEC>;
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
impl From<crate::W<HSTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFE` reader - Start of Frame Generation Enable"]
pub type SOFE_R = crate::BitReader<bool>;
#[doc = "Field `SOFE` writer - Start of Frame Generation Enable"]
pub type SOFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTCTRL_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Send USB Reset"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Send USB Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTCTRL_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - Send USB Resume"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - Send USB Resume"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTCTRL_SPEC, bool, O>;
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    NORMAL = 0,
    #[doc = "1: For a better consumption, if high speed is not needed."]
    LOW_POWER = 1,
}
impl From<SPDCONF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDCONF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPDCONF` reader - Mode Configuration"]
pub type SPDCONF_R = crate::FieldReader<u8, SPDCONF_A>;
impl SPDCONF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPDCONF_A> {
        match self.bits {
            0 => Some(SPDCONF_A::NORMAL),
            1 => Some(SPDCONF_A::LOW_POWER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SPDCONF_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == SPDCONF_A::LOW_POWER
    }
}
#[doc = "Field `SPDCONF` writer - Mode Configuration"]
pub type SPDCONF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HSTCTRL_SPEC, u8, SPDCONF_A, 2, O>;
impl<'a, const O: u8> SPDCONF_W<'a, O> {
    #[doc = "The host starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the downstream peripheral is high-speed capable."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SPDCONF_A::NORMAL)
    }
    #[doc = "For a better consumption, if high speed is not needed."]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(SPDCONF_A::LOW_POWER)
    }
}
impl R {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Start of Frame Generation Enable"]
    #[inline(always)]
    pub fn sofe(&mut self) -> SOFE_W<8> {
        SOFE_W::new(self)
    }
    #[doc = "Bit 9 - Send USB Reset"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<9> {
        RESET_W::new(self)
    }
    #[doc = "Bit 10 - Send USB Resume"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W<10> {
        RESUME_W::new(self)
    }
    #[doc = "Bits 12:13 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W<12> {
        SPDCONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstctrl](index.html) module"]
pub struct HSTCTRL_SPEC;
impl crate::RegisterSpec for HSTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstctrl::R](R) reader structure"]
impl crate::Readable for HSTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstctrl::W](W) writer structure"]
impl crate::Writable for HSTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTCTRL to value 0"]
impl crate::Resettable for HSTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
