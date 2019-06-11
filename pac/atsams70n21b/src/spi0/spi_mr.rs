#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPI_MR {
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
#[doc = "Possible values of the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRR {
    #[doc = "Master"]
    MASTER,
    #[doc = "Slave"]
    SLAVE,
}
impl MSTRR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            MSTRR::MASTER => true,
            MSTRR::SLAVE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTRR {
        match value {
            true => MSTRR::MASTER,
            false => MSTRR::SLAVE,
        }
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSTRR::MASTER
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSTRR::SLAVE
    }
}
#[doc = r" Value of the field"]
pub struct PSR {
    bits: bool,
}
impl PSR {
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
pub struct PCSDECR {
    bits: bool,
}
impl PCSDECR {
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
pub struct MODFDISR {
    bits: bool,
}
impl MODFDISR {
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
pub struct WDRBTR {
    bits: bool,
}
impl WDRBTR {
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
pub struct LLBR {
    bits: bool,
}
impl LLBR {
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
#[doc = "Possible values of the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSR {
    #[doc = "NPCS0 as Chip Select"]
    NPCS0,
    #[doc = "NPCS1 as Chip Select"]
    NPCS1,
    #[doc = "NPCS2 as Chip Select"]
    NPCS2,
    #[doc = "NPCS3 as Chip Select"]
    NPCS3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSR::NPCS0 => 14,
            PCSR::NPCS1 => 13,
            PCSR::NPCS2 => 11,
            PCSR::NPCS3 => 7,
            PCSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSR {
        match value {
            14 => PCSR::NPCS0,
            13 => PCSR::NPCS1,
            11 => PCSR::NPCS2,
            7 => PCSR::NPCS3,
            i => PCSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NPCS0`"]
    #[inline]
    pub fn is_npcs0(&self) -> bool {
        *self == PCSR::NPCS0
    }
    #[doc = "Checks if the value of the field is `NPCS1`"]
    #[inline]
    pub fn is_npcs1(&self) -> bool {
        *self == PCSR::NPCS1
    }
    #[doc = "Checks if the value of the field is `NPCS2`"]
    #[inline]
    pub fn is_npcs2(&self) -> bool {
        *self == PCSR::NPCS2
    }
    #[doc = "Checks if the value of the field is `NPCS3`"]
    #[inline]
    pub fn is_npcs3(&self) -> bool {
        *self == PCSR::NPCS3
    }
}
#[doc = r" Value of the field"]
pub struct DLYBCSR {
    bits: u8,
}
impl DLYBCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRW {
    #[doc = "Master"]
    MASTER,
    #[doc = "Slave"]
    SLAVE,
}
impl MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTRW::MASTER => true,
            MSTRW::SLAVE => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master"]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTRW::MASTER)
    }
    #[doc = "Slave"]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTRW::SLAVE)
    }
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
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
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
pub struct _PCSDECW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSDECW<'a> {
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
pub struct _MODFDISW<'a> {
    w: &'a mut W,
}
impl<'a> _MODFDISW<'a> {
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
pub struct _WDRBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRBTW<'a> {
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
pub struct _LLBW<'a> {
    w: &'a mut W,
}
impl<'a> _LLBW<'a> {
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
#[doc = "Values that can be written to the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSW {
    #[doc = "NPCS0 as Chip Select"]
    NPCS0,
    #[doc = "NPCS1 as Chip Select"]
    NPCS1,
    #[doc = "NPCS2 as Chip Select"]
    NPCS2,
    #[doc = "NPCS3 as Chip Select"]
    NPCS3,
}
impl PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSW::NPCS0 => 14,
            PCSW::NPCS1 => 13,
            PCSW::NPCS2 => 11,
            PCSW::NPCS3 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "NPCS0 as Chip Select"]
    #[inline]
    pub fn npcs0(self) -> &'a mut W {
        self.variant(PCSW::NPCS0)
    }
    #[doc = "NPCS1 as Chip Select"]
    #[inline]
    pub fn npcs1(self) -> &'a mut W {
        self.variant(PCSW::NPCS1)
    }
    #[doc = "NPCS2 as Chip Select"]
    #[inline]
    pub fn npcs2(self) -> &'a mut W {
        self.variant(PCSW::NPCS2)
    }
    #[doc = "NPCS3 as Chip Select"]
    #[inline]
    pub fn npcs3(self) -> &'a mut W {
        self.variant(PCSW::NPCS3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        MSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline]
    pub fn ps(&self) -> PSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PSR { bits }
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline]
    pub fn pcsdec(&self) -> PCSDECR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCSDECR { bits }
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline]
    pub fn modfdis(&self) -> MODFDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODFDISR { bits }
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline]
    pub fn wdrbt(&self) -> WDRBTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WDRBTR { bits }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline]
    pub fn llb(&self) -> LLBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LLBR { bits }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        PCSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline]
    pub fn dlybcs(&self) -> DLYBCSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBCSR { bits }
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
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline]
    pub fn pcsdec(&mut self) -> _PCSDECW {
        _PCSDECW { w: self }
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline]
    pub fn modfdis(&mut self) -> _MODFDISW {
        _MODFDISW { w: self }
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline]
    pub fn wdrbt(&mut self) -> _WDRBTW {
        _WDRBTW { w: self }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline]
    pub fn llb(&mut self) -> _LLBW {
        _LLBW { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline]
    pub fn dlybcs(&mut self) -> _DLYBCSW {
        _DLYBCSW { w: self }
    }
}
