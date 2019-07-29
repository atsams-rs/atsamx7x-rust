#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_CC {
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
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    MEM_TRAN,
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    PER_TRAN,
}
impl crate::ToBits<bool> for TYPER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TYPER::MEM_TRAN => false,
            TYPER::PER_TRAN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TYPE_R = crate::FR<bool, TYPER>;
impl TYPE_R {
    #[doc = "Checks if the value of the field is `MEM_TRAN`"]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        *self == TYPER::MEM_TRAN
    }
    #[doc = "Checks if the value of the field is `PER_TRAN`"]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        *self == TYPER::PER_TRAN
    }
}
#[doc = "Values that can be written to the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPEW {
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    MEM_TRAN,
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    PER_TRAN,
}
impl TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TYPEW::MEM_TRAN => false,
            TYPEW::PER_TRAN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut W {
        self.variant(TYPEW::MEM_TRAN)
    }
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut W {
        self.variant(TYPEW::PER_TRAN)
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
#[doc = "Possible values of the field `MBSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBSIZER {
    #[doc = "The memory burst size is set to one."]
    SINGLE,
    #[doc = "The memory burst size is set to four."]
    FOUR,
    #[doc = "The memory burst size is set to eight."]
    EIGHT,
    #[doc = "The memory burst size is set to sixteen."]
    SIXTEEN,
}
impl crate::ToBits<u8> for MBSIZER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MBSIZER::SINGLE => 0,
            MBSIZER::FOUR => 1,
            MBSIZER::EIGHT => 2,
            MBSIZER::SIXTEEN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MBSIZE_R = crate::FR<u8, MBSIZER>;
impl MBSIZE_R {
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MBSIZER::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == MBSIZER::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == MBSIZER::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == MBSIZER::SIXTEEN
    }
}
#[doc = "Values that can be written to the field `MBSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBSIZEW {
    #[doc = "The memory burst size is set to one."]
    SINGLE,
    #[doc = "The memory burst size is set to four."]
    FOUR,
    #[doc = "The memory burst size is set to eight."]
    EIGHT,
    #[doc = "The memory burst size is set to sixteen."]
    SIXTEEN,
}
impl MBSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MBSIZEW::SINGLE => 0,
            MBSIZEW::FOUR => 1,
            MBSIZEW::EIGHT => 2,
            MBSIZEW::SIXTEEN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MBSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _MBSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBSIZEW::SINGLE)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(MBSIZEW::FOUR)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(MBSIZEW::EIGHT)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(MBSIZEW::SIXTEEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `DSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNCR {
    #[doc = "Peripheral to Memory transfer."]
    PER2MEM,
    #[doc = "Memory to Peripheral transfer."]
    MEM2PER,
}
impl crate::ToBits<bool> for DSYNCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DSYNCR::PER2MEM => false,
            DSYNCR::MEM2PER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DSYNC_R = crate::FR<bool, DSYNCR>;
impl DSYNC_R {
    #[doc = "Checks if the value of the field is `PER2MEM`"]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        *self == DSYNCR::PER2MEM
    }
    #[doc = "Checks if the value of the field is `MEM2PER`"]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        *self == DSYNCR::MEM2PER
    }
}
#[doc = "Values that can be written to the field `DSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNCW {
    #[doc = "Peripheral to Memory transfer."]
    PER2MEM,
    #[doc = "Memory to Peripheral transfer."]
    MEM2PER,
}
impl DSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DSYNCW::PER2MEM => false,
            DSYNCW::MEM2PER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSYNCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral to Memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut W {
        self.variant(DSYNCW::PER2MEM)
    }
    #[doc = "Memory to Peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut W {
        self.variant(DSYNCW::MEM2PER)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SWREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQR {
    #[doc = "Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED,
    #[doc = "Software request is connected to the peripheral request line."]
    SWR_CONNECTED,
}
impl crate::ToBits<bool> for SWREQR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SWREQR::HWR_CONNECTED => false,
            SWREQR::SWR_CONNECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SWREQ_R = crate::FR<bool, SWREQR>;
impl SWREQ_R {
    #[doc = "Checks if the value of the field is `HWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        *self == SWREQR::HWR_CONNECTED
    }
    #[doc = "Checks if the value of the field is `SWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        *self == SWREQR::SWR_CONNECTED
    }
}
#[doc = "Values that can be written to the field `SWREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQW {
    #[doc = "Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED,
    #[doc = "Software request is connected to the peripheral request line."]
    SWR_CONNECTED,
}
impl SWREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SWREQW::HWR_CONNECTED => false,
            SWREQW::SWR_CONNECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SWREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut W {
        self.variant(SWREQW::HWR_CONNECTED)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut W {
        self.variant(SWREQW::SWR_CONNECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `MEMSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSETR {
    #[doc = "Memset is not activated."]
    NORMAL_MODE,
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE,
}
impl crate::ToBits<bool> for MEMSETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MEMSETR::NORMAL_MODE => false,
            MEMSETR::HW_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MEMSET_R = crate::FR<bool, MEMSETR>;
impl MEMSET_R {
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == MEMSETR::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `HW_MODE`"]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        *self == MEMSETR::HW_MODE
    }
}
#[doc = "Values that can be written to the field `MEMSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSETW {
    #[doc = "Memset is not activated."]
    NORMAL_MODE,
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE,
}
impl MEMSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMSETW::NORMAL_MODE => false,
            MEMSETW::HW_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MEMSETW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMSETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(MEMSETW::NORMAL_MODE)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut W {
        self.variant(MEMSETW::HW_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `CSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZER {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "2 data transferred"]
    CHK_2,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
}
impl crate::ToBits<u8> for CSIZER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSIZER::CHK_1 => 0,
            CSIZER::CHK_2 => 1,
            CSIZER::CHK_4 => 2,
            CSIZER::CHK_8 => 3,
            CSIZER::CHK_16 => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSIZE_R = crate::FR<u8, CSIZER>;
impl CSIZE_R {
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == CSIZER::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_2`"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        *self == CSIZER::CHK_2
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == CSIZER::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == CSIZER::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == CSIZER::CHK_16
    }
}
#[doc = "Values that can be written to the field `CSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZEW {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "2 data transferred"]
    CHK_2,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
}
impl CSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSIZEW::CHK_1 => 0,
            CSIZEW::CHK_2 => 1,
            CSIZEW::CHK_4 => 2,
            CSIZEW::CHK_8 => 3,
            CSIZEW::CHK_16 => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `DWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWIDTHR {
    #[doc = "The data size is set to 8 bits"]
    BYTE,
    #[doc = "The data size is set to 16 bits"]
    HALFWORD,
    #[doc = "The data size is set to 32 bits"]
    WORD,
}
impl crate::ToBits<u8> for DWIDTHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DWIDTHR::BYTE => 0,
            DWIDTHR::HALFWORD => 1,
            DWIDTHR::WORD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DWIDTH_R = crate::FR<u8, DWIDTHR>;
impl DWIDTH_R {
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DWIDTHR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DWIDTHR::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DWIDTHR::WORD
    }
}
#[doc = "Values that can be written to the field `DWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWIDTHW {
    #[doc = "The data size is set to 8 bits"]
    BYTE,
    #[doc = "The data size is set to 16 bits"]
    HALFWORD,
    #[doc = "The data size is set to 32 bits"]
    WORD,
}
impl DWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DWIDTHW::BYTE => 0,
            DWIDTHW::HALFWORD => 1,
            DWIDTHW::WORD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DWIDTHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DWIDTHW::BYTE)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DWIDTHW::HALFWORD)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DWIDTHW::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `SIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIFR {
    #[doc = "The data is read through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is read through the system bus interface 1."]
    AHB_IF1,
}
impl crate::ToBits<bool> for SIFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SIFR::AHB_IF0 => false,
            SIFR::AHB_IF1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SIF_R = crate::FR<bool, SIFR>;
impl SIF_R {
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == SIFR::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == SIFR::AHB_IF1
    }
}
#[doc = "Values that can be written to the field `SIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIFW {
    #[doc = "The data is read through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is read through the system bus interface 1."]
    AHB_IF1,
}
impl SIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SIFW::AHB_IF0 => false,
            SIFW::AHB_IF1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SIFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(SIFW::AHB_IF0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(SIFW::AHB_IF1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `DIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFR {
    #[doc = "The data is written through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is written though the system bus interface 1."]
    AHB_IF1,
}
impl crate::ToBits<bool> for DIFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIFR::AHB_IF0 => false,
            DIFR::AHB_IF1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIF_R = crate::FR<bool, DIFR>;
impl DIF_R {
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DIFR::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DIFR::AHB_IF1
    }
}
#[doc = "Values that can be written to the field `DIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFW {
    #[doc = "The data is written through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is written though the system bus interface 1."]
    AHB_IF1,
}
impl DIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFW::AHB_IF0 => false,
            DIFW::AHB_IF1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DIFW::AHB_IF0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DIFW::AHB_IF1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMR {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl crate::ToBits<u8> for SAMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SAMR::FIXED_AM => 0,
            SAMR::INCREMENTED_AM => 1,
            SAMR::UBS_AM => 2,
            SAMR::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SAM_R = crate::FR<u8, SAMR>;
impl SAM_R {
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == SAMR::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == SAMR::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == SAMR::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == SAMR::UBS_DS_AM
    }
}
#[doc = "Values that can be written to the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMW {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl SAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMW::FIXED_AM => 0,
            SAMW::INCREMENTED_AM => 1,
            SAMW::UBS_AM => 2,
            SAMW::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(SAMW::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(SAMW::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(SAMW::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(SAMW::UBS_DS_AM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `DAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAMR {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl crate::ToBits<u8> for DAMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DAMR::FIXED_AM => 0,
            DAMR::INCREMENTED_AM => 1,
            DAMR::UBS_AM => 2,
            DAMR::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DAM_R = crate::FR<u8, DAMR>;
impl DAM_R {
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == DAMR::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == DAMR::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == DAMR::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == DAMR::UBS_DS_AM
    }
}
#[doc = "Values that can be written to the field `DAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAMW {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl DAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAMW::FIXED_AM => 0,
            DAMW::INCREMENTED_AM => 1,
            DAMW::UBS_AM => 2,
            DAMW::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DAMW<'a> {
    w: &'a mut W,
}
impl<'a> _DAMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(DAMW::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(DAMW::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(DAMW::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(DAMW::UBS_DS_AM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `INITD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDR {
    #[doc = "Channel initialization is in progress."]
    IN_PROGRESS,
    #[doc = "Channel initialization is completed."]
    TERMINATED,
}
impl crate::ToBits<bool> for INITDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INITDR::IN_PROGRESS => false,
            INITDR::TERMINATED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INITD_R = crate::FR<bool, INITDR>;
impl INITD_R {
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == INITDR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `TERMINATED`"]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        *self == INITDR::TERMINATED
    }
}
#[doc = "Values that can be written to the field `INITD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDW {
    #[doc = "Channel initialization is in progress."]
    IN_PROGRESS,
    #[doc = "Channel initialization is completed."]
    TERMINATED,
}
impl INITDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INITDW::IN_PROGRESS => false,
            INITDW::TERMINATED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INITDW<'a> {
    w: &'a mut W,
}
impl<'a> _INITDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(INITDW::IN_PROGRESS)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut W {
        self.variant(INITDW::TERMINATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `RDIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIPR {
    #[doc = "No Active read transaction on the bus."]
    DONE,
    #[doc = "A read transaction is in progress."]
    IN_PROGRESS,
}
impl crate::ToBits<bool> for RDIPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RDIPR::DONE => false,
            RDIPR::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RDIP_R = crate::FR<bool, RDIPR>;
impl RDIP_R {
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == RDIPR::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RDIPR::IN_PROGRESS
    }
}
#[doc = "Values that can be written to the field `RDIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIPW {
    #[doc = "No Active read transaction on the bus."]
    DONE,
    #[doc = "A read transaction is in progress."]
    IN_PROGRESS,
}
impl RDIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RDIPW::DONE => false,
            RDIPW::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RDIPW<'a> {
    w: &'a mut W,
}
impl<'a> _RDIPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(RDIPW::DONE)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(RDIPW::IN_PROGRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `WRIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIPR {
    #[doc = "No Active write transaction on the bus."]
    DONE,
    #[doc = "A Write transaction is in progress."]
    IN_PROGRESS,
}
impl crate::ToBits<bool> for WRIPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WRIPR::DONE => false,
            WRIPR::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WRIP_R = crate::FR<bool, WRIPR>;
impl WRIP_R {
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == WRIPR::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == WRIPR::IN_PROGRESS
    }
}
#[doc = "Values that can be written to the field `WRIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIPW {
    #[doc = "No Active write transaction on the bus."]
    DONE,
    #[doc = "A Write transaction is in progress."]
    IN_PROGRESS,
}
impl WRIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WRIPW::DONE => false,
            WRIPW::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WRIPW<'a> {
    w: &'a mut W,
}
impl<'a> _WRIPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(WRIPW::DONE)
    }
    #[doc = "A Write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(WRIPW::IN_PROGRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PERID_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PERIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MBSIZE_R {
        MBSIZE_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DSYNC_R {
        DSYNC_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits() >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SIF_R {
        SIF_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DAM_R {
        DAM_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RDIP_R {
        RDIP_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WRIP_R {
        WRIP_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new(((self.bits() >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&mut self) -> _TYPEW {
        _TYPEW { w: self }
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&mut self) -> _MBSIZEW {
        _MBSIZEW { w: self }
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&mut self) -> _DSYNCW {
        _DSYNCW { w: self }
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&mut self) -> _SWREQW {
        _SWREQW { w: self }
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&mut self) -> _MEMSETW {
        _MEMSETW { w: self }
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&mut self) -> _CSIZEW {
        _CSIZEW { w: self }
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> _DWIDTHW {
        _DWIDTHW { w: self }
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&mut self) -> _SIFW {
        _SIFW { w: self }
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&mut self) -> _DIFW {
        _DIFW { w: self }
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&mut self) -> _SAMW {
        _SAMW { w: self }
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&mut self) -> _DAMW {
        _DAMW { w: self }
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&mut self) -> _INITDW {
        _INITDW { w: self }
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&mut self) -> _RDIPW {
        _RDIPW { w: self }
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&mut self) -> _WRIPW {
        _WRIPW { w: self }
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> _PERIDW {
        _PERIDW { w: self }
    }
}
