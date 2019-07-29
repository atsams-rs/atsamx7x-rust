#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCKBIT_WORD3 {
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
pub type LOCK_REGION_96_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_96W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_96W<'a> {
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
pub type LOCK_REGION_97_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_97W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_97W<'a> {
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
pub type LOCK_REGION_98_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_98W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_98W<'a> {
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
pub type LOCK_REGION_99_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_99W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_99W<'a> {
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
pub type LOCK_REGION_100_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_100W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_100W<'a> {
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
pub type LOCK_REGION_101_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_101W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_101W<'a> {
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
pub type LOCK_REGION_102_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_102W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_102W<'a> {
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
pub type LOCK_REGION_103_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_103W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_103W<'a> {
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
pub type LOCK_REGION_104_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_104W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_104W<'a> {
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
pub type LOCK_REGION_105_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_105W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_105W<'a> {
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
pub type LOCK_REGION_106_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_106W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_106W<'a> {
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
pub type LOCK_REGION_107_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_107W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_107W<'a> {
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
pub type LOCK_REGION_108_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_108W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_108W<'a> {
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
pub type LOCK_REGION_109_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_109W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_109W<'a> {
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
pub type LOCK_REGION_110_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_110W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_110W<'a> {
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
pub type LOCK_REGION_111_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_111W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_111W<'a> {
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
pub type LOCK_REGION_112_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_112W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_112W<'a> {
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
pub type LOCK_REGION_113_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_113W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_113W<'a> {
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
pub type LOCK_REGION_114_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_114W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_114W<'a> {
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
pub type LOCK_REGION_115_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_115W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_115W<'a> {
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
pub type LOCK_REGION_116_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_116W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_116W<'a> {
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
pub type LOCK_REGION_117_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_117W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_117W<'a> {
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
pub type LOCK_REGION_118_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_118W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_118W<'a> {
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
pub type LOCK_REGION_119_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_119W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_119W<'a> {
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
pub type LOCK_REGION_120_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_120W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_120W<'a> {
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
pub type LOCK_REGION_121_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_121W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_121W<'a> {
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
pub type LOCK_REGION_122_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_122W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_122W<'a> {
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
pub type LOCK_REGION_123_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_123W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_123W<'a> {
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
pub type LOCK_REGION_124_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_124W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_124W<'a> {
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
pub type LOCK_REGION_125_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_125W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_125W<'a> {
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
pub type LOCK_REGION_126_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_126W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_126W<'a> {
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
pub type LOCK_REGION_127_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_127W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_127W<'a> {
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
    #[doc = "Bit 0 - Lock Region 96"]
    #[inline(always)]
    pub fn lock_region_96(&self) -> LOCK_REGION_96_R {
        LOCK_REGION_96_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 97"]
    #[inline(always)]
    pub fn lock_region_97(&self) -> LOCK_REGION_97_R {
        LOCK_REGION_97_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 98"]
    #[inline(always)]
    pub fn lock_region_98(&self) -> LOCK_REGION_98_R {
        LOCK_REGION_98_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 99"]
    #[inline(always)]
    pub fn lock_region_99(&self) -> LOCK_REGION_99_R {
        LOCK_REGION_99_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 100"]
    #[inline(always)]
    pub fn lock_region_100(&self) -> LOCK_REGION_100_R {
        LOCK_REGION_100_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 101"]
    #[inline(always)]
    pub fn lock_region_101(&self) -> LOCK_REGION_101_R {
        LOCK_REGION_101_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 102"]
    #[inline(always)]
    pub fn lock_region_102(&self) -> LOCK_REGION_102_R {
        LOCK_REGION_102_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 103"]
    #[inline(always)]
    pub fn lock_region_103(&self) -> LOCK_REGION_103_R {
        LOCK_REGION_103_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 104"]
    #[inline(always)]
    pub fn lock_region_104(&self) -> LOCK_REGION_104_R {
        LOCK_REGION_104_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 105"]
    #[inline(always)]
    pub fn lock_region_105(&self) -> LOCK_REGION_105_R {
        LOCK_REGION_105_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 106"]
    #[inline(always)]
    pub fn lock_region_106(&self) -> LOCK_REGION_106_R {
        LOCK_REGION_106_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 107"]
    #[inline(always)]
    pub fn lock_region_107(&self) -> LOCK_REGION_107_R {
        LOCK_REGION_107_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 108"]
    #[inline(always)]
    pub fn lock_region_108(&self) -> LOCK_REGION_108_R {
        LOCK_REGION_108_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 109"]
    #[inline(always)]
    pub fn lock_region_109(&self) -> LOCK_REGION_109_R {
        LOCK_REGION_109_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 110"]
    #[inline(always)]
    pub fn lock_region_110(&self) -> LOCK_REGION_110_R {
        LOCK_REGION_110_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 111"]
    #[inline(always)]
    pub fn lock_region_111(&self) -> LOCK_REGION_111_R {
        LOCK_REGION_111_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 112"]
    #[inline(always)]
    pub fn lock_region_112(&self) -> LOCK_REGION_112_R {
        LOCK_REGION_112_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 113"]
    #[inline(always)]
    pub fn lock_region_113(&self) -> LOCK_REGION_113_R {
        LOCK_REGION_113_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 114"]
    #[inline(always)]
    pub fn lock_region_114(&self) -> LOCK_REGION_114_R {
        LOCK_REGION_114_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 115"]
    #[inline(always)]
    pub fn lock_region_115(&self) -> LOCK_REGION_115_R {
        LOCK_REGION_115_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 116"]
    #[inline(always)]
    pub fn lock_region_116(&self) -> LOCK_REGION_116_R {
        LOCK_REGION_116_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 117"]
    #[inline(always)]
    pub fn lock_region_117(&self) -> LOCK_REGION_117_R {
        LOCK_REGION_117_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 118"]
    #[inline(always)]
    pub fn lock_region_118(&self) -> LOCK_REGION_118_R {
        LOCK_REGION_118_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 119"]
    #[inline(always)]
    pub fn lock_region_119(&self) -> LOCK_REGION_119_R {
        LOCK_REGION_119_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 120"]
    #[inline(always)]
    pub fn lock_region_120(&self) -> LOCK_REGION_120_R {
        LOCK_REGION_120_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 121"]
    #[inline(always)]
    pub fn lock_region_121(&self) -> LOCK_REGION_121_R {
        LOCK_REGION_121_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 122"]
    #[inline(always)]
    pub fn lock_region_122(&self) -> LOCK_REGION_122_R {
        LOCK_REGION_122_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 123"]
    #[inline(always)]
    pub fn lock_region_123(&self) -> LOCK_REGION_123_R {
        LOCK_REGION_123_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 124"]
    #[inline(always)]
    pub fn lock_region_124(&self) -> LOCK_REGION_124_R {
        LOCK_REGION_124_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 125"]
    #[inline(always)]
    pub fn lock_region_125(&self) -> LOCK_REGION_125_R {
        LOCK_REGION_125_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 126"]
    #[inline(always)]
    pub fn lock_region_126(&self) -> LOCK_REGION_126_R {
        LOCK_REGION_126_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 127"]
    #[inline(always)]
    pub fn lock_region_127(&self) -> LOCK_REGION_127_R {
        LOCK_REGION_127_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Lock Region 96"]
    #[inline(always)]
    pub fn lock_region_96(&mut self) -> _LOCK_REGION_96W {
        _LOCK_REGION_96W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 97"]
    #[inline(always)]
    pub fn lock_region_97(&mut self) -> _LOCK_REGION_97W {
        _LOCK_REGION_97W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 98"]
    #[inline(always)]
    pub fn lock_region_98(&mut self) -> _LOCK_REGION_98W {
        _LOCK_REGION_98W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 99"]
    #[inline(always)]
    pub fn lock_region_99(&mut self) -> _LOCK_REGION_99W {
        _LOCK_REGION_99W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 100"]
    #[inline(always)]
    pub fn lock_region_100(&mut self) -> _LOCK_REGION_100W {
        _LOCK_REGION_100W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 101"]
    #[inline(always)]
    pub fn lock_region_101(&mut self) -> _LOCK_REGION_101W {
        _LOCK_REGION_101W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 102"]
    #[inline(always)]
    pub fn lock_region_102(&mut self) -> _LOCK_REGION_102W {
        _LOCK_REGION_102W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 103"]
    #[inline(always)]
    pub fn lock_region_103(&mut self) -> _LOCK_REGION_103W {
        _LOCK_REGION_103W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 104"]
    #[inline(always)]
    pub fn lock_region_104(&mut self) -> _LOCK_REGION_104W {
        _LOCK_REGION_104W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 105"]
    #[inline(always)]
    pub fn lock_region_105(&mut self) -> _LOCK_REGION_105W {
        _LOCK_REGION_105W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 106"]
    #[inline(always)]
    pub fn lock_region_106(&mut self) -> _LOCK_REGION_106W {
        _LOCK_REGION_106W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 107"]
    #[inline(always)]
    pub fn lock_region_107(&mut self) -> _LOCK_REGION_107W {
        _LOCK_REGION_107W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 108"]
    #[inline(always)]
    pub fn lock_region_108(&mut self) -> _LOCK_REGION_108W {
        _LOCK_REGION_108W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 109"]
    #[inline(always)]
    pub fn lock_region_109(&mut self) -> _LOCK_REGION_109W {
        _LOCK_REGION_109W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 110"]
    #[inline(always)]
    pub fn lock_region_110(&mut self) -> _LOCK_REGION_110W {
        _LOCK_REGION_110W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 111"]
    #[inline(always)]
    pub fn lock_region_111(&mut self) -> _LOCK_REGION_111W {
        _LOCK_REGION_111W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 112"]
    #[inline(always)]
    pub fn lock_region_112(&mut self) -> _LOCK_REGION_112W {
        _LOCK_REGION_112W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 113"]
    #[inline(always)]
    pub fn lock_region_113(&mut self) -> _LOCK_REGION_113W {
        _LOCK_REGION_113W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 114"]
    #[inline(always)]
    pub fn lock_region_114(&mut self) -> _LOCK_REGION_114W {
        _LOCK_REGION_114W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 115"]
    #[inline(always)]
    pub fn lock_region_115(&mut self) -> _LOCK_REGION_115W {
        _LOCK_REGION_115W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 116"]
    #[inline(always)]
    pub fn lock_region_116(&mut self) -> _LOCK_REGION_116W {
        _LOCK_REGION_116W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 117"]
    #[inline(always)]
    pub fn lock_region_117(&mut self) -> _LOCK_REGION_117W {
        _LOCK_REGION_117W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 118"]
    #[inline(always)]
    pub fn lock_region_118(&mut self) -> _LOCK_REGION_118W {
        _LOCK_REGION_118W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 119"]
    #[inline(always)]
    pub fn lock_region_119(&mut self) -> _LOCK_REGION_119W {
        _LOCK_REGION_119W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 120"]
    #[inline(always)]
    pub fn lock_region_120(&mut self) -> _LOCK_REGION_120W {
        _LOCK_REGION_120W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 121"]
    #[inline(always)]
    pub fn lock_region_121(&mut self) -> _LOCK_REGION_121W {
        _LOCK_REGION_121W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 122"]
    #[inline(always)]
    pub fn lock_region_122(&mut self) -> _LOCK_REGION_122W {
        _LOCK_REGION_122W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 123"]
    #[inline(always)]
    pub fn lock_region_123(&mut self) -> _LOCK_REGION_123W {
        _LOCK_REGION_123W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 124"]
    #[inline(always)]
    pub fn lock_region_124(&mut self) -> _LOCK_REGION_124W {
        _LOCK_REGION_124W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 125"]
    #[inline(always)]
    pub fn lock_region_125(&mut self) -> _LOCK_REGION_125W {
        _LOCK_REGION_125W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 126"]
    #[inline(always)]
    pub fn lock_region_126(&mut self) -> _LOCK_REGION_126W {
        _LOCK_REGION_126W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 127"]
    #[inline(always)]
    pub fn lock_region_127(&mut self) -> _LOCK_REGION_127W {
        _LOCK_REGION_127W { w: self }
    }
}
