#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_FMR {
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
pub type FPOL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _FPOLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FMOD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FMODW<'a> {
    w: &'a mut W,
}
impl<'a> _FMODW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FFIL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _FFILW<'a> {
    w: &'a mut W,
}
impl<'a> _FFILW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&self) -> FPOL_R {
        FPOL_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&self) -> FMOD_R {
        FMOD_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&self) -> FFIL_R {
        FFIL_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&mut self) -> _FPOLW {
        _FPOLW { w: self }
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&mut self) -> _FMODW {
        _FMODW { w: self }
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&mut self) -> _FFILW {
        _FFILW { w: self }
    }
}
