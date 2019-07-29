#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_FSMR {
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
pub type FSTT0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT0W<'a> {
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
pub type FSTT1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT1W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT1W<'a> {
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
pub type FSTT2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT2W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT2W<'a> {
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
pub type FSTT3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT3W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT3W<'a> {
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
pub type FSTT4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT4W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT4W<'a> {
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
pub type FSTT5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT5W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT5W<'a> {
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
pub type FSTT6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT6W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT6W<'a> {
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
pub type FSTT7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT7W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT7W<'a> {
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
pub type FSTT8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT8W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT8W<'a> {
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
pub type FSTT9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT9W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT9W<'a> {
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
pub type FSTT10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT10W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT10W<'a> {
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
pub type FSTT11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT11W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT11W<'a> {
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
pub type FSTT12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT12W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT12W<'a> {
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
pub type FSTT13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT13W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT13W<'a> {
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
pub type FSTT14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT14W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT14W<'a> {
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
pub type FSTT15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTT15W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT15W<'a> {
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
pub type RTTAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RTTALW<'a> {
    w: &'a mut W,
}
impl<'a> _RTTALW<'a> {
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
pub type RTCAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RTCALW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCALW<'a> {
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
pub type USBAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _USBALW<'a> {
    w: &'a mut W,
}
impl<'a> _USBALW<'a> {
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
pub type LPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMW<'a> {
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
#[doc = "Possible values of the field `FLPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLPMR {
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    FLASH_STANDBY,
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    FLASH_DEEP_POWERDOWN,
    #[doc = "Idle mode"]
    FLASH_IDLE,
}
impl crate::ToBits<u8> for FLPMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FLPMR::FLASH_STANDBY => 0,
            FLPMR::FLASH_DEEP_POWERDOWN => 1,
            FLPMR::FLASH_IDLE => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FLPM_R = crate::FR<u8, FLPMR>;
impl FLPM_R {
    #[doc = "Checks if the value of the field is `FLASH_STANDBY`"]
    #[inline(always)]
    pub fn is_flash_standby(&self) -> bool {
        *self == FLPMR::FLASH_STANDBY
    }
    #[doc = "Checks if the value of the field is `FLASH_DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_flash_deep_powerdown(&self) -> bool {
        *self == FLPMR::FLASH_DEEP_POWERDOWN
    }
    #[doc = "Checks if the value of the field is `FLASH_IDLE`"]
    #[inline(always)]
    pub fn is_flash_idle(&self) -> bool {
        *self == FLPMR::FLASH_IDLE
    }
}
#[doc = "Values that can be written to the field `FLPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLPMW {
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    FLASH_STANDBY,
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    FLASH_DEEP_POWERDOWN,
    #[doc = "Idle mode"]
    FLASH_IDLE,
}
impl FLPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLPMW::FLASH_STANDBY => 0,
            FLPMW::FLASH_DEEP_POWERDOWN => 1,
            FLPMW::FLASH_IDLE => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_standby(self) -> &'a mut W {
        self.variant(FLPMW::FLASH_STANDBY)
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_deep_powerdown(self) -> &'a mut W {
        self.variant(FLPMW::FLASH_DEEP_POWERDOWN)
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn flash_idle(self) -> &'a mut W {
        self.variant(FLPMW::FLASH_IDLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FFLPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FFLPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLPMW<'a> {
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
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> FSTT0_R {
        FSTT0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> FSTT1_R {
        FSTT1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> FSTT2_R {
        FSTT2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> FSTT3_R {
        FSTT3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> FSTT4_R {
        FSTT4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> FSTT5_R {
        FSTT5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> FSTT6_R {
        FSTT6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> FSTT7_R {
        FSTT7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> FSTT8_R {
        FSTT8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> FSTT9_R {
        FSTT9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> FSTT10_R {
        FSTT10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> FSTT11_R {
        FSTT11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> FSTT12_R {
        FSTT12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> FSTT13_R {
        FSTT13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> FSTT14_R {
        FSTT14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> FSTT15_R {
        FSTT15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RTTAL_R {
        RTTAL_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RTCAL_R {
        RTCAL_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> USBAL_R {
        USBAL_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&self) -> FLPM_R {
        FLPM_R::new(((self.bits() >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&self) -> FFLPM_R {
        FFLPM_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&mut self) -> _FSTT0W {
        _FSTT0W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&mut self) -> _FSTT1W {
        _FSTT1W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&mut self) -> _FSTT2W {
        _FSTT2W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&mut self) -> _FSTT3W {
        _FSTT3W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&mut self) -> _FSTT4W {
        _FSTT4W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&mut self) -> _FSTT5W {
        _FSTT5W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&mut self) -> _FSTT6W {
        _FSTT6W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&mut self) -> _FSTT7W {
        _FSTT7W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&mut self) -> _FSTT8W {
        _FSTT8W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&mut self) -> _FSTT9W {
        _FSTT9W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&mut self) -> _FSTT10W {
        _FSTT10W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&mut self) -> _FSTT11W {
        _FSTT11W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&mut self) -> _FSTT12W {
        _FSTT12W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&mut self) -> _FSTT13W {
        _FSTT13W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&mut self) -> _FSTT14W {
        _FSTT14W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&mut self) -> _FSTT15W {
        _FSTT15W { w: self }
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&mut self) -> _RTTALW {
        _RTTALW { w: self }
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&mut self) -> _RTCALW {
        _RTCALW { w: self }
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&mut self) -> _USBALW {
        _USBALW { w: self }
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> _LPMW {
        _LPMW { w: self }
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&mut self) -> _FLPMW {
        _FLPMW { w: self }
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&mut self) -> _FFLPMW {
        _FFLPMW { w: self }
    }
}
