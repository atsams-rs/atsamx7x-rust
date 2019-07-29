#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_TEMPCWR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type TLOWTHRES_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _TLOWTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _TLOWTHRESW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type THIGHTHRES_R = crate::FR<u16, u16>;
#[doc = r"Proxy"]
pub struct _THIGHTHRESW<'a> {
    w: &'a mut W,
}
impl<'a> _THIGHTHRESW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&self) -> TLOWTHRES_R {
        TLOWTHRES_R::new((self.bits() & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&self) -> THIGHTHRES_R {
        THIGHTHRES_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&mut self) -> _TLOWTHRESW {
        _TLOWTHRESW { w: self }
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&mut self) -> _THIGHTHRESW {
        _THIGHTHRESW { w: self }
    }
}
