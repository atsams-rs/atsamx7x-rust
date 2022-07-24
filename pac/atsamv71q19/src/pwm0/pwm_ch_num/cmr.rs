#[doc = "Register `CMR` reader"]
pub struct R(crate::R<CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR` writer"]
pub struct W(crate::W<CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_SPEC>;
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
impl From<crate::W<CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Pre-scaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPRE_A {
    #[doc = "0: Peripheral clock"]
    MCK = 0,
    #[doc = "1: Peripheral clock/2"]
    MCK_DIV_2 = 1,
    #[doc = "2: Peripheral clock/4"]
    MCK_DIV_4 = 2,
    #[doc = "3: Peripheral clock/8"]
    MCK_DIV_8 = 3,
    #[doc = "4: Peripheral clock/16"]
    MCK_DIV_16 = 4,
    #[doc = "5: Peripheral clock/32"]
    MCK_DIV_32 = 5,
    #[doc = "6: Peripheral clock/64"]
    MCK_DIV_64 = 6,
    #[doc = "7: Peripheral clock/128"]
    MCK_DIV_128 = 7,
    #[doc = "8: Peripheral clock/256"]
    MCK_DIV_256 = 8,
    #[doc = "9: Peripheral clock/512"]
    MCK_DIV_512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    MCK_DIV_1024 = 10,
    #[doc = "11: Clock A"]
    CLKA = 11,
    #[doc = "12: Clock B"]
    CLKB = 12,
}
impl From<CPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPRE` reader - Channel Pre-scaler"]
pub type CPRE_R = crate::FieldReader<u8, CPRE_A>;
impl CPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPRE_A> {
        match self.bits {
            0 => Some(CPRE_A::MCK),
            1 => Some(CPRE_A::MCK_DIV_2),
            2 => Some(CPRE_A::MCK_DIV_4),
            3 => Some(CPRE_A::MCK_DIV_8),
            4 => Some(CPRE_A::MCK_DIV_16),
            5 => Some(CPRE_A::MCK_DIV_32),
            6 => Some(CPRE_A::MCK_DIV_64),
            7 => Some(CPRE_A::MCK_DIV_128),
            8 => Some(CPRE_A::MCK_DIV_256),
            9 => Some(CPRE_A::MCK_DIV_512),
            10 => Some(CPRE_A::MCK_DIV_1024),
            11 => Some(CPRE_A::CLKA),
            12 => Some(CPRE_A::CLKB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CPRE_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        *self == CPRE_A::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        *self == CPRE_A::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        *self == CPRE_A::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        *self == CPRE_A::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        *self == CPRE_A::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        *self == CPRE_A::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        *self == CPRE_A::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        *self == CPRE_A::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        *self == CPRE_A::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == CPRE_A::MCK_DIV_1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == CPRE_A::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == CPRE_A::CLKB
    }
}
#[doc = "Field `CPRE` writer - Channel Pre-scaler"]
pub type CPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMR_SPEC, u8, CPRE_A, 4, O>;
impl<'a, const O: u8> CPRE_W<'a, O> {
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPRE_A::MCK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPRE_A::MCK_DIV_1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPRE_A::CLKA)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPRE_A::CLKB)
    }
}
#[doc = "Field `CALG` reader - Channel Alignment"]
pub type CALG_R = crate::BitReader<bool>;
#[doc = "Field `CALG` writer - Channel Alignment"]
pub type CALG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - Channel Polarity"]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Channel Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `CES` reader - Counter Event Selection"]
pub type CES_R = crate::BitReader<bool>;
#[doc = "Field `CES` writer - Counter Event Selection"]
pub type CES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `UPDS` reader - Update Selection"]
pub type UPDS_R = crate::BitReader<bool>;
#[doc = "Field `UPDS` writer - Update Selection"]
pub type UPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `DPOLI` reader - Disabled Polarity Inverted"]
pub type DPOLI_R = crate::BitReader<bool>;
#[doc = "Field `DPOLI` writer - Disabled Polarity Inverted"]
pub type DPOLI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `TCTS` reader - Timer Counter Trigger Selection"]
pub type TCTS_R = crate::BitReader<bool>;
#[doc = "Field `TCTS` writer - Timer Counter Trigger Selection"]
pub type TCTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `DTE` reader - Dead-Time Generator Enable"]
pub type DTE_R = crate::BitReader<bool>;
#[doc = "Field `DTE` writer - Dead-Time Generator Enable"]
pub type DTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `DTHI` reader - Dead-Time PWMHx Output Inverted"]
pub type DTHI_R = crate::BitReader<bool>;
#[doc = "Field `DTHI` writer - Dead-Time PWMHx Output Inverted"]
pub type DTHI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `DTLI` reader - Dead-Time PWMLx Output Inverted"]
pub type DTLI_R = crate::BitReader<bool>;
#[doc = "Field `DTLI` writer - Dead-Time PWMLx Output Inverted"]
pub type DTLI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `PPM` reader - Push-Pull Mode"]
pub type PPM_R = crate::BitReader<bool>;
#[doc = "Field `PPM` writer - Push-Pull Mode"]
pub type PPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&self) -> UPDS_R {
        UPDS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&self) -> DPOLI_R {
        DPOLI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&self) -> TCTS_R {
        TCTS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DTHI_R {
        DTHI_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DTLI_R {
        DTLI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&self) -> PPM_R {
        PPM_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&mut self) -> CPRE_W<0> {
        CPRE_W::new(self)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&mut self) -> CALG_W<8> {
        CALG_W::new(self)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<9> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W<10> {
        CES_W::new(self)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&mut self) -> UPDS_W<11> {
        UPDS_W::new(self)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&mut self) -> DPOLI_W<12> {
        DPOLI_W::new(self)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&mut self) -> TCTS_W<13> {
        TCTS_W::new(self)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> DTE_W<16> {
        DTE_W::new(self)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&mut self) -> DTHI_W<17> {
        DTHI_W::new(self)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&mut self) -> DTLI_W<18> {
        DTLI_W::new(self)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&mut self) -> PPM_W<19> {
        PPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](index.html) module"]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr::R](R) reader structure"]
impl crate::Readable for CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr::W](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
