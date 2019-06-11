#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EEFC_FCR {
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _FCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _FCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Get Flash descriptor"]
    #[inline]
    pub fn getd(self) -> &'a mut W {
        self.variant(FCMDW::GETD)
    }
    #[doc = "Write page"]
    #[inline]
    pub fn wp(self) -> &'a mut W {
        self.variant(FCMDW::WP)
    }
    #[doc = "Write page and lock"]
    #[inline]
    pub fn wpl(self) -> &'a mut W {
        self.variant(FCMDW::WPL)
    }
    #[doc = "Erase page and write page"]
    #[inline]
    pub fn ewp(self) -> &'a mut W {
        self.variant(FCMDW::EWP)
    }
    #[doc = "Erase page and write page then lock"]
    #[inline]
    pub fn ewpl(self) -> &'a mut W {
        self.variant(FCMDW::EWPL)
    }
    #[doc = "Erase all"]
    #[inline]
    pub fn ea(self) -> &'a mut W {
        self.variant(FCMDW::EA)
    }
    #[doc = "Erase pages"]
    #[inline]
    pub fn epa(self) -> &'a mut W {
        self.variant(FCMDW::EPA)
    }
    #[doc = "Set lock bit"]
    #[inline]
    pub fn slb(self) -> &'a mut W {
        self.variant(FCMDW::SLB)
    }
    #[doc = "Clear lock bit"]
    #[inline]
    pub fn clb(self) -> &'a mut W {
        self.variant(FCMDW::CLB)
    }
    #[doc = "Get lock bit"]
    #[inline]
    pub fn glb(self) -> &'a mut W {
        self.variant(FCMDW::GLB)
    }
    #[doc = "Set GPNVM bit"]
    #[inline]
    pub fn sgpb(self) -> &'a mut W {
        self.variant(FCMDW::SGPB)
    }
    #[doc = "Clear GPNVM bit"]
    #[inline]
    pub fn cgpb(self) -> &'a mut W {
        self.variant(FCMDW::CGPB)
    }
    #[doc = "Get GPNVM bit"]
    #[inline]
    pub fn ggpb(self) -> &'a mut W {
        self.variant(FCMDW::GGPB)
    }
    #[doc = "Start read unique identifier"]
    #[inline]
    pub fn stui(self) -> &'a mut W {
        self.variant(FCMDW::STUI)
    }
    #[doc = "Stop read unique identifier"]
    #[inline]
    pub fn spui(self) -> &'a mut W {
        self.variant(FCMDW::SPUI)
    }
    #[doc = "Get CALIB bit"]
    #[inline]
    pub fn gcalb(self) -> &'a mut W {
        self.variant(FCMDW::GCALB)
    }
    #[doc = "Erase sector"]
    #[inline]
    pub fn es(self) -> &'a mut W {
        self.variant(FCMDW::ES)
    }
    #[doc = "Write user signature"]
    #[inline]
    pub fn wus(self) -> &'a mut W {
        self.variant(FCMDW::WUS)
    }
    #[doc = "Erase user signature"]
    #[inline]
    pub fn eus(self) -> &'a mut W {
        self.variant(FCMDW::EUS)
    }
    #[doc = "Start read user signature"]
    #[inline]
    pub fn stus(self) -> &'a mut W {
        self.variant(FCMDW::STUS)
    }
    #[doc = "Stop read user signature"]
    #[inline]
    pub fn spus(self) -> &'a mut W {
        self.variant(FCMDW::SPUS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FARGW<'a> {
    w: &'a mut W,
}
impl<'a> _FARGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FKEYW::PASSWD => 90,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _FKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The 0x5A value enables the command defined by the bits of the register. If the field is written with a different value, the write is not performed and no action is started."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(FKEYW::PASSWD)
    }
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
    #[doc = "Bits 0:7 - Flash Command"]
    #[inline]
    pub fn fcmd(&mut self) -> _FCMDW {
        _FCMDW { w: self }
    }
    #[doc = "Bits 8:23 - Flash Command Argument"]
    #[inline]
    pub fn farg(&mut self) -> _FARGW {
        _FARGW { w: self }
    }
    #[doc = "Bits 24:31 - Flash Writing Protection Key"]
    #[inline]
    pub fn fkey(&mut self) -> _FKEYW {
        _FKEYW { w: self }
    }
}
