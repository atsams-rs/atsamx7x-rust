#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TWIHS_SMR {
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
pub type NACKEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NACKENW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKENW<'a> {
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
pub type SMDA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMDAW<'a> {
    w: &'a mut W,
}
impl<'a> _SMDAW<'a> {
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
pub type SMHH_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMHHW<'a> {
    w: &'a mut W,
}
impl<'a> _SMHHW<'a> {
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
pub type SCLWSDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCLWSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SCLWSDISW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MASK_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SADRW<'a> {
    w: &'a mut W,
}
impl<'a> _SADRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADR1EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SADR1ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SADR1ENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADR2EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SADR2ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SADR2ENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SADR3EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SADR3ENW<'a> {
    w: &'a mut W,
}
impl<'a> _SADR3ENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DATAMEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAMENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&self) -> SCLWSDIS_R {
        SCLWSDIS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits() >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&self) -> SADR1EN_R {
        SADR1EN_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&self) -> SADR2EN_R {
        SADR2EN_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&self) -> SADR3EN_R {
        SADR3EN_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&self) -> DATAMEN_R {
        DATAMEN_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&mut self) -> _NACKENW {
        _NACKENW { w: self }
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&mut self) -> _SMDAW {
        _SMDAW { w: self }
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&mut self) -> _SMHHW {
        _SMHHW { w: self }
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&mut self) -> _SCLWSDISW {
        _SCLWSDISW { w: self }
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> _MASKW {
        _MASKW { w: self }
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> _SADRW {
        _SADRW { w: self }
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&mut self) -> _SADR1ENW {
        _SADR1ENW { w: self }
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&mut self) -> _SADR2ENW {
        _SADR2ENW { w: self }
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&mut self) -> _SADR3ENW {
        _SADR3ENW { w: self }
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&mut self) -> _DATAMENW {
        _DATAMENW { w: self }
    }
}
