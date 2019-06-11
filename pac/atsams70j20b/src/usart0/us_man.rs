#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_MAN {
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
pub struct TX_PLR {
    bits: u8,
}
impl TX_PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TX_PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_PPR {
    #[doc = "The preamble is composed of '1's"]
    ALL_ONE,
    #[doc = "The preamble is composed of '0's"]
    ALL_ZERO,
    #[doc = "The preamble is composed of '01's"]
    ZERO_ONE,
    #[doc = "The preamble is composed of '10's"]
    ONE_ZERO,
}
impl TX_PPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TX_PPR::ALL_ONE => 0,
            TX_PPR::ALL_ZERO => 1,
            TX_PPR::ZERO_ONE => 2,
            TX_PPR::ONE_ZERO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TX_PPR {
        match value {
            0 => TX_PPR::ALL_ONE,
            1 => TX_PPR::ALL_ZERO,
            2 => TX_PPR::ZERO_ONE,
            3 => TX_PPR::ONE_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline]
    pub fn is_all_one(&self) -> bool {
        *self == TX_PPR::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline]
    pub fn is_all_zero(&self) -> bool {
        *self == TX_PPR::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline]
    pub fn is_zero_one(&self) -> bool {
        *self == TX_PPR::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline]
    pub fn is_one_zero(&self) -> bool {
        *self == TX_PPR::ONE_ZERO
    }
}
#[doc = r" Value of the field"]
pub struct TX_MPOLR {
    bits: bool,
}
impl TX_MPOLR {
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
pub struct RX_PLR {
    bits: u8,
}
impl RX_PLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RX_PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PPR {
    #[doc = "The preamble is composed of '1's"]
    ALL_ONE,
    #[doc = "The preamble is composed of '0's"]
    ALL_ZERO,
    #[doc = "The preamble is composed of '01's"]
    ZERO_ONE,
    #[doc = "The preamble is composed of '10's"]
    ONE_ZERO,
}
impl RX_PPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RX_PPR::ALL_ONE => 0,
            RX_PPR::ALL_ZERO => 1,
            RX_PPR::ZERO_ONE => 2,
            RX_PPR::ONE_ZERO => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RX_PPR {
        match value {
            0 => RX_PPR::ALL_ONE,
            1 => RX_PPR::ALL_ZERO,
            2 => RX_PPR::ZERO_ONE,
            3 => RX_PPR::ONE_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline]
    pub fn is_all_one(&self) -> bool {
        *self == RX_PPR::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline]
    pub fn is_all_zero(&self) -> bool {
        *self == RX_PPR::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline]
    pub fn is_zero_one(&self) -> bool {
        *self == RX_PPR::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline]
    pub fn is_one_zero(&self) -> bool {
        *self == RX_PPR::ONE_ZERO
    }
}
#[doc = r" Value of the field"]
pub struct RX_MPOLR {
    bits: bool,
}
impl RX_MPOLR {
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
pub struct ONER {
    bits: bool,
}
impl ONER {
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
pub struct DRIFTR {
    bits: bool,
}
impl DRIFTR {
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
pub struct RXIDLEVR {
    bits: bool,
}
impl RXIDLEVR {
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
pub struct _TX_PLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TX_PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_PPW {
    #[doc = "The preamble is composed of '1's"]
    ALL_ONE,
    #[doc = "The preamble is composed of '0's"]
    ALL_ZERO,
    #[doc = "The preamble is composed of '01's"]
    ZERO_ONE,
    #[doc = "The preamble is composed of '10's"]
    ONE_ZERO,
}
impl TX_PPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_PPW::ALL_ONE => 0,
            TX_PPW::ALL_ZERO => 1,
            TX_PPW::ZERO_ONE => 2,
            TX_PPW::ONE_ZERO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TX_PPW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TX_PPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline]
    pub fn all_one(self) -> &'a mut W {
        self.variant(TX_PPW::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(TX_PPW::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(TX_PPW::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(TX_PPW::ONE_ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TX_MPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_MPOLW<'a> {
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
pub struct _RX_PLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PLW<'a> {
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
#[doc = "Values that can be written to the field `RX_PP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_PPW {
    #[doc = "The preamble is composed of '1's"]
    ALL_ONE,
    #[doc = "The preamble is composed of '0's"]
    ALL_ZERO,
    #[doc = "The preamble is composed of '01's"]
    ZERO_ONE,
    #[doc = "The preamble is composed of '10's"]
    ONE_ZERO,
}
impl RX_PPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_PPW::ALL_ONE => 0,
            RX_PPW::ALL_ZERO => 1,
            RX_PPW::ZERO_ONE => 2,
            RX_PPW::ONE_ZERO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RX_PPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RX_PPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline]
    pub fn all_one(self) -> &'a mut W {
        self.variant(RX_PPW::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(RX_PPW::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(RX_PPW::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(RX_PPW::ONE_ZERO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX_MPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MPOLW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRIFTW<'a> {
    w: &'a mut W,
}
impl<'a> _DRIFTW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXIDLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIDLEVW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline]
    pub fn tx_pl(&self) -> TX_PLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TX_PLR { bits }
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline]
    pub fn tx_pp(&self) -> TX_PPR {
        TX_PPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline]
    pub fn tx_mpol(&self) -> TX_MPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TX_MPOLR { bits }
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline]
    pub fn rx_pl(&self) -> RX_PLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RX_PLR { bits }
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline]
    pub fn rx_pp(&self) -> RX_PPR {
        RX_PPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline]
    pub fn rx_mpol(&self) -> RX_MPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX_MPOLR { bits }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline]
    pub fn one(&self) -> ONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONER { bits }
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline]
    pub fn drift(&self) -> DRIFTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DRIFTR { bits }
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline]
    pub fn rxidlev(&self) -> RXIDLEVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXIDLEVR { bits }
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
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline]
    pub fn tx_pl(&mut self) -> _TX_PLW {
        _TX_PLW { w: self }
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline]
    pub fn tx_pp(&mut self) -> _TX_PPW {
        _TX_PPW { w: self }
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline]
    pub fn tx_mpol(&mut self) -> _TX_MPOLW {
        _TX_MPOLW { w: self }
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline]
    pub fn rx_pl(&mut self) -> _RX_PLW {
        _RX_PLW { w: self }
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline]
    pub fn rx_pp(&mut self) -> _RX_PPW {
        _RX_PPW { w: self }
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline]
    pub fn rx_mpol(&mut self) -> _RX_MPOLW {
        _RX_MPOLW { w: self }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline]
    pub fn one(&mut self) -> _ONEW {
        _ONEW { w: self }
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline]
    pub fn drift(&mut self) -> _DRIFTW {
        _DRIFTW { w: self }
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline]
    pub fn rxidlev(&mut self) -> _RXIDLEVW {
        _RXIDLEVW { w: self }
    }
}
