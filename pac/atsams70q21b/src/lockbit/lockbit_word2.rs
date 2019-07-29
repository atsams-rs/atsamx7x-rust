#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCKBIT_WORD2 {
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
pub type LOCK_REGION_64_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_64W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_64W<'a> {
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
pub type LOCK_REGION_65_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_65W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_65W<'a> {
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
pub type LOCK_REGION_66_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_66W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_66W<'a> {
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
pub type LOCK_REGION_67_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_67W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_67W<'a> {
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
pub type LOCK_REGION_68_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_68W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_68W<'a> {
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
pub type LOCK_REGION_69_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_69W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_69W<'a> {
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
pub type LOCK_REGION_70_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_70W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_70W<'a> {
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
pub type LOCK_REGION_71_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_71W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_71W<'a> {
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
pub type LOCK_REGION_72_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_72W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_72W<'a> {
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
pub type LOCK_REGION_73_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_73W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_73W<'a> {
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
pub type LOCK_REGION_74_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_74W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_74W<'a> {
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
pub type LOCK_REGION_75_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_75W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_75W<'a> {
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
pub type LOCK_REGION_76_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_76W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_76W<'a> {
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
pub type LOCK_REGION_77_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_77W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_77W<'a> {
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
pub type LOCK_REGION_78_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_78W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_78W<'a> {
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
pub type LOCK_REGION_79_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_79W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_79W<'a> {
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
pub type LOCK_REGION_80_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_80W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_80W<'a> {
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
pub type LOCK_REGION_81_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_81W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_81W<'a> {
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
pub type LOCK_REGION_82_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_82W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_82W<'a> {
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
pub type LOCK_REGION_83_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_83W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_83W<'a> {
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
pub type LOCK_REGION_84_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_84W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_84W<'a> {
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
pub type LOCK_REGION_85_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_85W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_85W<'a> {
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
pub type LOCK_REGION_86_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_86W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_86W<'a> {
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
pub type LOCK_REGION_87_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_87W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_87W<'a> {
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
pub type LOCK_REGION_88_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_88W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_88W<'a> {
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
pub type LOCK_REGION_89_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_89W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_89W<'a> {
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
pub type LOCK_REGION_90_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_90W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_90W<'a> {
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
pub type LOCK_REGION_91_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_91W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_91W<'a> {
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
pub type LOCK_REGION_92_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_92W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_92W<'a> {
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
pub type LOCK_REGION_93_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_93W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_93W<'a> {
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
pub type LOCK_REGION_94_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_94W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_94W<'a> {
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
pub type LOCK_REGION_95_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_95W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_95W<'a> {
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
    #[doc = "Bit 0 - Lock Region 64"]
    #[inline(always)]
    pub fn lock_region_64(&self) -> LOCK_REGION_64_R {
        LOCK_REGION_64_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 65"]
    #[inline(always)]
    pub fn lock_region_65(&self) -> LOCK_REGION_65_R {
        LOCK_REGION_65_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 66"]
    #[inline(always)]
    pub fn lock_region_66(&self) -> LOCK_REGION_66_R {
        LOCK_REGION_66_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 67"]
    #[inline(always)]
    pub fn lock_region_67(&self) -> LOCK_REGION_67_R {
        LOCK_REGION_67_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 68"]
    #[inline(always)]
    pub fn lock_region_68(&self) -> LOCK_REGION_68_R {
        LOCK_REGION_68_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 69"]
    #[inline(always)]
    pub fn lock_region_69(&self) -> LOCK_REGION_69_R {
        LOCK_REGION_69_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 70"]
    #[inline(always)]
    pub fn lock_region_70(&self) -> LOCK_REGION_70_R {
        LOCK_REGION_70_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 71"]
    #[inline(always)]
    pub fn lock_region_71(&self) -> LOCK_REGION_71_R {
        LOCK_REGION_71_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 72"]
    #[inline(always)]
    pub fn lock_region_72(&self) -> LOCK_REGION_72_R {
        LOCK_REGION_72_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 73"]
    #[inline(always)]
    pub fn lock_region_73(&self) -> LOCK_REGION_73_R {
        LOCK_REGION_73_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 74"]
    #[inline(always)]
    pub fn lock_region_74(&self) -> LOCK_REGION_74_R {
        LOCK_REGION_74_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 75"]
    #[inline(always)]
    pub fn lock_region_75(&self) -> LOCK_REGION_75_R {
        LOCK_REGION_75_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 76"]
    #[inline(always)]
    pub fn lock_region_76(&self) -> LOCK_REGION_76_R {
        LOCK_REGION_76_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 77"]
    #[inline(always)]
    pub fn lock_region_77(&self) -> LOCK_REGION_77_R {
        LOCK_REGION_77_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 78"]
    #[inline(always)]
    pub fn lock_region_78(&self) -> LOCK_REGION_78_R {
        LOCK_REGION_78_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 79"]
    #[inline(always)]
    pub fn lock_region_79(&self) -> LOCK_REGION_79_R {
        LOCK_REGION_79_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 80"]
    #[inline(always)]
    pub fn lock_region_80(&self) -> LOCK_REGION_80_R {
        LOCK_REGION_80_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 81"]
    #[inline(always)]
    pub fn lock_region_81(&self) -> LOCK_REGION_81_R {
        LOCK_REGION_81_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 82"]
    #[inline(always)]
    pub fn lock_region_82(&self) -> LOCK_REGION_82_R {
        LOCK_REGION_82_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 83"]
    #[inline(always)]
    pub fn lock_region_83(&self) -> LOCK_REGION_83_R {
        LOCK_REGION_83_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 84"]
    #[inline(always)]
    pub fn lock_region_84(&self) -> LOCK_REGION_84_R {
        LOCK_REGION_84_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 85"]
    #[inline(always)]
    pub fn lock_region_85(&self) -> LOCK_REGION_85_R {
        LOCK_REGION_85_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 86"]
    #[inline(always)]
    pub fn lock_region_86(&self) -> LOCK_REGION_86_R {
        LOCK_REGION_86_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 87"]
    #[inline(always)]
    pub fn lock_region_87(&self) -> LOCK_REGION_87_R {
        LOCK_REGION_87_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 88"]
    #[inline(always)]
    pub fn lock_region_88(&self) -> LOCK_REGION_88_R {
        LOCK_REGION_88_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 89"]
    #[inline(always)]
    pub fn lock_region_89(&self) -> LOCK_REGION_89_R {
        LOCK_REGION_89_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 90"]
    #[inline(always)]
    pub fn lock_region_90(&self) -> LOCK_REGION_90_R {
        LOCK_REGION_90_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 91"]
    #[inline(always)]
    pub fn lock_region_91(&self) -> LOCK_REGION_91_R {
        LOCK_REGION_91_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 92"]
    #[inline(always)]
    pub fn lock_region_92(&self) -> LOCK_REGION_92_R {
        LOCK_REGION_92_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 93"]
    #[inline(always)]
    pub fn lock_region_93(&self) -> LOCK_REGION_93_R {
        LOCK_REGION_93_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 94"]
    #[inline(always)]
    pub fn lock_region_94(&self) -> LOCK_REGION_94_R {
        LOCK_REGION_94_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 95"]
    #[inline(always)]
    pub fn lock_region_95(&self) -> LOCK_REGION_95_R {
        LOCK_REGION_95_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Lock Region 64"]
    #[inline(always)]
    pub fn lock_region_64(&mut self) -> _LOCK_REGION_64W {
        _LOCK_REGION_64W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 65"]
    #[inline(always)]
    pub fn lock_region_65(&mut self) -> _LOCK_REGION_65W {
        _LOCK_REGION_65W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 66"]
    #[inline(always)]
    pub fn lock_region_66(&mut self) -> _LOCK_REGION_66W {
        _LOCK_REGION_66W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 67"]
    #[inline(always)]
    pub fn lock_region_67(&mut self) -> _LOCK_REGION_67W {
        _LOCK_REGION_67W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 68"]
    #[inline(always)]
    pub fn lock_region_68(&mut self) -> _LOCK_REGION_68W {
        _LOCK_REGION_68W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 69"]
    #[inline(always)]
    pub fn lock_region_69(&mut self) -> _LOCK_REGION_69W {
        _LOCK_REGION_69W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 70"]
    #[inline(always)]
    pub fn lock_region_70(&mut self) -> _LOCK_REGION_70W {
        _LOCK_REGION_70W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 71"]
    #[inline(always)]
    pub fn lock_region_71(&mut self) -> _LOCK_REGION_71W {
        _LOCK_REGION_71W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 72"]
    #[inline(always)]
    pub fn lock_region_72(&mut self) -> _LOCK_REGION_72W {
        _LOCK_REGION_72W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 73"]
    #[inline(always)]
    pub fn lock_region_73(&mut self) -> _LOCK_REGION_73W {
        _LOCK_REGION_73W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 74"]
    #[inline(always)]
    pub fn lock_region_74(&mut self) -> _LOCK_REGION_74W {
        _LOCK_REGION_74W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 75"]
    #[inline(always)]
    pub fn lock_region_75(&mut self) -> _LOCK_REGION_75W {
        _LOCK_REGION_75W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 76"]
    #[inline(always)]
    pub fn lock_region_76(&mut self) -> _LOCK_REGION_76W {
        _LOCK_REGION_76W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 77"]
    #[inline(always)]
    pub fn lock_region_77(&mut self) -> _LOCK_REGION_77W {
        _LOCK_REGION_77W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 78"]
    #[inline(always)]
    pub fn lock_region_78(&mut self) -> _LOCK_REGION_78W {
        _LOCK_REGION_78W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 79"]
    #[inline(always)]
    pub fn lock_region_79(&mut self) -> _LOCK_REGION_79W {
        _LOCK_REGION_79W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 80"]
    #[inline(always)]
    pub fn lock_region_80(&mut self) -> _LOCK_REGION_80W {
        _LOCK_REGION_80W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 81"]
    #[inline(always)]
    pub fn lock_region_81(&mut self) -> _LOCK_REGION_81W {
        _LOCK_REGION_81W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 82"]
    #[inline(always)]
    pub fn lock_region_82(&mut self) -> _LOCK_REGION_82W {
        _LOCK_REGION_82W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 83"]
    #[inline(always)]
    pub fn lock_region_83(&mut self) -> _LOCK_REGION_83W {
        _LOCK_REGION_83W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 84"]
    #[inline(always)]
    pub fn lock_region_84(&mut self) -> _LOCK_REGION_84W {
        _LOCK_REGION_84W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 85"]
    #[inline(always)]
    pub fn lock_region_85(&mut self) -> _LOCK_REGION_85W {
        _LOCK_REGION_85W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 86"]
    #[inline(always)]
    pub fn lock_region_86(&mut self) -> _LOCK_REGION_86W {
        _LOCK_REGION_86W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 87"]
    #[inline(always)]
    pub fn lock_region_87(&mut self) -> _LOCK_REGION_87W {
        _LOCK_REGION_87W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 88"]
    #[inline(always)]
    pub fn lock_region_88(&mut self) -> _LOCK_REGION_88W {
        _LOCK_REGION_88W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 89"]
    #[inline(always)]
    pub fn lock_region_89(&mut self) -> _LOCK_REGION_89W {
        _LOCK_REGION_89W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 90"]
    #[inline(always)]
    pub fn lock_region_90(&mut self) -> _LOCK_REGION_90W {
        _LOCK_REGION_90W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 91"]
    #[inline(always)]
    pub fn lock_region_91(&mut self) -> _LOCK_REGION_91W {
        _LOCK_REGION_91W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 92"]
    #[inline(always)]
    pub fn lock_region_92(&mut self) -> _LOCK_REGION_92W {
        _LOCK_REGION_92W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 93"]
    #[inline(always)]
    pub fn lock_region_93(&mut self) -> _LOCK_REGION_93W {
        _LOCK_REGION_93W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 94"]
    #[inline(always)]
    pub fn lock_region_94(&mut self) -> _LOCK_REGION_94W {
        _LOCK_REGION_94W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 95"]
    #[inline(always)]
    pub fn lock_region_95(&mut self) -> _LOCK_REGION_95W {
        _LOCK_REGION_95W { w: self }
    }
}
