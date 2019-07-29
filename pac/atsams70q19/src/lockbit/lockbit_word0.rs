#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCKBIT_WORD0 {
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
pub type LOCK_REGION_0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_0W<'a> {
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
pub type LOCK_REGION_1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_1W<'a> {
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
pub type LOCK_REGION_2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_2W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_2W<'a> {
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
pub type LOCK_REGION_3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_3W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_3W<'a> {
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
pub type LOCK_REGION_4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_4W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_4W<'a> {
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
pub type LOCK_REGION_5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_5W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_5W<'a> {
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
pub type LOCK_REGION_6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_6W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_6W<'a> {
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
pub type LOCK_REGION_7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_7W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_7W<'a> {
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
pub type LOCK_REGION_8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_8W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_8W<'a> {
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
pub type LOCK_REGION_9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_9W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_9W<'a> {
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
pub type LOCK_REGION_10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_10W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_10W<'a> {
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
pub type LOCK_REGION_11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_11W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_11W<'a> {
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
pub type LOCK_REGION_12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_12W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_12W<'a> {
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
pub type LOCK_REGION_13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_13W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_13W<'a> {
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
pub type LOCK_REGION_14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_14W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_14W<'a> {
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
pub type LOCK_REGION_15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_15W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_15W<'a> {
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
pub type LOCK_REGION_16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_16W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_16W<'a> {
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
pub type LOCK_REGION_17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_17W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_17W<'a> {
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
pub type LOCK_REGION_18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_18W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_18W<'a> {
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
pub type LOCK_REGION_19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_19W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_19W<'a> {
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
pub type LOCK_REGION_20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_20W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_20W<'a> {
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
pub type LOCK_REGION_21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_21W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_21W<'a> {
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
pub type LOCK_REGION_22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_22W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_22W<'a> {
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
pub type LOCK_REGION_23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_23W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_23W<'a> {
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
#[doc = r"Reader of the field"]
pub type LOCK_REGION_24_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_24W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_24W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_REGION_25_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_25W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_25W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_REGION_26_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_26W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_26W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_REGION_27_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_27W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_27W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LOCK_REGION_28_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_28W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_28W<'a> {
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
pub type LOCK_REGION_29_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_29W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_29W<'a> {
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
pub type LOCK_REGION_30_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_30W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_30W<'a> {
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
pub type LOCK_REGION_31_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_31W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_31W<'a> {
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
    #[doc = "Bit 0 - Lock Region 0"]
    #[inline(always)]
    pub fn lock_region_0(&self) -> LOCK_REGION_0_R {
        LOCK_REGION_0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 1"]
    #[inline(always)]
    pub fn lock_region_1(&self) -> LOCK_REGION_1_R {
        LOCK_REGION_1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 2"]
    #[inline(always)]
    pub fn lock_region_2(&self) -> LOCK_REGION_2_R {
        LOCK_REGION_2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 3"]
    #[inline(always)]
    pub fn lock_region_3(&self) -> LOCK_REGION_3_R {
        LOCK_REGION_3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 4"]
    #[inline(always)]
    pub fn lock_region_4(&self) -> LOCK_REGION_4_R {
        LOCK_REGION_4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 5"]
    #[inline(always)]
    pub fn lock_region_5(&self) -> LOCK_REGION_5_R {
        LOCK_REGION_5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 6"]
    #[inline(always)]
    pub fn lock_region_6(&self) -> LOCK_REGION_6_R {
        LOCK_REGION_6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 7"]
    #[inline(always)]
    pub fn lock_region_7(&self) -> LOCK_REGION_7_R {
        LOCK_REGION_7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 8"]
    #[inline(always)]
    pub fn lock_region_8(&self) -> LOCK_REGION_8_R {
        LOCK_REGION_8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 9"]
    #[inline(always)]
    pub fn lock_region_9(&self) -> LOCK_REGION_9_R {
        LOCK_REGION_9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 10"]
    #[inline(always)]
    pub fn lock_region_10(&self) -> LOCK_REGION_10_R {
        LOCK_REGION_10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 11"]
    #[inline(always)]
    pub fn lock_region_11(&self) -> LOCK_REGION_11_R {
        LOCK_REGION_11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 12"]
    #[inline(always)]
    pub fn lock_region_12(&self) -> LOCK_REGION_12_R {
        LOCK_REGION_12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 13"]
    #[inline(always)]
    pub fn lock_region_13(&self) -> LOCK_REGION_13_R {
        LOCK_REGION_13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 14"]
    #[inline(always)]
    pub fn lock_region_14(&self) -> LOCK_REGION_14_R {
        LOCK_REGION_14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 15"]
    #[inline(always)]
    pub fn lock_region_15(&self) -> LOCK_REGION_15_R {
        LOCK_REGION_15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 16"]
    #[inline(always)]
    pub fn lock_region_16(&self) -> LOCK_REGION_16_R {
        LOCK_REGION_16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 17"]
    #[inline(always)]
    pub fn lock_region_17(&self) -> LOCK_REGION_17_R {
        LOCK_REGION_17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 18"]
    #[inline(always)]
    pub fn lock_region_18(&self) -> LOCK_REGION_18_R {
        LOCK_REGION_18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 19"]
    #[inline(always)]
    pub fn lock_region_19(&self) -> LOCK_REGION_19_R {
        LOCK_REGION_19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 20"]
    #[inline(always)]
    pub fn lock_region_20(&self) -> LOCK_REGION_20_R {
        LOCK_REGION_20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 21"]
    #[inline(always)]
    pub fn lock_region_21(&self) -> LOCK_REGION_21_R {
        LOCK_REGION_21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 22"]
    #[inline(always)]
    pub fn lock_region_22(&self) -> LOCK_REGION_22_R {
        LOCK_REGION_22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 23"]
    #[inline(always)]
    pub fn lock_region_23(&self) -> LOCK_REGION_23_R {
        LOCK_REGION_23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 24"]
    #[inline(always)]
    pub fn lock_region_24(&self) -> LOCK_REGION_24_R {
        LOCK_REGION_24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 25"]
    #[inline(always)]
    pub fn lock_region_25(&self) -> LOCK_REGION_25_R {
        LOCK_REGION_25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 26"]
    #[inline(always)]
    pub fn lock_region_26(&self) -> LOCK_REGION_26_R {
        LOCK_REGION_26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 27"]
    #[inline(always)]
    pub fn lock_region_27(&self) -> LOCK_REGION_27_R {
        LOCK_REGION_27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 28"]
    #[inline(always)]
    pub fn lock_region_28(&self) -> LOCK_REGION_28_R {
        LOCK_REGION_28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 29"]
    #[inline(always)]
    pub fn lock_region_29(&self) -> LOCK_REGION_29_R {
        LOCK_REGION_29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 30"]
    #[inline(always)]
    pub fn lock_region_30(&self) -> LOCK_REGION_30_R {
        LOCK_REGION_30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 31"]
    #[inline(always)]
    pub fn lock_region_31(&self) -> LOCK_REGION_31_R {
        LOCK_REGION_31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Lock Region 0"]
    #[inline(always)]
    pub fn lock_region_0(&mut self) -> _LOCK_REGION_0W {
        _LOCK_REGION_0W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 1"]
    #[inline(always)]
    pub fn lock_region_1(&mut self) -> _LOCK_REGION_1W {
        _LOCK_REGION_1W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 2"]
    #[inline(always)]
    pub fn lock_region_2(&mut self) -> _LOCK_REGION_2W {
        _LOCK_REGION_2W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 3"]
    #[inline(always)]
    pub fn lock_region_3(&mut self) -> _LOCK_REGION_3W {
        _LOCK_REGION_3W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 4"]
    #[inline(always)]
    pub fn lock_region_4(&mut self) -> _LOCK_REGION_4W {
        _LOCK_REGION_4W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 5"]
    #[inline(always)]
    pub fn lock_region_5(&mut self) -> _LOCK_REGION_5W {
        _LOCK_REGION_5W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 6"]
    #[inline(always)]
    pub fn lock_region_6(&mut self) -> _LOCK_REGION_6W {
        _LOCK_REGION_6W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 7"]
    #[inline(always)]
    pub fn lock_region_7(&mut self) -> _LOCK_REGION_7W {
        _LOCK_REGION_7W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 8"]
    #[inline(always)]
    pub fn lock_region_8(&mut self) -> _LOCK_REGION_8W {
        _LOCK_REGION_8W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 9"]
    #[inline(always)]
    pub fn lock_region_9(&mut self) -> _LOCK_REGION_9W {
        _LOCK_REGION_9W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 10"]
    #[inline(always)]
    pub fn lock_region_10(&mut self) -> _LOCK_REGION_10W {
        _LOCK_REGION_10W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 11"]
    #[inline(always)]
    pub fn lock_region_11(&mut self) -> _LOCK_REGION_11W {
        _LOCK_REGION_11W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 12"]
    #[inline(always)]
    pub fn lock_region_12(&mut self) -> _LOCK_REGION_12W {
        _LOCK_REGION_12W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 13"]
    #[inline(always)]
    pub fn lock_region_13(&mut self) -> _LOCK_REGION_13W {
        _LOCK_REGION_13W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 14"]
    #[inline(always)]
    pub fn lock_region_14(&mut self) -> _LOCK_REGION_14W {
        _LOCK_REGION_14W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 15"]
    #[inline(always)]
    pub fn lock_region_15(&mut self) -> _LOCK_REGION_15W {
        _LOCK_REGION_15W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 16"]
    #[inline(always)]
    pub fn lock_region_16(&mut self) -> _LOCK_REGION_16W {
        _LOCK_REGION_16W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 17"]
    #[inline(always)]
    pub fn lock_region_17(&mut self) -> _LOCK_REGION_17W {
        _LOCK_REGION_17W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 18"]
    #[inline(always)]
    pub fn lock_region_18(&mut self) -> _LOCK_REGION_18W {
        _LOCK_REGION_18W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 19"]
    #[inline(always)]
    pub fn lock_region_19(&mut self) -> _LOCK_REGION_19W {
        _LOCK_REGION_19W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 20"]
    #[inline(always)]
    pub fn lock_region_20(&mut self) -> _LOCK_REGION_20W {
        _LOCK_REGION_20W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 21"]
    #[inline(always)]
    pub fn lock_region_21(&mut self) -> _LOCK_REGION_21W {
        _LOCK_REGION_21W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 22"]
    #[inline(always)]
    pub fn lock_region_22(&mut self) -> _LOCK_REGION_22W {
        _LOCK_REGION_22W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 23"]
    #[inline(always)]
    pub fn lock_region_23(&mut self) -> _LOCK_REGION_23W {
        _LOCK_REGION_23W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 24"]
    #[inline(always)]
    pub fn lock_region_24(&mut self) -> _LOCK_REGION_24W {
        _LOCK_REGION_24W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 25"]
    #[inline(always)]
    pub fn lock_region_25(&mut self) -> _LOCK_REGION_25W {
        _LOCK_REGION_25W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 26"]
    #[inline(always)]
    pub fn lock_region_26(&mut self) -> _LOCK_REGION_26W {
        _LOCK_REGION_26W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 27"]
    #[inline(always)]
    pub fn lock_region_27(&mut self) -> _LOCK_REGION_27W {
        _LOCK_REGION_27W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 28"]
    #[inline(always)]
    pub fn lock_region_28(&mut self) -> _LOCK_REGION_28W {
        _LOCK_REGION_28W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 29"]
    #[inline(always)]
    pub fn lock_region_29(&mut self) -> _LOCK_REGION_29W {
        _LOCK_REGION_29W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 30"]
    #[inline(always)]
    pub fn lock_region_30(&mut self) -> _LOCK_REGION_30W {
        _LOCK_REGION_30W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 31"]
    #[inline(always)]
    pub fn lock_region_31(&mut self) -> _LOCK_REGION_31W {
        _LOCK_REGION_31W { w: self }
    }
}
