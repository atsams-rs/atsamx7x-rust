#[doc = "Register `DEVCTRL` reader"]
pub struct R(crate::R<DEVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCTRL` writer"]
pub struct W(crate::W<DEVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCTRL_SPEC>;
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
impl From<crate::W<DEVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UADD` reader - USB Address"]
pub type UADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UADD` writer - USB Address"]
pub type UADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVCTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADDEN` reader - Address Enable"]
pub type ADDEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDEN` writer - Address Enable"]
pub type ADDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Field `DETACH` reader - Detach"]
pub type DETACH_R = crate::BitReader<bool>;
#[doc = "Field `DETACH` writer - Detach"]
pub type DETACH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Field `RMWKUP` reader - Remote Wake-Up"]
pub type RMWKUP_R = crate::BitReader<bool>;
#[doc = "Field `RMWKUP` writer - Remote Wake-Up"]
pub type RMWKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Mode Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDCONF_A {
    #[doc = "0: The peripheral starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the host is high-speed-capable."]
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
    crate::FieldWriter<'a, u32, DEVCTRL_SPEC, u8, SPDCONF_A, 2, O>;
impl<'a, const O: u8> SPDCONF_W<'a, O> {
    #[doc = "The peripheral starts in Full-speed mode and performs a high-speed reset to switch to High-speed mode if the host is high-speed-capable."]
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
#[doc = "Field `LS` reader - Low-Speed Mode Force"]
pub type LS_R = crate::BitReader<bool>;
#[doc = "Field `LS` writer - Low-Speed Mode Force"]
pub type LS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Field `TSTJ` reader - Test mode J"]
pub type TSTJ_R = crate::BitReader<bool>;
#[doc = "Field `TSTJ` writer - Test mode J"]
pub type TSTJ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Field `TSTK` reader - Test mode K"]
pub type TSTK_R = crate::BitReader<bool>;
#[doc = "Field `TSTK` writer - Test mode K"]
pub type TSTK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Field `TSTPCKT` reader - Test packet mode"]
pub type TSTPCKT_R = crate::BitReader<bool>;
#[doc = "Field `TSTPCKT` writer - Test packet mode"]
pub type TSTPCKT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
#[doc = "Field `OPMODE2` reader - Specific Operational mode"]
pub type OPMODE2_R = crate::BitReader<bool>;
#[doc = "Field `OPMODE2` writer - Specific Operational mode"]
pub type OPMODE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn uadd(&self) -> UADD_R {
        UADD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    pub fn adden(&self) -> ADDEN_R {
        ADDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    pub fn detach(&self) -> DETACH_R {
        DETACH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    pub fn rmwkup(&self) -> RMWKUP_R {
        RMWKUP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&self) -> SPDCONF_R {
        SPDCONF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Low-Speed Mode Force"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&self) -> TSTJ_R {
        TSTJ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&self) -> TSTK_R {
        TSTK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&self) -> TSTPCKT_R {
        TSTPCKT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    pub fn opmode2(&self) -> OPMODE2_R {
        OPMODE2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Address"]
    #[inline(always)]
    pub fn uadd(&mut self) -> UADD_W<0> {
        UADD_W::new(self)
    }
    #[doc = "Bit 7 - Address Enable"]
    #[inline(always)]
    pub fn adden(&mut self) -> ADDEN_W<7> {
        ADDEN_W::new(self)
    }
    #[doc = "Bit 8 - Detach"]
    #[inline(always)]
    pub fn detach(&mut self) -> DETACH_W<8> {
        DETACH_W::new(self)
    }
    #[doc = "Bit 9 - Remote Wake-Up"]
    #[inline(always)]
    pub fn rmwkup(&mut self) -> RMWKUP_W<9> {
        RMWKUP_W::new(self)
    }
    #[doc = "Bits 10:11 - Mode Configuration"]
    #[inline(always)]
    pub fn spdconf(&mut self) -> SPDCONF_W<10> {
        SPDCONF_W::new(self)
    }
    #[doc = "Bit 12 - Low-Speed Mode Force"]
    #[inline(always)]
    pub fn ls(&mut self) -> LS_W<12> {
        LS_W::new(self)
    }
    #[doc = "Bit 13 - Test mode J"]
    #[inline(always)]
    pub fn tstj(&mut self) -> TSTJ_W<13> {
        TSTJ_W::new(self)
    }
    #[doc = "Bit 14 - Test mode K"]
    #[inline(always)]
    pub fn tstk(&mut self) -> TSTK_W<14> {
        TSTK_W::new(self)
    }
    #[doc = "Bit 15 - Test packet mode"]
    #[inline(always)]
    pub fn tstpckt(&mut self) -> TSTPCKT_W<15> {
        TSTPCKT_W::new(self)
    }
    #[doc = "Bit 16 - Specific Operational mode"]
    #[inline(always)]
    pub fn opmode2(&mut self) -> OPMODE2_W<16> {
        OPMODE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devctrl](index.html) module"]
pub struct DEVCTRL_SPEC;
impl crate::RegisterSpec for DEVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devctrl::R](R) reader structure"]
impl crate::Readable for DEVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devctrl::W](W) writer structure"]
impl crate::Writable for DEVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVCTRL to value 0"]
impl crate::Resettable for DEVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
