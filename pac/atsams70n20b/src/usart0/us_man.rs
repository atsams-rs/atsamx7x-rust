#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_MAN {
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
pub type TX_PL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TX_PLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
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
impl crate::ToBits<u8> for TX_PPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TX_PPR::ALL_ONE => 0,
            TX_PPR::ALL_ZERO => 1,
            TX_PPR::ZERO_ONE => 2,
            TX_PPR::ONE_ZERO => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TX_PP_R = crate::FR<u8, TX_PPR>;
impl TX_PP_R {
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == TX_PPR::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == TX_PPR::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == TX_PPR::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == TX_PPR::ONE_ZERO
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TX_PPW::ALL_ONE => 0,
            TX_PPW::ALL_ZERO => 1,
            TX_PPW::ZERO_ONE => 2,
            TX_PPW::ONE_ZERO => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TX_PPW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_PPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_PPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut W {
        self.variant(TX_PPW::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(TX_PPW::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(TX_PPW::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(TX_PPW::ONE_ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TX_MPOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TX_MPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _TX_MPOLW<'a> {
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
pub type RX_PL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _RX_PLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
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
impl crate::ToBits<u8> for RX_PPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RX_PPR::ALL_ONE => 0,
            RX_PPR::ALL_ZERO => 1,
            RX_PPR::ZERO_ONE => 2,
            RX_PPR::ONE_ZERO => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RX_PP_R = crate::FR<u8, RX_PPR>;
impl RX_PP_R {
    #[doc = "Checks if the value of the field is `ALL_ONE`"]
    #[inline(always)]
    pub fn is_all_one(&self) -> bool {
        *self == RX_PPR::ALL_ONE
    }
    #[doc = "Checks if the value of the field is `ALL_ZERO`"]
    #[inline(always)]
    pub fn is_all_zero(&self) -> bool {
        *self == RX_PPR::ALL_ZERO
    }
    #[doc = "Checks if the value of the field is `ZERO_ONE`"]
    #[inline(always)]
    pub fn is_zero_one(&self) -> bool {
        *self == RX_PPR::ZERO_ONE
    }
    #[doc = "Checks if the value of the field is `ONE_ZERO`"]
    #[inline(always)]
    pub fn is_one_zero(&self) -> bool {
        *self == RX_PPR::ONE_ZERO
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            RX_PPW::ALL_ONE => 0,
            RX_PPW::ALL_ZERO => 1,
            RX_PPW::ZERO_ONE => 2,
            RX_PPW::ONE_ZERO => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RX_PPW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_PPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_PPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The preamble is composed of '1's"]
    #[inline(always)]
    pub fn all_one(self) -> &'a mut W {
        self.variant(RX_PPW::ALL_ONE)
    }
    #[doc = "The preamble is composed of '0's"]
    #[inline(always)]
    pub fn all_zero(self) -> &'a mut W {
        self.variant(RX_PPW::ALL_ZERO)
    }
    #[doc = "The preamble is composed of '01's"]
    #[inline(always)]
    pub fn zero_one(self) -> &'a mut W {
        self.variant(RX_PPW::ZERO_ONE)
    }
    #[doc = "The preamble is composed of '10's"]
    #[inline(always)]
    pub fn one_zero(self) -> &'a mut W {
        self.variant(RX_PPW::ONE_ZERO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RX_MPOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RX_MPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _RX_MPOLW<'a> {
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
pub type ONE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEW<'a> {
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
pub type DRIFT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRIFTW<'a> {
    w: &'a mut W,
}
impl<'a> _DRIFTW<'a> {
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
pub type RXIDLEV_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXIDLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIDLEVW<'a> {
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
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TX_PL_R {
        TX_PL_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TX_PP_R {
        TX_PP_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TX_MPOL_R {
        TX_MPOL_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RX_PL_R {
        RX_PL_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RX_PP_R {
        RX_PP_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RX_MPOL_R {
        RX_MPOL_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DRIFT_R {
        DRIFT_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&self) -> RXIDLEV_R {
        RXIDLEV_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&mut self) -> _TX_PLW {
        _TX_PLW { w: self }
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&mut self) -> _TX_PPW {
        _TX_PPW { w: self }
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&mut self) -> _TX_MPOLW {
        _TX_MPOLW { w: self }
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&mut self) -> _RX_PLW {
        _RX_PLW { w: self }
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&mut self) -> _RX_PPW {
        _RX_PPW { w: self }
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&mut self) -> _RX_MPOLW {
        _RX_MPOLW { w: self }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> _ONEW {
        _ONEW { w: self }
    }
    #[doc = "Bit 30 - Drift Compensation"]
    #[inline(always)]
    pub fn drift(&mut self) -> _DRIFTW {
        _DRIFTW { w: self }
    }
    #[doc = "Bit 31 - Receiver Idle Value"]
    #[inline(always)]
    pub fn rxidlev(&mut self) -> _RXIDLEVW {
        _RXIDLEVW { w: self }
    }
}
