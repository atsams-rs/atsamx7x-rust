#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_SCM {
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
pub struct SYNC0R {
    bits: bool,
}
impl SYNC0R {
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
pub struct SYNC1R {
    bits: bool,
}
impl SYNC1R {
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
pub struct SYNC2R {
    bits: bool,
}
impl SYNC2R {
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
pub struct SYNC3R {
    bits: bool,
}
impl SYNC3R {
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
#[doc = "Possible values of the field `UPDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDMR {
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    MODE0,
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    MODE1,
    #[doc = "Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    MODE2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl UPDMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            UPDMR::MODE0 => 0,
            UPDMR::MODE1 => 1,
            UPDMR::MODE2 => 2,
            UPDMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> UPDMR {
        match value {
            0 => UPDMR::MODE0,
            1 => UPDMR::MODE1,
            2 => UPDMR::MODE2,
            i => UPDMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline]
    pub fn is_mode0(&self) -> bool {
        *self == UPDMR::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == UPDMR::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == UPDMR::MODE2
    }
}
#[doc = r" Value of the field"]
pub struct PTRMR {
    bits: bool,
}
impl PTRMR {
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
pub struct PTRCSR {
    bits: u8,
}
impl PTRCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SYNC0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC0W<'a> {
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
pub struct _SYNC1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC1W<'a> {
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
pub struct _SYNC2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC2W<'a> {
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
pub struct _SYNC3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYNC3W<'a> {
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
#[doc = "Values that can be written to the field `UPDM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDMW {
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    MODE0,
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    MODE1,
    #[doc = "Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    MODE2,
}
impl UPDMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            UPDMW::MODE0 => 0,
            UPDMW::MODE1 => 1,
            UPDMW::MODE2 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDMW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline]
    pub fn mode0(self) -> &'a mut W {
        self.variant(UPDMW::MODE0)
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(UPDMW::MODE1)
    }
    #[doc = "Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(UPDMW::MODE2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PTRMW<'a> {
    w: &'a mut W,
}
impl<'a> _PTRMW<'a> {
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
#[doc = r" Proxy"]
pub struct _PTRCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PTRCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 21;
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
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline]
    pub fn sync0(&self) -> SYNC0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNC0R { bits }
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline]
    pub fn sync1(&self) -> SYNC1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNC1R { bits }
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline]
    pub fn sync2(&self) -> SYNC2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNC2R { bits }
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline]
    pub fn sync3(&self) -> SYNC3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNC3R { bits }
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline]
    pub fn updm(&self) -> UPDMR {
        UPDMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - DMA Controller Transfer Request Mode"]
    #[inline]
    pub fn ptrm(&self) -> PTRMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PTRMR { bits }
    }
    #[doc = "Bits 21:23 - DMA Controller Transfer Request Comparison Selection"]
    #[inline]
    pub fn ptrcs(&self) -> PTRCSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PTRCSR { bits }
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
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline]
    pub fn sync0(&mut self) -> _SYNC0W {
        _SYNC0W { w: self }
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline]
    pub fn sync1(&mut self) -> _SYNC1W {
        _SYNC1W { w: self }
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline]
    pub fn sync2(&mut self) -> _SYNC2W {
        _SYNC2W { w: self }
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline]
    pub fn sync3(&mut self) -> _SYNC3W {
        _SYNC3W { w: self }
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline]
    pub fn updm(&mut self) -> _UPDMW {
        _UPDMW { w: self }
    }
    #[doc = "Bit 20 - DMA Controller Transfer Request Mode"]
    #[inline]
    pub fn ptrm(&mut self) -> _PTRMW {
        _PTRMW { w: self }
    }
    #[doc = "Bits 21:23 - DMA Controller Transfer Request Comparison Selection"]
    #[inline]
    pub fn ptrcs(&mut self) -> _PTRCSW {
        _PTRCSW { w: self }
    }
}
