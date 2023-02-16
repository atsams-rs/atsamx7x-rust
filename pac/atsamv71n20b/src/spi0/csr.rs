#[doc = "Register `CSR[%s]` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR[%s]` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOLSELECT_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOLSELECT_A {
    #[doc = "0: Clock is low when inactive (CPOL=0)"]
    IDLE_LOW = 0,
    #[doc = "1: Clock is high when inactive (CPOL=1)"]
    IDLE_HIGH = 1,
}
impl From<CPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::IDLE_LOW,
            true => CPOLSELECT_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOLSELECT_A::IDLE_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, CPOLSELECT_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "Clock is low when inactive (CPOL=0)"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::IDLE_LOW)
    }
    #[doc = "Clock is high when inactive (CPOL=1)"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::IDLE_HIGH)
    }
}
#[doc = "Field `NCPHA` reader - Clock Phase"]
pub type NCPHA_R = crate::BitReader<NCPHASELECT_A>;
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCPHASELECT_A {
    #[doc = "1: Data is valid on clock leading edge (NCPHA=1)"]
    VALID_LEADING_EDGE = 1,
    #[doc = "0: Data is valid on clock trailing edge (NCPHA=0)"]
    VALID_TRAILING_EDGE = 0,
}
impl From<NCPHASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NCPHASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NCPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCPHASELECT_A {
        match self.bits {
            true => NCPHASELECT_A::VALID_LEADING_EDGE,
            false => NCPHASELECT_A::VALID_TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `VALID_LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_valid_leading_edge(&self) -> bool {
        *self == NCPHASELECT_A::VALID_LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `VALID_TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_valid_trailing_edge(&self) -> bool {
        *self == NCPHASELECT_A::VALID_TRAILING_EDGE
    }
}
#[doc = "Field `NCPHA` writer - Clock Phase"]
pub type NCPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, NCPHASELECT_A, O>;
impl<'a, const O: u8> NCPHA_W<'a, O> {
    #[doc = "Data is valid on clock leading edge (NCPHA=1)"]
    #[inline(always)]
    pub fn valid_leading_edge(self) -> &'a mut W {
        self.variant(NCPHASELECT_A::VALID_LEADING_EDGE)
    }
    #[doc = "Data is valid on clock trailing edge (NCPHA=0)"]
    #[inline(always)]
    pub fn valid_trailing_edge(self) -> &'a mut W {
        self.variant(NCPHASELECT_A::VALID_TRAILING_EDGE)
    }
}
#[doc = "Field `CSNAAT` reader - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CSNAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSNAAT` writer - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
pub type CSNAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `CSAAT` reader - Chip Select Active After Transfer"]
pub type CSAAT_R = crate::BitReader<bool>;
#[doc = "Field `CSAAT` writer - Chip Select Active After Transfer"]
pub type CSAAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `BITS` reader - Bits Per Transfer"]
pub type BITS_R = crate::FieldReader<u8, BITSSELECT_A>;
#[doc = "Bits Per Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITSSELECT_A {
    #[doc = "0: 8 bits for transfer"]
    _8_BIT = 0,
    #[doc = "1: 9 bits for transfer"]
    _9_BIT = 1,
    #[doc = "2: 10 bits for transfer"]
    _10_BIT = 2,
    #[doc = "3: 11 bits for transfer"]
    _11_BIT = 3,
    #[doc = "4: 12 bits for transfer"]
    _12_BIT = 4,
    #[doc = "5: 13 bits for transfer"]
    _13_BIT = 5,
    #[doc = "6: 14 bits for transfer"]
    _14_BIT = 6,
    #[doc = "7: 15 bits for transfer"]
    _15_BIT = 7,
    #[doc = "8: 16 bits for transfer"]
    _16_BIT = 8,
}
impl From<BITSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BITSSELECT_A) -> Self {
        variant as _
    }
}
impl BITS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITSSELECT_A> {
        match self.bits {
            0 => Some(BITSSELECT_A::_8_BIT),
            1 => Some(BITSSELECT_A::_9_BIT),
            2 => Some(BITSSELECT_A::_10_BIT),
            3 => Some(BITSSELECT_A::_11_BIT),
            4 => Some(BITSSELECT_A::_12_BIT),
            5 => Some(BITSSELECT_A::_13_BIT),
            6 => Some(BITSSELECT_A::_14_BIT),
            7 => Some(BITSSELECT_A::_15_BIT),
            8 => Some(BITSSELECT_A::_16_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == BITSSELECT_A::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == BITSSELECT_A::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_10_BIT`"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == BITSSELECT_A::_10_BIT
    }
    #[doc = "Checks if the value of the field is `_11_BIT`"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == BITSSELECT_A::_11_BIT
    }
    #[doc = "Checks if the value of the field is `_12_BIT`"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == BITSSELECT_A::_12_BIT
    }
    #[doc = "Checks if the value of the field is `_13_BIT`"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == BITSSELECT_A::_13_BIT
    }
    #[doc = "Checks if the value of the field is `_14_BIT`"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == BITSSELECT_A::_14_BIT
    }
    #[doc = "Checks if the value of the field is `_15_BIT`"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == BITSSELECT_A::_15_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == BITSSELECT_A::_16_BIT
    }
}
#[doc = "Field `BITS` writer - Bits Per Transfer"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, BITSSELECT_A, 4, O>;
impl<'a, const O: u8> BITS_W<'a, O> {
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_8_BIT)
    }
    #[doc = "9 bits for transfer"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_9_BIT)
    }
    #[doc = "10 bits for transfer"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_10_BIT)
    }
    #[doc = "11 bits for transfer"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_11_BIT)
    }
    #[doc = "12 bits for transfer"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_12_BIT)
    }
    #[doc = "13 bits for transfer"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_13_BIT)
    }
    #[doc = "14 bits for transfer"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_14_BIT)
    }
    #[doc = "15 bits for transfer"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_15_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(BITSSELECT_A::_16_BIT)
    }
}
#[doc = "Field `SCBR` reader - Serial Clock Bit Rate"]
pub type SCBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCBR` writer - Serial Clock Bit Rate"]
pub type SCBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBS` reader - Delay Before SPCK"]
pub type DLYBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBS` writer - Delay Before SPCK"]
pub type DLYBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DLYBCT` reader - Delay Between Consecutive Transfers"]
pub type DLYBCT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCT` writer - Delay Between Consecutive Transfers"]
pub type DLYBCT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&self) -> NCPHA_R {
        NCPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&self) -> CSNAAT_R {
        CSNAAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&self) -> CSAAT_R {
        CSAAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline(always)]
    pub fn scbr(&self) -> SCBR_R {
        SCBR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&self) -> DLYBS_R {
        DLYBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<0> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline(always)]
    pub fn ncpha(&mut self) -> NCPHA_W<1> {
        NCPHA_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline(always)]
    pub fn csnaat(&mut self) -> CSNAAT_W<2> {
        CSNAAT_W::new(self)
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline(always)]
    pub fn csaat(&mut self) -> CSAAT_W<3> {
        CSAAT_W::new(self)
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W<4> {
        BITS_W::new(self)
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline(always)]
    pub fn scbr(&mut self) -> SCBR_W<8> {
        SCBR_W::new(self)
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline(always)]
    pub fn dlybs(&mut self) -> DLYBS_W<16> {
        DLYBS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> DLYBCT_W<24> {
        DLYBCT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR[%s]
to value 0"]
impl crate::Resettable for CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
