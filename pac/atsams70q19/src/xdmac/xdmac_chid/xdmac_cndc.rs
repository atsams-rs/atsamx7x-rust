#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_CNDC {
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
#[doc = "Possible values of the field `NDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDER {
    #[doc = "Descriptor fetch is disabled."]
    DSCR_FETCH_DIS,
    #[doc = "Descriptor fetch is enabled."]
    DSCR_FETCH_EN,
}
impl NDER {
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
            NDER::DSCR_FETCH_DIS => false,
            NDER::DSCR_FETCH_EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDER {
        match value {
            false => NDER::DSCR_FETCH_DIS,
            true => NDER::DSCR_FETCH_EN,
        }
    }
    #[doc = "Checks if the value of the field is `DSCR_FETCH_DIS`"]
    #[inline]
    pub fn is_dscr_fetch_dis(&self) -> bool {
        *self == NDER::DSCR_FETCH_DIS
    }
    #[doc = "Checks if the value of the field is `DSCR_FETCH_EN`"]
    #[inline]
    pub fn is_dscr_fetch_en(&self) -> bool {
        *self == NDER::DSCR_FETCH_EN
    }
}
#[doc = "Possible values of the field `NDSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDSUPR {
    #[doc = "Source parameters remain unchanged."]
    SRC_PARAMS_UNCHANGED,
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    SRC_PARAMS_UPDATED,
}
impl NDSUPR {
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
            NDSUPR::SRC_PARAMS_UNCHANGED => false,
            NDSUPR::SRC_PARAMS_UPDATED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDSUPR {
        match value {
            false => NDSUPR::SRC_PARAMS_UNCHANGED,
            true => NDSUPR::SRC_PARAMS_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `SRC_PARAMS_UNCHANGED`"]
    #[inline]
    pub fn is_src_params_unchanged(&self) -> bool {
        *self == NDSUPR::SRC_PARAMS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `SRC_PARAMS_UPDATED`"]
    #[inline]
    pub fn is_src_params_updated(&self) -> bool {
        *self == NDSUPR::SRC_PARAMS_UPDATED
    }
}
#[doc = "Possible values of the field `NDDUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDUPR {
    #[doc = "Destination parameters remain unchanged."]
    DST_PARAMS_UNCHANGED,
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    DST_PARAMS_UPDATED,
}
impl NDDUPR {
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
            NDDUPR::DST_PARAMS_UNCHANGED => false,
            NDDUPR::DST_PARAMS_UPDATED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDDUPR {
        match value {
            false => NDDUPR::DST_PARAMS_UNCHANGED,
            true => NDDUPR::DST_PARAMS_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `DST_PARAMS_UNCHANGED`"]
    #[inline]
    pub fn is_dst_params_unchanged(&self) -> bool {
        *self == NDDUPR::DST_PARAMS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `DST_PARAMS_UPDATED`"]
    #[inline]
    pub fn is_dst_params_updated(&self) -> bool {
        *self == NDDUPR::DST_PARAMS_UPDATED
    }
}
#[doc = "Possible values of the field `NDVIEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDVIEWR {
    #[doc = "Next Descriptor View 0"]
    NDV0,
    #[doc = "Next Descriptor View 1"]
    NDV1,
    #[doc = "Next Descriptor View 2"]
    NDV2,
    #[doc = "Next Descriptor View 3"]
    NDV3,
}
impl NDVIEWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NDVIEWR::NDV0 => 0,
            NDVIEWR::NDV1 => 1,
            NDVIEWR::NDV2 => 2,
            NDVIEWR::NDV3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NDVIEWR {
        match value {
            0 => NDVIEWR::NDV0,
            1 => NDVIEWR::NDV1,
            2 => NDVIEWR::NDV2,
            3 => NDVIEWR::NDV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NDV0`"]
    #[inline]
    pub fn is_ndv0(&self) -> bool {
        *self == NDVIEWR::NDV0
    }
    #[doc = "Checks if the value of the field is `NDV1`"]
    #[inline]
    pub fn is_ndv1(&self) -> bool {
        *self == NDVIEWR::NDV1
    }
    #[doc = "Checks if the value of the field is `NDV2`"]
    #[inline]
    pub fn is_ndv2(&self) -> bool {
        *self == NDVIEWR::NDV2
    }
    #[doc = "Checks if the value of the field is `NDV3`"]
    #[inline]
    pub fn is_ndv3(&self) -> bool {
        *self == NDVIEWR::NDV3
    }
}
#[doc = "Values that can be written to the field `NDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDEW {
    #[doc = "Descriptor fetch is disabled."]
    DSCR_FETCH_DIS,
    #[doc = "Descriptor fetch is enabled."]
    DSCR_FETCH_EN,
}
impl NDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDEW::DSCR_FETCH_DIS => false,
            NDEW::DSCR_FETCH_EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDEW<'a> {
    w: &'a mut W,
}
impl<'a> _NDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Descriptor fetch is disabled."]
    #[inline]
    pub fn dscr_fetch_dis(self) -> &'a mut W {
        self.variant(NDEW::DSCR_FETCH_DIS)
    }
    #[doc = "Descriptor fetch is enabled."]
    #[inline]
    pub fn dscr_fetch_en(self) -> &'a mut W {
        self.variant(NDEW::DSCR_FETCH_EN)
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
#[doc = "Values that can be written to the field `NDSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDSUPW {
    #[doc = "Source parameters remain unchanged."]
    SRC_PARAMS_UNCHANGED,
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    SRC_PARAMS_UPDATED,
}
impl NDSUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDSUPW::SRC_PARAMS_UNCHANGED => false,
            NDSUPW::SRC_PARAMS_UPDATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NDSUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDSUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Source parameters remain unchanged."]
    #[inline]
    pub fn src_params_unchanged(self) -> &'a mut W {
        self.variant(NDSUPW::SRC_PARAMS_UNCHANGED)
    }
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    #[inline]
    pub fn src_params_updated(self) -> &'a mut W {
        self.variant(NDSUPW::SRC_PARAMS_UPDATED)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NDDUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDUPW {
    #[doc = "Destination parameters remain unchanged."]
    DST_PARAMS_UNCHANGED,
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    DST_PARAMS_UPDATED,
}
impl NDDUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDDUPW::DST_PARAMS_UNCHANGED => false,
            NDDUPW::DST_PARAMS_UPDATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDDUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NDDUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDDUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Destination parameters remain unchanged."]
    #[inline]
    pub fn dst_params_unchanged(self) -> &'a mut W {
        self.variant(NDDUPW::DST_PARAMS_UNCHANGED)
    }
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    #[inline]
    pub fn dst_params_updated(self) -> &'a mut W {
        self.variant(NDDUPW::DST_PARAMS_UPDATED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NDVIEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDVIEWW {
    #[doc = "Next Descriptor View 0"]
    NDV0,
    #[doc = "Next Descriptor View 1"]
    NDV1,
    #[doc = "Next Descriptor View 2"]
    NDV2,
    #[doc = "Next Descriptor View 3"]
    NDV3,
}
impl NDVIEWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NDVIEWW::NDV0 => 0,
            NDVIEWW::NDV1 => 1,
            NDVIEWW::NDV2 => 2,
            NDVIEWW::NDV3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDVIEWW<'a> {
    w: &'a mut W,
}
impl<'a> _NDVIEWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDVIEWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Next Descriptor View 0"]
    #[inline]
    pub fn ndv0(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV0)
    }
    #[doc = "Next Descriptor View 1"]
    #[inline]
    pub fn ndv1(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV1)
    }
    #[doc = "Next Descriptor View 2"]
    #[inline]
    pub fn ndv2(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV2)
    }
    #[doc = "Next Descriptor View 3"]
    #[inline]
    pub fn ndv3(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline]
    pub fn nde(&self) -> NDER {
        NDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline]
    pub fn ndsup(&self) -> NDSUPR {
        NDSUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline]
    pub fn nddup(&self) -> NDDUPR {
        NDDUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline]
    pub fn ndview(&self) -> NDVIEWR {
        NDVIEWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline]
    pub fn nde(&mut self) -> _NDEW {
        _NDEW { w: self }
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline]
    pub fn ndsup(&mut self) -> _NDSUPW {
        _NDSUPW { w: self }
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline]
    pub fn nddup(&mut self) -> _NDDUPW {
        _NDDUPW { w: self }
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline]
    pub fn ndview(&mut self) -> _NDVIEWW {
        _NDVIEWW { w: self }
    }
}
