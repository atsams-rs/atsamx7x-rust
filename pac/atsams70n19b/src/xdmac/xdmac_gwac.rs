#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_GWAC {
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
pub type PW0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PW0W<'a> {
    w: &'a mut W,
}
impl<'a> _PW0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PW1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PW1W<'a> {
    w: &'a mut W,
}
impl<'a> _PW1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PW2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PW2W<'a> {
    w: &'a mut W,
}
impl<'a> _PW2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PW3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PW3W<'a> {
    w: &'a mut W,
}
impl<'a> _PW3W<'a> {
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
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&self) -> PW0_R {
        PW0_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&self) -> PW1_R {
        PW1_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&self) -> PW2_R {
        PW2_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&self) -> PW3_R {
        PW3_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&mut self) -> _PW0W {
        _PW0W { w: self }
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&mut self) -> _PW1W {
        _PW1W { w: self }
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&mut self) -> _PW2W {
        _PW2W { w: self }
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&mut self) -> _PW3W {
        _PW3W { w: self }
    }
}
