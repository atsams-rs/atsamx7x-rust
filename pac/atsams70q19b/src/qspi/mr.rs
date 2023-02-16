#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMM` reader - Serial Memory Mode"]
pub type SMM_R = crate::BitReader<SMMSELECT_A>;
#[doc = "Serial Memory Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMMSELECT_A {
    #[doc = "0: The QSPI is in SPI mode."]
    SPI = 0,
    #[doc = "1: The QSPI is in Serial Memory mode."]
    MEMORY = 1,
}
impl From<SMMSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SMMSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMMSELECT_A {
        match self.bits {
            false => SMMSELECT_A::SPI,
            true => SMMSELECT_A::MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == SMMSELECT_A::SPI
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == SMMSELECT_A::MEMORY
    }
}
#[doc = "Field `SMM` writer - Serial Memory Mode"]
pub type SMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, SMMSELECT_A, O>;
impl<'a, const O: u8> SMM_W<'a, O> {
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(SMMSELECT_A::SPI)
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn memory(self) -> &'a mut W {
        self.variant(SMMSELECT_A::MEMORY)
    }
}
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LLB_R = crate::BitReader<LLBSELECT_A>;
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LLBSELECT_A {
    #[doc = "0: Local loopback path disabled."]
    DISABLED = 0,
    #[doc = "1: Local loopback path enabled."]
    ENABLED = 1,
}
impl From<LLBSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LLBSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLBSELECT_A {
        match self.bits {
            false => LLBSELECT_A::DISABLED,
            true => LLBSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LLBSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LLBSELECT_A::ENABLED
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, LLBSELECT_A, O>;
impl<'a, const O: u8> LLB_W<'a, O> {
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LLBSELECT_A::DISABLED)
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LLBSELECT_A::ENABLED)
    }
}
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WDRBT_R = crate::BitReader<WDRBTSELECT_A>;
#[doc = "Wait Data Read Before Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDRBTSELECT_A {
    #[doc = "0: No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    DISABLED = 0,
    #[doc = "1: In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    ENABLED = 1,
}
impl From<WDRBTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WDRBTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDRBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDRBTSELECT_A {
        match self.bits {
            false => WDRBTSELECT_A::DISABLED,
            true => WDRBTSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDRBTSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDRBTSELECT_A::ENABLED
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WDRBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, WDRBTSELECT_A, O>;
impl<'a, const O: u8> WDRBT_W<'a, O> {
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDRBTSELECT_A::DISABLED)
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDRBTSELECT_A::ENABLED)
    }
}
#[doc = "Field `CSMODE` reader - Chip Select Mode"]
pub type CSMODE_R = crate::FieldReader<u8, CSMODESELECT_A>;
#[doc = "Chip Select Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSMODESELECT_A {
    #[doc = "0: The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    NOT_RELOADED = 0,
    #[doc = "1: The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    LASTXFER = 1,
    #[doc = "2: The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY = 2,
}
impl From<CSMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CSMODESELECT_A) -> Self {
        variant as _
    }
}
impl CSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSMODESELECT_A> {
        match self.bits {
            0 => Some(CSMODESELECT_A::NOT_RELOADED),
            1 => Some(CSMODESELECT_A::LASTXFER),
            2 => Some(CSMODESELECT_A::SYSTEMATICALLY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RELOADED`"]
    #[inline(always)]
    pub fn is_not_reloaded(&self) -> bool {
        *self == CSMODESELECT_A::NOT_RELOADED
    }
    #[doc = "Checks if the value of the field is `LASTXFER`"]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        *self == CSMODESELECT_A::LASTXFER
    }
    #[doc = "Checks if the value of the field is `SYSTEMATICALLY`"]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        *self == CSMODESELECT_A::SYSTEMATICALLY
    }
}
#[doc = "Field `CSMODE` writer - Chip Select Mode"]
pub type CSMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, CSMODESELECT_A, 2, O>;
impl<'a, const O: u8> CSMODE_W<'a, O> {
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn not_reloaded(self) -> &'a mut W {
        self.variant(CSMODESELECT_A::NOT_RELOADED)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut W {
        self.variant(CSMODESELECT_A::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut W {
        self.variant(CSMODESELECT_A::SYSTEMATICALLY)
    }
}
#[doc = "Field `NBBITS` reader - Number Of Bits Per Transfer"]
pub type NBBITS_R = crate::FieldReader<u8, NBBITSSELECT_A>;
#[doc = "Number Of Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBBITSSELECT_A {
    #[doc = "0: 8 bits for transfer"]
    _8_BIT = 0,
    #[doc = "8: 16 bits for transfer"]
    _16_BIT = 8,
}
impl From<NBBITSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NBBITSSELECT_A) -> Self {
        variant as _
    }
}
impl NBBITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NBBITSSELECT_A> {
        match self.bits {
            0 => Some(NBBITSSELECT_A::_8_BIT),
            8 => Some(NBBITSSELECT_A::_16_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == NBBITSSELECT_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == NBBITSSELECT_A::_16_BIT
    }
}
#[doc = "Field `NBBITS` writer - Number Of Bits Per Transfer"]
pub type NBBITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, NBBITSSELECT_A, 4, O>;
impl<'a, const O: u8> NBBITS_W<'a, O> {
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(NBBITSSELECT_A::_8_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(NBBITSSELECT_A::_16_BIT)
    }
}
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYCS` reader - Minimum Inactive QCS Delay"]
pub type DLYCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYCS` writer - Minimum Inactive QCS Delay"]
pub type DLYCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&self) -> SMM_R {
        SMM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&self) -> NBBITS_R {
        NBBITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DLYCS_R {
        DLYCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&mut self) -> SMM_W<0> {
        SMM_W::new(self)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W<1> {
        LLB_W::new(self)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W<2> {
        WDRBT_W::new(self)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&mut self) -> CSMODE_W<4> {
        CSMODE_W::new(self)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&mut self) -> NBBITS_W<8> {
        NBBITS_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DLYBCT_W<16> {
        DLYBCT_W::new(self)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&mut self) -> DLYCS_W<24> {
        DLYCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
