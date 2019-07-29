#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TWIHS_SWMR {
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
pub type SADR1_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SADR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SADR1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADR2_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SADR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SADR2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADR3_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SADR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SADR3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DATAM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DATAMW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&self) -> SADR1_R {
        SADR1_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&self) -> SADR2_R {
        SADR2_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&self) -> SADR3_R {
        SADR3_R::new(((self.bits() >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&mut self) -> _SADR1W {
        _SADR1W { w: self }
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&mut self) -> _SADR2W {
        _SADR2W { w: self }
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&mut self) -> _SADR3W {
        _SADR3W { w: self }
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&mut self) -> _DATAMW {
        _DATAMW { w: self }
    }
}
