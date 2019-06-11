#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_LINMR {
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
#[doc = "Possible values of the field `NACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACTR {
    #[doc = "The USART transmits the response."]
    PUBLISH,
    #[doc = "The USART receives the response."]
    SUBSCRIBE,
    #[doc = "The USART does not transmit and does not receive the response."]
    IGNORE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NACTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NACTR::PUBLISH => 0,
            NACTR::SUBSCRIBE => 1,
            NACTR::IGNORE => 2,
            NACTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NACTR {
        match value {
            0 => NACTR::PUBLISH,
            1 => NACTR::SUBSCRIBE,
            2 => NACTR::IGNORE,
            i => NACTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PUBLISH`"]
    #[inline]
    pub fn is_publish(&self) -> bool {
        *self == NACTR::PUBLISH
    }
    #[doc = "Checks if the value of the field is `SUBSCRIBE`"]
    #[inline]
    pub fn is_subscribe(&self) -> bool {
        *self == NACTR::SUBSCRIBE
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline]
    pub fn is_ignore(&self) -> bool {
        *self == NACTR::IGNORE
    }
}
#[doc = r" Value of the field"]
pub struct PARDISR {
    bits: bool,
}
impl PARDISR {
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
pub struct CHKDISR {
    bits: bool,
}
impl CHKDISR {
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
pub struct CHKTYPR {
    bits: bool,
}
impl CHKTYPR {
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
pub struct DLMR {
    bits: bool,
}
impl DLMR {
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
pub struct FSDISR {
    bits: bool,
}
impl FSDISR {
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
pub struct WKUPTYPR {
    bits: bool,
}
impl WKUPTYPR {
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
pub struct DLCR {
    bits: u8,
}
impl DLCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PDCMR {
    bits: bool,
}
impl PDCMR {
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
pub struct SYNCDISR {
    bits: bool,
}
impl SYNCDISR {
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
#[doc = "Values that can be written to the field `NACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACTW {
    #[doc = "The USART transmits the response."]
    PUBLISH,
    #[doc = "The USART receives the response."]
    SUBSCRIBE,
    #[doc = "The USART does not transmit and does not receive the response."]
    IGNORE,
}
impl NACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NACTW::PUBLISH => 0,
            NACTW::SUBSCRIBE => 1,
            NACTW::IGNORE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACTW<'a> {
    w: &'a mut W,
}
impl<'a> _NACTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The USART transmits the response."]
    #[inline]
    pub fn publish(self) -> &'a mut W {
        self.variant(NACTW::PUBLISH)
    }
    #[doc = "The USART receives the response."]
    #[inline]
    pub fn subscribe(self) -> &'a mut W {
        self.variant(NACTW::SUBSCRIBE)
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline]
    pub fn ignore(self) -> &'a mut W {
        self.variant(NACTW::IGNORE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PARDISW<'a> {
    w: &'a mut W,
}
impl<'a> _PARDISW<'a> {
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
pub struct _CHKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CHKDISW<'a> {
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
pub struct _CHKTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _CHKTYPW<'a> {
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
pub struct _DLMW<'a> {
    w: &'a mut W,
}
impl<'a> _DLMW<'a> {
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
pub struct _FSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDISW<'a> {
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
pub struct _WKUPTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPTYPW<'a> {
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
pub struct _DLCW<'a> {
    w: &'a mut W,
}
impl<'a> _DLCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PDCMW<'a> {
    w: &'a mut W,
}
impl<'a> _PDCMW<'a> {
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
pub struct _SYNCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCDISW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline]
    pub fn nact(&self) -> NACTR {
        NACTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline]
    pub fn pardis(&self) -> PARDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PARDISR { bits }
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline]
    pub fn chkdis(&self) -> CHKDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHKDISR { bits }
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline]
    pub fn chktyp(&self) -> CHKTYPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHKTYPR { bits }
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline]
    pub fn dlm(&self) -> DLMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DLMR { bits }
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline]
    pub fn fsdis(&self) -> FSDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSDISR { bits }
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline]
    pub fn wkuptyp(&self) -> WKUPTYPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WKUPTYPR { bits }
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline]
    pub fn dlc(&self) -> DLCR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLCR { bits }
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline]
    pub fn pdcm(&self) -> PDCMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PDCMR { bits }
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline]
    pub fn syncdis(&self) -> SYNCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCDISR { bits }
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
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline]
    pub fn nact(&mut self) -> _NACTW {
        _NACTW { w: self }
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline]
    pub fn pardis(&mut self) -> _PARDISW {
        _PARDISW { w: self }
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline]
    pub fn chkdis(&mut self) -> _CHKDISW {
        _CHKDISW { w: self }
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline]
    pub fn chktyp(&mut self) -> _CHKTYPW {
        _CHKTYPW { w: self }
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline]
    pub fn dlm(&mut self) -> _DLMW {
        _DLMW { w: self }
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline]
    pub fn fsdis(&mut self) -> _FSDISW {
        _FSDISW { w: self }
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline]
    pub fn wkuptyp(&mut self) -> _WKUPTYPW {
        _WKUPTYPW { w: self }
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline]
    pub fn dlc(&mut self) -> _DLCW {
        _DLCW { w: self }
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline]
    pub fn pdcm(&mut self) -> _PDCMW {
        _PDCMW { w: self }
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline]
    pub fn syncdis(&mut self) -> _SYNCDISW {
        _SYNCDISW { w: self }
    }
}
