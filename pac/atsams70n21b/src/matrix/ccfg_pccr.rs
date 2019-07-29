#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_PCCR {
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
pub type TC0CC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TC0CCW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0CCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type I2SC0CC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _I2SC0CCW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SC0CCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type I2SC1CC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _I2SC1CCW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SC1CCW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&self) -> TC0CC_R {
        TC0CC_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&self) -> I2SC0CC_R {
        I2SC0CC_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&self) -> I2SC1CC_R {
        I2SC1CC_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&mut self) -> _TC0CCW {
        _TC0CCW { w: self }
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&mut self) -> _I2SC0CCW {
        _I2SC0CCW { w: self }
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&mut self) -> _I2SC1CCW {
        _I2SC1CCW { w: self }
    }
}
