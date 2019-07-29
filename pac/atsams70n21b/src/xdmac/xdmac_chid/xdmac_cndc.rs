#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_CNDC {
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
#[doc = "Possible values of the field `NDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDER {
    #[doc = "Descriptor fetch is disabled."]
    DSCR_FETCH_DIS,
    #[doc = "Descriptor fetch is enabled."]
    DSCR_FETCH_EN,
}
impl crate::ToBits<bool> for NDER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NDER::DSCR_FETCH_DIS => false,
            NDER::DSCR_FETCH_EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NDE_R = crate::FR<bool, NDER>;
impl NDE_R {
    #[doc = "Checks if the value of the field is `DSCR_FETCH_DIS`"]
    #[inline(always)]
    pub fn is_dscr_fetch_dis(&self) -> bool {
        *self == NDER::DSCR_FETCH_DIS
    }
    #[doc = "Checks if the value of the field is `DSCR_FETCH_EN`"]
    #[inline(always)]
    pub fn is_dscr_fetch_en(&self) -> bool {
        *self == NDER::DSCR_FETCH_EN
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NDEW::DSCR_FETCH_DIS => false,
            NDEW::DSCR_FETCH_EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NDEW<'a> {
    w: &'a mut W,
}
impl<'a> _NDEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Descriptor fetch is disabled."]
    #[inline(always)]
    pub fn dscr_fetch_dis(self) -> &'a mut W {
        self.variant(NDEW::DSCR_FETCH_DIS)
    }
    #[doc = "Descriptor fetch is enabled."]
    #[inline(always)]
    pub fn dscr_fetch_en(self) -> &'a mut W {
        self.variant(NDEW::DSCR_FETCH_EN)
    }
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
#[doc = "Possible values of the field `NDSUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDSUPR {
    #[doc = "Source parameters remain unchanged."]
    SRC_PARAMS_UNCHANGED,
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    SRC_PARAMS_UPDATED,
}
impl crate::ToBits<bool> for NDSUPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NDSUPR::SRC_PARAMS_UNCHANGED => false,
            NDSUPR::SRC_PARAMS_UPDATED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NDSUP_R = crate::FR<bool, NDSUPR>;
impl NDSUP_R {
    #[doc = "Checks if the value of the field is `SRC_PARAMS_UNCHANGED`"]
    #[inline(always)]
    pub fn is_src_params_unchanged(&self) -> bool {
        *self == NDSUPR::SRC_PARAMS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `SRC_PARAMS_UPDATED`"]
    #[inline(always)]
    pub fn is_src_params_updated(&self) -> bool {
        *self == NDSUPR::SRC_PARAMS_UPDATED
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NDSUPW::SRC_PARAMS_UNCHANGED => false,
            NDSUPW::SRC_PARAMS_UPDATED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NDSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NDSUPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDSUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Source parameters remain unchanged."]
    #[inline(always)]
    pub fn src_params_unchanged(self) -> &'a mut W {
        self.variant(NDSUPW::SRC_PARAMS_UNCHANGED)
    }
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn src_params_updated(self) -> &'a mut W {
        self.variant(NDSUPW::SRC_PARAMS_UPDATED)
    }
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
#[doc = "Possible values of the field `NDDUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDUPR {
    #[doc = "Destination parameters remain unchanged."]
    DST_PARAMS_UNCHANGED,
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    DST_PARAMS_UPDATED,
}
impl crate::ToBits<bool> for NDDUPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NDDUPR::DST_PARAMS_UNCHANGED => false,
            NDDUPR::DST_PARAMS_UPDATED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NDDUP_R = crate::FR<bool, NDDUPR>;
impl NDDUP_R {
    #[doc = "Checks if the value of the field is `DST_PARAMS_UNCHANGED`"]
    #[inline(always)]
    pub fn is_dst_params_unchanged(&self) -> bool {
        *self == NDDUPR::DST_PARAMS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `DST_PARAMS_UPDATED`"]
    #[inline(always)]
    pub fn is_dst_params_updated(&self) -> bool {
        *self == NDDUPR::DST_PARAMS_UPDATED
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NDDUPW::DST_PARAMS_UNCHANGED => false,
            NDDUPW::DST_PARAMS_UPDATED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NDDUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NDDUPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDDUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Destination parameters remain unchanged."]
    #[inline(always)]
    pub fn dst_params_unchanged(self) -> &'a mut W {
        self.variant(NDDUPW::DST_PARAMS_UNCHANGED)
    }
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn dst_params_updated(self) -> &'a mut W {
        self.variant(NDDUPW::DST_PARAMS_UPDATED)
    }
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
impl crate::ToBits<u8> for NDVIEWR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NDVIEWR::NDV0 => 0,
            NDVIEWR::NDV1 => 1,
            NDVIEWR::NDV2 => 2,
            NDVIEWR::NDV3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NDVIEW_R = crate::FR<u8, NDVIEWR>;
impl NDVIEW_R {
    #[doc = "Checks if the value of the field is `NDV0`"]
    #[inline(always)]
    pub fn is_ndv0(&self) -> bool {
        *self == NDVIEWR::NDV0
    }
    #[doc = "Checks if the value of the field is `NDV1`"]
    #[inline(always)]
    pub fn is_ndv1(&self) -> bool {
        *self == NDVIEWR::NDV1
    }
    #[doc = "Checks if the value of the field is `NDV2`"]
    #[inline(always)]
    pub fn is_ndv2(&self) -> bool {
        *self == NDVIEWR::NDV2
    }
    #[doc = "Checks if the value of the field is `NDV3`"]
    #[inline(always)]
    pub fn is_ndv3(&self) -> bool {
        *self == NDVIEWR::NDV3
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            NDVIEWW::NDV0 => 0,
            NDVIEWW::NDV1 => 1,
            NDVIEWW::NDV2 => 2,
            NDVIEWW::NDV3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NDVIEWW<'a> {
    w: &'a mut W,
}
impl<'a> _NDVIEWW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NDVIEWW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Next Descriptor View 0"]
    #[inline(always)]
    pub fn ndv0(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV0)
    }
    #[doc = "Next Descriptor View 1"]
    #[inline(always)]
    pub fn ndv1(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV1)
    }
    #[doc = "Next Descriptor View 2"]
    #[inline(always)]
    pub fn ndv2(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV2)
    }
    #[doc = "Next Descriptor View 3"]
    #[inline(always)]
    pub fn ndv3(self) -> &'a mut W {
        self.variant(NDVIEWW::NDV3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline(always)]
    pub fn nde(&self) -> NDE_R {
        NDE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline(always)]
    pub fn ndsup(&self) -> NDSUP_R {
        NDSUP_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline(always)]
    pub fn nddup(&self) -> NDDUP_R {
        NDDUP_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline(always)]
    pub fn ndview(&self) -> NDVIEW_R {
        NDVIEW_R::new(((self.bits() >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline(always)]
    pub fn nde(&mut self) -> _NDEW {
        _NDEW { w: self }
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline(always)]
    pub fn ndsup(&mut self) -> _NDSUPW {
        _NDSUPW { w: self }
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline(always)]
    pub fn nddup(&mut self) -> _NDDUPW {
        _NDDUPW { w: self }
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline(always)]
    pub fn ndview(&mut self) -> _NDVIEWW {
        _NDVIEWW { w: self }
    }
}
