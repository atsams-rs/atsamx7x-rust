#[doc = "Register `CIM` writer"]
pub struct W(crate::W<CIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIM_SPEC>;
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
impl From<crate::W<CIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIM` writer - End of Block Interrupt Mask Bit"]
pub type BIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
#[doc = "Field `LIM` writer - End of Linked List Interrupt Mask Bit"]
pub type LIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
#[doc = "Field `DIM` writer - End of Disable Interrupt Mask Bit"]
pub type DIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
#[doc = "Field `FIM` writer - End of Flush Interrupt Mask Bit"]
pub type FIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
#[doc = "Field `RBEIM` writer - Read Bus Error Interrupt Mask Bit"]
pub type RBEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
#[doc = "Field `WBEIM` writer - Write Bus Error Interrupt Mask Bit"]
pub type WBEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
#[doc = "Field `ROIM` writer - Request Overflow Error Interrupt Mask Bit"]
pub type ROIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIM_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Mask Bit"]
    #[inline(always)]
    pub fn bim(&mut self) -> BIM_W<0> {
        BIM_W::new(self)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Mask Bit"]
    #[inline(always)]
    pub fn lim(&mut self) -> LIM_W<1> {
        LIM_W::new(self)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Mask Bit"]
    #[inline(always)]
    pub fn dim(&mut self) -> DIM_W<2> {
        DIM_W::new(self)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Mask Bit"]
    #[inline(always)]
    pub fn fim(&mut self) -> FIM_W<3> {
        FIM_W::new(self)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn rbeim(&mut self) -> RBEIM_W<4> {
        RBEIM_W::new(self)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn wbeim(&mut self) -> WBEIM_W<5> {
        WBEIM_W::new(self)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn roim(&mut self) -> ROIM_W<6> {
        ROIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Interrupt Mask Register (chid = 0)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cim](index.html) module"]
pub struct CIM_SPEC;
impl crate::RegisterSpec for CIM_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cim::W](W) writer structure"]
impl crate::Writable for CIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CIM to value 0"]
impl crate::Resettable for CIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
