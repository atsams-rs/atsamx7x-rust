#[doc = "Register `GFC` reader"]
pub struct R(crate::R<GFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFC` writer"]
pub struct W(crate::W<GFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFC_SPEC>;
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
impl From<crate::W<GFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reject Remote Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRFE_A {
    #[doc = "0: Filter remote frames with 29-bit extended IDs."]
    FILTER = 0,
    #[doc = "1: Reject all remote frames with 29-bit extended IDs."]
    REJECT = 1,
}
impl From<RRFE_A> for bool {
    #[inline(always)]
    fn from(variant: RRFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFE` reader - Reject Remote Frames Extended"]
pub type RRFE_R = crate::BitReader<RRFE_A>;
impl RRFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFE_A {
        match self.bits {
            false => RRFE_A::FILTER,
            true => RRFE_A::REJECT,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == RRFE_A::FILTER
    }
    #[doc = "Checks if the value of the field is `REJECT`"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == RRFE_A::REJECT
    }
}
#[doc = "Field `RRFE` writer - Reject Remote Frames Extended"]
pub type RRFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFC_SPEC, RRFE_A, O>;
impl<'a, const O: u8> RRFE_W<'a, O> {
    #[doc = "Filter remote frames with 29-bit extended IDs."]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(RRFE_A::FILTER)
    }
    #[doc = "Reject all remote frames with 29-bit extended IDs."]
    #[inline(always)]
    pub fn reject(self) -> &'a mut W {
        self.variant(RRFE_A::REJECT)
    }
}
#[doc = "Reject Remote Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRFS_A {
    #[doc = "0: Filter remote frames with 11-bit standard IDs."]
    FILTER = 0,
    #[doc = "1: Reject all remote frames with 11-bit standard IDs."]
    REJECT = 1,
}
impl From<RRFS_A> for bool {
    #[inline(always)]
    fn from(variant: RRFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRFS` reader - Reject Remote Frames Standard"]
pub type RRFS_R = crate::BitReader<RRFS_A>;
impl RRFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRFS_A {
        match self.bits {
            false => RRFS_A::FILTER,
            true => RRFS_A::REJECT,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == RRFS_A::FILTER
    }
    #[doc = "Checks if the value of the field is `REJECT`"]
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        *self == RRFS_A::REJECT
    }
}
#[doc = "Field `RRFS` writer - Reject Remote Frames Standard"]
pub type RRFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFC_SPEC, RRFS_A, O>;
impl<'a, const O: u8> RRFS_W<'a, O> {
    #[doc = "Filter remote frames with 11-bit standard IDs."]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(RRFS_A::FILTER)
    }
    #[doc = "Reject all remote frames with 11-bit standard IDs."]
    #[inline(always)]
    pub fn reject(self) -> &'a mut W {
        self.variant(RRFS_A::REJECT)
    }
}
#[doc = "Accept Non-matching Frames Extended\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANFE_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    RX_FIFO_0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RX_FIFO_1 = 1,
}
impl From<ANFE_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ANFE` reader - Accept Non-matching Frames Extended"]
pub type ANFE_R = crate::FieldReader<u8, ANFE_A>;
impl ANFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANFE_A> {
        match self.bits {
            0 => Some(ANFE_A::RX_FIFO_0),
            1 => Some(ANFE_A::RX_FIFO_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_0(&self) -> bool {
        *self == ANFE_A::RX_FIFO_0
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_1(&self) -> bool {
        *self == ANFE_A::RX_FIFO_1
    }
}
#[doc = "Field `ANFE` writer - Accept Non-matching Frames Extended"]
pub type ANFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GFC_SPEC, u8, ANFE_A, 2, O>;
impl<'a, const O: u8> ANFE_W<'a, O> {
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rx_fifo_0(self) -> &'a mut W {
        self.variant(ANFE_A::RX_FIFO_0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rx_fifo_1(self) -> &'a mut W {
        self.variant(ANFE_A::RX_FIFO_1)
    }
}
#[doc = "Accept Non-matching Frames Standard\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANFS_A {
    #[doc = "0: Accept in Rx FIFO 0"]
    RX_FIFO_0 = 0,
    #[doc = "1: Accept in Rx FIFO 1"]
    RX_FIFO_1 = 1,
}
impl From<ANFS_A> for u8 {
    #[inline(always)]
    fn from(variant: ANFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ANFS` reader - Accept Non-matching Frames Standard"]
pub type ANFS_R = crate::FieldReader<u8, ANFS_A>;
impl ANFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANFS_A> {
        match self.bits {
            0 => Some(ANFS_A::RX_FIFO_0),
            1 => Some(ANFS_A::RX_FIFO_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_0`"]
    #[inline(always)]
    pub fn is_rx_fifo_0(&self) -> bool {
        *self == ANFS_A::RX_FIFO_0
    }
    #[doc = "Checks if the value of the field is `RX_FIFO_1`"]
    #[inline(always)]
    pub fn is_rx_fifo_1(&self) -> bool {
        *self == ANFS_A::RX_FIFO_1
    }
}
#[doc = "Field `ANFS` writer - Accept Non-matching Frames Standard"]
pub type ANFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GFC_SPEC, u8, ANFS_A, 2, O>;
impl<'a, const O: u8> ANFS_W<'a, O> {
    #[doc = "Accept in Rx FIFO 0"]
    #[inline(always)]
    pub fn rx_fifo_0(self) -> &'a mut W {
        self.variant(ANFS_A::RX_FIFO_0)
    }
    #[doc = "Accept in Rx FIFO 1"]
    #[inline(always)]
    pub fn rx_fifo_1(self) -> &'a mut W {
        self.variant(ANFS_A::RX_FIFO_1)
    }
}
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended"]
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W<0> {
        RRFE_W::new(self)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard"]
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W<1> {
        RRFS_W::new(self)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended"]
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W<2> {
        ANFE_W::new(self)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard"]
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W<4> {
        ANFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfc](index.html) module"]
pub struct GFC_SPEC;
impl crate::RegisterSpec for GFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfc::R](R) reader structure"]
impl crate::Readable for GFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfc::W](W) writer structure"]
impl crate::Writable for GFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GFC to value 0"]
impl crate::Resettable for GFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
