#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_GRS {
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
pub type RS0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS0W<'a> {
    w: &'a mut W,
}
impl<'a> _RS0W<'a> {
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
pub type RS1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS1W<'a> {
    w: &'a mut W,
}
impl<'a> _RS1W<'a> {
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
pub type RS2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS2W<'a> {
    w: &'a mut W,
}
impl<'a> _RS2W<'a> {
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
pub type RS3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS3W<'a> {
    w: &'a mut W,
}
impl<'a> _RS3W<'a> {
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
pub type RS4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS4W<'a> {
    w: &'a mut W,
}
impl<'a> _RS4W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS5W<'a> {
    w: &'a mut W,
}
impl<'a> _RS5W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS6W<'a> {
    w: &'a mut W,
}
impl<'a> _RS6W<'a> {
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
pub type RS7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS7W<'a> {
    w: &'a mut W,
}
impl<'a> _RS7W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS8W<'a> {
    w: &'a mut W,
}
impl<'a> _RS8W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS9W<'a> {
    w: &'a mut W,
}
impl<'a> _RS9W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS10W<'a> {
    w: &'a mut W,
}
impl<'a> _RS10W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS11W<'a> {
    w: &'a mut W,
}
impl<'a> _RS11W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS12W<'a> {
    w: &'a mut W,
}
impl<'a> _RS12W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS13W<'a> {
    w: &'a mut W,
}
impl<'a> _RS13W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS14W<'a> {
    w: &'a mut W,
}
impl<'a> _RS14W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS15W<'a> {
    w: &'a mut W,
}
impl<'a> _RS15W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RS16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS16W<'a> {
    w: &'a mut W,
}
impl<'a> _RS16W<'a> {
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
pub type RS17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS17W<'a> {
    w: &'a mut W,
}
impl<'a> _RS17W<'a> {
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
pub type RS18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS18W<'a> {
    w: &'a mut W,
}
impl<'a> _RS18W<'a> {
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
pub type RS19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS19W<'a> {
    w: &'a mut W,
}
impl<'a> _RS19W<'a> {
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
#[doc = r"Reader of the field"]
pub type RS20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS20W<'a> {
    w: &'a mut W,
}
impl<'a> _RS20W<'a> {
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
pub type RS21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS21W<'a> {
    w: &'a mut W,
}
impl<'a> _RS21W<'a> {
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
pub type RS22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS22W<'a> {
    w: &'a mut W,
}
impl<'a> _RS22W<'a> {
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
#[doc = r"Reader of the field"]
pub type RS23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RS23W<'a> {
    w: &'a mut W,
}
impl<'a> _RS23W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - XDMAC Channel 0 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs0(&self) -> RS0_R {
        RS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs1(&self) -> RS1_R {
        RS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs2(&self) -> RS2_R {
        RS2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs3(&self) -> RS3_R {
        RS3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs4(&self) -> RS4_R {
        RS4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs5(&self) -> RS5_R {
        RS5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs6(&self) -> RS6_R {
        RS6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs7(&self) -> RS7_R {
        RS7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs8(&self) -> RS8_R {
        RS8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs9(&self) -> RS9_R {
        RS9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs10(&self) -> RS10_R {
        RS10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs11(&self) -> RS11_R {
        RS11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs12(&self) -> RS12_R {
        RS12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs13(&self) -> RS13_R {
        RS13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs14(&self) -> RS14_R {
        RS14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs15(&self) -> RS15_R {
        RS15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs16(&self) -> RS16_R {
        RS16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs17(&self) -> RS17_R {
        RS17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs18(&self) -> RS18_R {
        RS18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs19(&self) -> RS19_R {
        RS19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs20(&self) -> RS20_R {
        RS20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs21(&self) -> RS21_R {
        RS21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs22(&self) -> RS22_R {
        RS22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs23(&self) -> RS23_R {
        RS23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XDMAC Channel 0 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs0(&mut self) -> _RS0W {
        _RS0W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs1(&mut self) -> _RS1W {
        _RS1W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs2(&mut self) -> _RS2W {
        _RS2W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs3(&mut self) -> _RS3W {
        _RS3W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs4(&mut self) -> _RS4W {
        _RS4W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs5(&mut self) -> _RS5W {
        _RS5W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs6(&mut self) -> _RS6W {
        _RS6W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs7(&mut self) -> _RS7W {
        _RS7W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs8(&mut self) -> _RS8W {
        _RS8W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs9(&mut self) -> _RS9W {
        _RS9W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs10(&mut self) -> _RS10W {
        _RS10W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs11(&mut self) -> _RS11W {
        _RS11W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs12(&mut self) -> _RS12W {
        _RS12W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs13(&mut self) -> _RS13W {
        _RS13W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs14(&mut self) -> _RS14W {
        _RS14W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs15(&mut self) -> _RS15W {
        _RS15W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs16(&mut self) -> _RS16W {
        _RS16W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs17(&mut self) -> _RS17W {
        _RS17W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs18(&mut self) -> _RS18W {
        _RS18W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs19(&mut self) -> _RS19W {
        _RS19W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs20(&mut self) -> _RS20W {
        _RS20W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs21(&mut self) -> _RS21W {
        _RS21W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs22(&mut self) -> _RS22W {
        _RS22W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Read Suspend Bit"]
    #[inline(always)]
    pub fn rs23(&mut self) -> _RS23W {
        _RS23W { w: self }
    }
}
