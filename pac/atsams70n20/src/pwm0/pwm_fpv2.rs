#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_FPV2 {
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
pub type FPZH0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZH0W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZH0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZH1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZH1W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZH1W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZH2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZH2W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZH2W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZH3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZH3W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZH3W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZL0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZL0W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZL0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZL1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZL1W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZL1W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZL2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZL2W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZL2W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FPZL3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FPZL3W<'a> {
    w: &'a mut W,
}
impl<'a> _FPZL3W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&self) -> FPZH0_R {
        FPZH0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&self) -> FPZH1_R {
        FPZH1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&self) -> FPZH2_R {
        FPZH2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&self) -> FPZH3_R {
        FPZH3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&self) -> FPZL0_R {
        FPZL0_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&self) -> FPZL1_R {
        FPZL1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&self) -> FPZL2_R {
        FPZL2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&self) -> FPZL3_R {
        FPZL3_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&mut self) -> _FPZH0W {
        _FPZH0W { w: self }
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&mut self) -> _FPZH1W {
        _FPZH1W { w: self }
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&mut self) -> _FPZH2W {
        _FPZH2W { w: self }
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&mut self) -> _FPZH3W {
        _FPZH3W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&mut self) -> _FPZL0W {
        _FPZL0W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&mut self) -> _FPZL1W {
        _FPZL1W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&mut self) -> _FPZL2W {
        _FPZL2W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&mut self) -> _FPZL3W {
        _FPZL3W { w: self }
    }
}
