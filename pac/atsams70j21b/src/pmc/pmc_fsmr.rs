#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_FSMR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct FSTT0R {
    bits: bool,
}
impl FSTT0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT1R {
    bits: bool,
}
impl FSTT1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT2R {
    bits: bool,
}
impl FSTT2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT3R {
    bits: bool,
}
impl FSTT3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT4R {
    bits: bool,
}
impl FSTT4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT5R {
    bits: bool,
}
impl FSTT5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT6R {
    bits: bool,
}
impl FSTT6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT7R {
    bits: bool,
}
impl FSTT7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT8R {
    bits: bool,
}
impl FSTT8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT9R {
    bits: bool,
}
impl FSTT9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT10R {
    bits: bool,
}
impl FSTT10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT11R {
    bits: bool,
}
impl FSTT11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT12R {
    bits: bool,
}
impl FSTT12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT13R {
    bits: bool,
}
impl FSTT13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT14R {
    bits: bool,
}
impl FSTT14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FSTT15R {
    bits: bool,
}
impl FSTT15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RTTALR {
    bits: bool,
}
impl RTTALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RTCALR {
    bits: bool,
}
impl RTCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct USBALR {
    bits: bool,
}
impl USBALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct LPMR {
    bits: bool,
}
impl LPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FLPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FLPMR::FLASH_STANDBY => 0,
            FLPMR::FLASH_DEEP_POWERDOWN => 1,
            FLPMR::FLASH_IDLE => 2,
            FLPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FLPMR {
        match value {
            0 => FLPMR::FLASH_STANDBY,
            1 => FLPMR::FLASH_DEEP_POWERDOWN,
            2 => FLPMR::FLASH_IDLE,
            i => FLPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_STANDBY`"]
    #[inline]
    pub fn is_flash_standby(&self) -> bool {
        *self == FLPMR::FLASH_STANDBY
    }
    #[doc = "Checks if the value of the field is `FLASH_DEEP_POWERDOWN`"]
    #[inline]
    pub fn is_flash_deep_powerdown(&self) -> bool {
        *self == FLPMR::FLASH_DEEP_POWERDOWN
    }
    #[doc = "Checks if the value of the field is `FLASH_IDLE`"]
    #[inline]
    pub fn is_flash_idle(&self) -> bool {
        *self == FLPMR::FLASH_IDLE
    }
}
#[doc = r" Value of the field"]
pub struct FFLPMR {
    bits: bool,
}
impl FFLPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _FSTT0W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT1W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT2W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT3W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT3W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT4W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT4W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT5W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT5W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT6W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT6W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT7W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT7W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT8W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT8W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT9W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT9W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT10W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT10W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT11W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT11W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT12W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT12W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT13W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT13W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT14W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT14W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSTT15W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTT15W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTTALW<'a> {
    w: &'a mut W,
}
impl<'a> _RTTALW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTCALW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCALW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _USBALW<'a> {
    w: &'a mut W,
}
impl<'a> _USBALW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPMW<'a> {
    w: &'a mut W,
}
impl<'a> _LPMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLPMW::FLASH_STANDBY => 0,
            FLPMW::FLASH_DEEP_POWERDOWN => 1,
            FLPMW::FLASH_IDLE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FLPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline]
    pub fn flash_standby(self) -> &'a mut W {
        self.variant(FLPMW::FLASH_STANDBY)
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline]
    pub fn flash_deep_powerdown(self) -> &'a mut W {
        self.variant(FLPMW::FLASH_DEEP_POWERDOWN)
    }
    #[doc = "Idle mode"]
    #[inline]
    pub fn flash_idle(self) -> &'a mut W {
        self.variant(FLPMW::FLASH_IDLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FFLPMW<'a> {
    w: &'a mut W,
}
impl<'a> _FFLPMW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline]
    pub fn fstt0(&self) -> FSTT0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT0R { bits }
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline]
    pub fn fstt1(&self) -> FSTT1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT1R { bits }
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline]
    pub fn fstt2(&self) -> FSTT2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT2R { bits }
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline]
    pub fn fstt3(&self) -> FSTT3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT3R { bits }
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline]
    pub fn fstt4(&self) -> FSTT4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT4R { bits }
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline]
    pub fn fstt5(&self) -> FSTT5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT5R { bits }
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline]
    pub fn fstt6(&self) -> FSTT6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT6R { bits }
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline]
    pub fn fstt7(&self) -> FSTT7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT7R { bits }
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline]
    pub fn fstt8(&self) -> FSTT8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT8R { bits }
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline]
    pub fn fstt9(&self) -> FSTT9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT9R { bits }
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline]
    pub fn fstt10(&self) -> FSTT10R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT10R { bits }
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline]
    pub fn fstt11(&self) -> FSTT11R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT11R { bits }
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline]
    pub fn fstt12(&self) -> FSTT12R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT12R { bits }
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline]
    pub fn fstt13(&self) -> FSTT13R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT13R { bits }
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline]
    pub fn fstt14(&self) -> FSTT14R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT14R { bits }
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline]
    pub fn fstt15(&self) -> FSTT15R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSTT15R { bits }
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline]
    pub fn rttal(&self) -> RTTALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTTALR { bits }
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline]
    pub fn rtcal(&self) -> RTCALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCALR { bits }
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline]
    pub fn usbal(&self) -> USBALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        USBALR { bits }
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline]
    pub fn lpm(&self) -> LPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPMR { bits }
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline]
    pub fn flpm(&self) -> FLPMR {
        FLPMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline]
    pub fn fflpm(&self) -> FFLPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FFLPMR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline]
    pub fn fstt0(&mut self) -> _FSTT0W {
        _FSTT0W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline]
    pub fn fstt1(&mut self) -> _FSTT1W {
        _FSTT1W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline]
    pub fn fstt2(&mut self) -> _FSTT2W {
        _FSTT2W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline]
    pub fn fstt3(&mut self) -> _FSTT3W {
        _FSTT3W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline]
    pub fn fstt4(&mut self) -> _FSTT4W {
        _FSTT4W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline]
    pub fn fstt5(&mut self) -> _FSTT5W {
        _FSTT5W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline]
    pub fn fstt6(&mut self) -> _FSTT6W {
        _FSTT6W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline]
    pub fn fstt7(&mut self) -> _FSTT7W {
        _FSTT7W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline]
    pub fn fstt8(&mut self) -> _FSTT8W {
        _FSTT8W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline]
    pub fn fstt9(&mut self) -> _FSTT9W {
        _FSTT9W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline]
    pub fn fstt10(&mut self) -> _FSTT10W {
        _FSTT10W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline]
    pub fn fstt11(&mut self) -> _FSTT11W {
        _FSTT11W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline]
    pub fn fstt12(&mut self) -> _FSTT12W {
        _FSTT12W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline]
    pub fn fstt13(&mut self) -> _FSTT13W {
        _FSTT13W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline]
    pub fn fstt14(&mut self) -> _FSTT14W {
        _FSTT14W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline]
    pub fn fstt15(&mut self) -> _FSTT15W {
        _FSTT15W { w: self }
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline]
    pub fn rttal(&mut self) -> _RTTALW {
        _RTTALW { w: self }
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline]
    pub fn rtcal(&mut self) -> _RTCALW {
        _RTCALW { w: self }
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline]
    pub fn usbal(&mut self) -> _USBALW {
        _USBALW { w: self }
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline]
    pub fn lpm(&mut self) -> _LPMW {
        _LPMW { w: self }
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline]
    pub fn flpm(&mut self) -> _FLPMW {
        _FLPMW { w: self }
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline]
    pub fn fflpm(&mut self) -> _FFLPMW {
        _FFLPMW { w: self }
    }
}
