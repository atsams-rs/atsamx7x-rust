#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_SEQ1R {
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
pub type USCH0_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH0W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH1W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH2W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH3W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH4_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH4W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH5_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH5W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH5W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH6_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH6W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH6W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type USCH7_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _USCH7W<'a> {
    w: &'a mut W,
}
impl<'a> _USCH7W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&self) -> USCH0_R {
        USCH0_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&self) -> USCH1_R {
        USCH1_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&self) -> USCH2_R {
        USCH2_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&self) -> USCH3_R {
        USCH3_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&self) -> USCH4_R {
        USCH4_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&self) -> USCH5_R {
        USCH5_R::new(((self.bits() >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&self) -> USCH6_R {
        USCH6_R::new(((self.bits() >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&self) -> USCH7_R {
        USCH7_R::new(((self.bits() >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&mut self) -> _USCH0W {
        _USCH0W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&mut self) -> _USCH1W {
        _USCH1W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&mut self) -> _USCH2W {
        _USCH2W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&mut self) -> _USCH3W {
        _USCH3W { w: self }
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&mut self) -> _USCH4W {
        _USCH4W { w: self }
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&mut self) -> _USCH5W {
        _USCH5W { w: self }
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&mut self) -> _USCH6W {
        _USCH6W { w: self }
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&mut self) -> _USCH7W {
        _USCH7W { w: self }
    }
}
