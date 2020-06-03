#[doc = "Reader of register PIO_SCHMITT"]
pub type R = crate::R<u32, super::PIO_SCHMITT>;
#[doc = "Writer for register PIO_SCHMITT"]
pub type W = crate::W<u32, super::PIO_SCHMITT>;
#[doc = "Register PIO_SCHMITT `reset()`'s with value 0"]
impl crate::ResetValue for super::PIO_SCHMITT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SCHMITT0`"]
pub type SCHMITT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT0`"]
pub struct SCHMITT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT0_W<'a> {
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
#[doc = "Reader of field `SCHMITT1`"]
pub type SCHMITT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT1`"]
pub struct SCHMITT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT1_W<'a> {
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
#[doc = "Reader of field `SCHMITT2`"]
pub type SCHMITT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT2`"]
pub struct SCHMITT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT2_W<'a> {
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
#[doc = "Reader of field `SCHMITT3`"]
pub type SCHMITT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT3`"]
pub struct SCHMITT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT3_W<'a> {
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
#[doc = "Reader of field `SCHMITT4`"]
pub type SCHMITT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT4`"]
pub struct SCHMITT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT4_W<'a> {
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
#[doc = "Reader of field `SCHMITT5`"]
pub type SCHMITT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT5`"]
pub struct SCHMITT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT5_W<'a> {
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
#[doc = "Reader of field `SCHMITT6`"]
pub type SCHMITT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT6`"]
pub struct SCHMITT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT6_W<'a> {
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
#[doc = "Reader of field `SCHMITT7`"]
pub type SCHMITT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT7`"]
pub struct SCHMITT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT7_W<'a> {
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
#[doc = "Reader of field `SCHMITT8`"]
pub type SCHMITT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT8`"]
pub struct SCHMITT8_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT8_W<'a> {
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
#[doc = "Reader of field `SCHMITT9`"]
pub type SCHMITT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT9`"]
pub struct SCHMITT9_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT9_W<'a> {
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
#[doc = "Reader of field `SCHMITT10`"]
pub type SCHMITT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT10`"]
pub struct SCHMITT10_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT10_W<'a> {
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
#[doc = "Reader of field `SCHMITT11`"]
pub type SCHMITT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT11`"]
pub struct SCHMITT11_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT11_W<'a> {
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
#[doc = "Reader of field `SCHMITT12`"]
pub type SCHMITT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT12`"]
pub struct SCHMITT12_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT12_W<'a> {
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
#[doc = "Reader of field `SCHMITT13`"]
pub type SCHMITT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT13`"]
pub struct SCHMITT13_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT13_W<'a> {
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
#[doc = "Reader of field `SCHMITT14`"]
pub type SCHMITT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT14`"]
pub struct SCHMITT14_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT14_W<'a> {
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
#[doc = "Reader of field `SCHMITT15`"]
pub type SCHMITT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT15`"]
pub struct SCHMITT15_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT15_W<'a> {
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
#[doc = "Reader of field `SCHMITT16`"]
pub type SCHMITT16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT16`"]
pub struct SCHMITT16_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT16_W<'a> {
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
#[doc = "Reader of field `SCHMITT17`"]
pub type SCHMITT17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT17`"]
pub struct SCHMITT17_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT17_W<'a> {
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
#[doc = "Reader of field `SCHMITT18`"]
pub type SCHMITT18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT18`"]
pub struct SCHMITT18_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT18_W<'a> {
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
#[doc = "Reader of field `SCHMITT19`"]
pub type SCHMITT19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT19`"]
pub struct SCHMITT19_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT19_W<'a> {
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
#[doc = "Reader of field `SCHMITT20`"]
pub type SCHMITT20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT20`"]
pub struct SCHMITT20_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT20_W<'a> {
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
#[doc = "Reader of field `SCHMITT21`"]
pub type SCHMITT21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT21`"]
pub struct SCHMITT21_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT21_W<'a> {
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
#[doc = "Reader of field `SCHMITT22`"]
pub type SCHMITT22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT22`"]
pub struct SCHMITT22_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT22_W<'a> {
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
#[doc = "Reader of field `SCHMITT23`"]
pub type SCHMITT23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT23`"]
pub struct SCHMITT23_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT23_W<'a> {
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
#[doc = "Reader of field `SCHMITT24`"]
pub type SCHMITT24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT24`"]
pub struct SCHMITT24_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT24_W<'a> {
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
#[doc = "Reader of field `SCHMITT25`"]
pub type SCHMITT25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT25`"]
pub struct SCHMITT25_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT25_W<'a> {
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
#[doc = "Reader of field `SCHMITT26`"]
pub type SCHMITT26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT26`"]
pub struct SCHMITT26_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT26_W<'a> {
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
#[doc = "Reader of field `SCHMITT27`"]
pub type SCHMITT27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT27`"]
pub struct SCHMITT27_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT27_W<'a> {
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
#[doc = "Reader of field `SCHMITT28`"]
pub type SCHMITT28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT28`"]
pub struct SCHMITT28_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT28_W<'a> {
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
#[doc = "Reader of field `SCHMITT29`"]
pub type SCHMITT29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT29`"]
pub struct SCHMITT29_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT29_W<'a> {
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
#[doc = "Reader of field `SCHMITT30`"]
pub type SCHMITT30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT30`"]
pub struct SCHMITT30_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT30_W<'a> {
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
#[doc = "Reader of field `SCHMITT31`"]
pub type SCHMITT31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCHMITT31`"]
pub struct SCHMITT31_W<'a> {
    w: &'a mut W,
}
impl<'a> SCHMITT31_W<'a> {
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
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&self) -> SCHMITT0_R {
        SCHMITT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&self) -> SCHMITT1_R {
        SCHMITT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&self) -> SCHMITT2_R {
        SCHMITT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&self) -> SCHMITT3_R {
        SCHMITT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&self) -> SCHMITT4_R {
        SCHMITT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&self) -> SCHMITT5_R {
        SCHMITT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&self) -> SCHMITT6_R {
        SCHMITT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&self) -> SCHMITT7_R {
        SCHMITT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&self) -> SCHMITT8_R {
        SCHMITT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&self) -> SCHMITT9_R {
        SCHMITT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&self) -> SCHMITT10_R {
        SCHMITT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&self) -> SCHMITT11_R {
        SCHMITT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&self) -> SCHMITT12_R {
        SCHMITT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&self) -> SCHMITT13_R {
        SCHMITT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&self) -> SCHMITT14_R {
        SCHMITT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&self) -> SCHMITT15_R {
        SCHMITT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&self) -> SCHMITT16_R {
        SCHMITT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&self) -> SCHMITT17_R {
        SCHMITT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&self) -> SCHMITT18_R {
        SCHMITT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&self) -> SCHMITT19_R {
        SCHMITT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&self) -> SCHMITT20_R {
        SCHMITT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&self) -> SCHMITT21_R {
        SCHMITT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&self) -> SCHMITT22_R {
        SCHMITT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&self) -> SCHMITT23_R {
        SCHMITT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&self) -> SCHMITT24_R {
        SCHMITT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&self) -> SCHMITT25_R {
        SCHMITT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&self) -> SCHMITT26_R {
        SCHMITT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&self) -> SCHMITT27_R {
        SCHMITT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&self) -> SCHMITT28_R {
        SCHMITT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&self) -> SCHMITT29_R {
        SCHMITT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&self) -> SCHMITT30_R {
        SCHMITT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&self) -> SCHMITT31_R {
        SCHMITT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&mut self) -> SCHMITT0_W {
        SCHMITT0_W { w: self }
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&mut self) -> SCHMITT1_W {
        SCHMITT1_W { w: self }
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&mut self) -> SCHMITT2_W {
        SCHMITT2_W { w: self }
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&mut self) -> SCHMITT3_W {
        SCHMITT3_W { w: self }
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&mut self) -> SCHMITT4_W {
        SCHMITT4_W { w: self }
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&mut self) -> SCHMITT5_W {
        SCHMITT5_W { w: self }
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&mut self) -> SCHMITT6_W {
        SCHMITT6_W { w: self }
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&mut self) -> SCHMITT7_W {
        SCHMITT7_W { w: self }
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&mut self) -> SCHMITT8_W {
        SCHMITT8_W { w: self }
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&mut self) -> SCHMITT9_W {
        SCHMITT9_W { w: self }
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&mut self) -> SCHMITT10_W {
        SCHMITT10_W { w: self }
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&mut self) -> SCHMITT11_W {
        SCHMITT11_W { w: self }
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&mut self) -> SCHMITT12_W {
        SCHMITT12_W { w: self }
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&mut self) -> SCHMITT13_W {
        SCHMITT13_W { w: self }
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&mut self) -> SCHMITT14_W {
        SCHMITT14_W { w: self }
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&mut self) -> SCHMITT15_W {
        SCHMITT15_W { w: self }
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&mut self) -> SCHMITT16_W {
        SCHMITT16_W { w: self }
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&mut self) -> SCHMITT17_W {
        SCHMITT17_W { w: self }
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&mut self) -> SCHMITT18_W {
        SCHMITT18_W { w: self }
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&mut self) -> SCHMITT19_W {
        SCHMITT19_W { w: self }
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&mut self) -> SCHMITT20_W {
        SCHMITT20_W { w: self }
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&mut self) -> SCHMITT21_W {
        SCHMITT21_W { w: self }
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&mut self) -> SCHMITT22_W {
        SCHMITT22_W { w: self }
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&mut self) -> SCHMITT23_W {
        SCHMITT23_W { w: self }
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&mut self) -> SCHMITT24_W {
        SCHMITT24_W { w: self }
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&mut self) -> SCHMITT25_W {
        SCHMITT25_W { w: self }
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&mut self) -> SCHMITT26_W {
        SCHMITT26_W { w: self }
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&mut self) -> SCHMITT27_W {
        SCHMITT27_W { w: self }
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&mut self) -> SCHMITT28_W {
        SCHMITT28_W { w: self }
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&mut self) -> SCHMITT29_W {
        SCHMITT29_W { w: self }
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&mut self) -> SCHMITT30_W {
        SCHMITT30_W { w: self }
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&mut self) -> SCHMITT31_W {
        SCHMITT31_W { w: self }
    }
}
