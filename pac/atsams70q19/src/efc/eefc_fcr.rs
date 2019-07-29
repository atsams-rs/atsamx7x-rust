#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEFC_FCR {
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
#[doc = "Values that can be written to the field `FCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCMDW {
    #[doc = "Get Flash descriptor"]
    GETD,
    #[doc = "Write page"]
    WP,
    #[doc = "Write page and lock"]
    WPL,
    #[doc = "Erase page and write page"]
    EWP,
    #[doc = "Erase page and write page then lock"]
    EWPL,
    #[doc = "Erase all"]
    EA,
    #[doc = "Erase pages"]
    EPA,
    #[doc = "Set lock bit"]
    SLB,
    #[doc = "Clear lock bit"]
    CLB,
    #[doc = "Get lock bit"]
    GLB,
    #[doc = "Set GPNVM bit"]
    SGPB,
    #[doc = "Clear GPNVM bit"]
    CGPB,
    #[doc = "Get GPNVM bit"]
    GGPB,
    #[doc = "Start read unique identifier"]
    STUI,
    #[doc = "Stop read unique identifier"]
    SPUI,
    #[doc = "Get CALIB bit"]
    GCALB,
    #[doc = "Erase sector"]
    ES,
    #[doc = "Write user signature"]
    WUS,
    #[doc = "Erase user signature"]
    EUS,
    #[doc = "Start read user signature"]
    STUS,
    #[doc = "Stop read user signature"]
    SPUS,
}
impl FCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FCMDW::GETD => 0,
            FCMDW::WP => 1,
            FCMDW::WPL => 2,
            FCMDW::EWP => 3,
            FCMDW::EWPL => 4,
            FCMDW::EA => 5,
            FCMDW::EPA => 7,
            FCMDW::SLB => 8,
            FCMDW::CLB => 9,
            FCMDW::GLB => 10,
            FCMDW::SGPB => 11,
            FCMDW::CGPB => 12,
            FCMDW::GGPB => 13,
            FCMDW::STUI => 14,
            FCMDW::SPUI => 15,
            FCMDW::GCALB => 16,
            FCMDW::ES => 17,
            FCMDW::WUS => 18,
            FCMDW::EUS => 19,
            FCMDW::STUS => 20,
            FCMDW::SPUS => 21,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _FCMDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Get Flash descriptor"]
    #[inline(always)]
    pub fn getd(self) -> &'a mut W {
        self.variant(FCMDW::GETD)
    }
    #[doc = "Write page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(FCMDW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline(always)]
    pub fn wpl(self) -> &'a mut W {
        self.variant(FCMDW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline(always)]
    pub fn ewp(self) -> &'a mut W {
        self.variant(FCMDW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline(always)]
    pub fn ewpl(self) -> &'a mut W {
        self.variant(FCMDW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline(always)]
    pub fn ea(self) -> &'a mut W {
        self.variant(FCMDW::EA)
    }
    #[doc = "Erase pages"]
    #[inline(always)]
    pub fn epa(self) -> &'a mut W {
        self.variant(FCMDW::EPA)
    }
    #[doc = "Set lock bit"]
    #[inline(always)]
    pub fn slb(self) -> &'a mut W {
        self.variant(FCMDW::SLB)
    }
    #[doc = "Clear lock bit"]
    #[inline(always)]
    pub fn clb(self) -> &'a mut W {
        self.variant(FCMDW::CLB)
    }
    #[doc = "Get lock bit"]
    #[inline(always)]
    pub fn glb(self) -> &'a mut W {
        self.variant(FCMDW::GLB)
    }
    #[doc = "Set GPNVM bit"]
    #[inline(always)]
    pub fn sgpb(self) -> &'a mut W {
        self.variant(FCMDW::SGPB)
    }
    #[doc = "Clear GPNVM bit"]
    #[inline(always)]
    pub fn cgpb(self) -> &'a mut W {
        self.variant(FCMDW::CGPB)
    }
    #[doc = "Get GPNVM bit"]
    #[inline(always)]
    pub fn ggpb(self) -> &'a mut W {
        self.variant(FCMDW::GGPB)
    }
    #[doc = "Start read unique identifier"]
    #[inline(always)]
    pub fn stui(self) -> &'a mut W {
        self.variant(FCMDW::STUI)
    }
    #[doc = "Stop read unique identifier"]
    #[inline(always)]
    pub fn spui(self) -> &'a mut W {
        self.variant(FCMDW::SPUI)
    }
    #[doc = "Get CALIB bit"]
    #[inline(always)]
    pub fn gcalb(self) -> &'a mut W {
        self.variant(FCMDW::GCALB)
    }
    #[doc = "Erase sector"]
    #[inline(always)]
    pub fn es(self) -> &'a mut W {
        self.variant(FCMDW::ES)
    }
    #[doc = "Write user signature"]
    #[inline(always)]
    pub fn wus(self) -> &'a mut W {
        self.variant(FCMDW::WUS)
    }
    #[doc = "Erase user signature"]
    #[inline(always)]
    pub fn eus(self) -> &'a mut W {
        self.variant(FCMDW::EUS)
    }
    #[doc = "Start read user signature"]
    #[inline(always)]
    pub fn stus(self) -> &'a mut W {
        self.variant(FCMDW::STUS)
    }
    #[doc = "Stop read user signature"]
    #[inline(always)]
    pub fn spus(self) -> &'a mut W {
        self.variant(FCMDW::SPUS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _FARGW<'a> {
    w: &'a mut W,
}
impl<'a> _FARGW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Values that can be written to the field `FKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FKEYW {
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    PASSWD,
}
impl FKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FKEYW::PASSWD => 90,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _FKEYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(FKEYW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline(always)]
    pub fn fcmd(&mut self) -> _FCMDW {
        _FCMDW { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline(always)]
    pub fn farg(&mut self) -> _FARGW {
        _FARGW { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline(always)]
    pub fn fkey(&mut self) -> _FKEYW {
        _FKEYW { w: self }
    }
}
