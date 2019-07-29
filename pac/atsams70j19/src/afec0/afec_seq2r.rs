#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_SEQ2R {
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
pub type USCH8_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH8W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH8W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH9_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH9W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH9W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH10_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH10W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH10W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH11_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH11W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH11W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> USCH8_R {
        USCH8_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&mut self) -> _USCH8W {
        _USCH8W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&mut self) -> _USCH9W {
        _USCH9W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&mut self) -> _USCH10W {
        _USCH10W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&mut self) -> _USCH11W {
        _USCH11W { w: self }
    }
}
