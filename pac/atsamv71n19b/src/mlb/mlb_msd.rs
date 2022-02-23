#[doc = "Register `MLB_MSD` reader"]
pub struct R(crate::R<MLB_MSD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_MSD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_MSD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_MSD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SD0` reader - System Data (Byte 0)"]
pub struct SD0_R(crate::FieldReader<u8, u8>);
impl SD0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD1` reader - System Data (Byte 1)"]
pub struct SD1_R(crate::FieldReader<u8, u8>);
impl SD1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD2` reader - System Data (Byte 2)"]
pub struct SD2_R(crate::FieldReader<u8, u8>);
impl SD2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SD2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SD3` reader - System Data (Byte 3)"]
pub struct SD3_R(crate::FieldReader<u8, u8>);
impl SD3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SD3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - System Data (Byte 0)"]
    #[inline(always)]
    pub fn sd0(&self) -> SD0_R {
        SD0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - System Data (Byte 1)"]
    #[inline(always)]
    pub fn sd1(&self) -> SD1_R {
        SD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - System Data (Byte 2)"]
    #[inline(always)]
    pub fn sd2(&self) -> SD2_R {
        SD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - System Data (Byte 3)"]
    #[inline(always)]
    pub fn sd3(&self) -> SD3_R {
        SD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "MediaLB System Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_msd](index.html) module"]
pub struct MLB_MSD_SPEC;
impl crate::RegisterSpec for MLB_MSD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_msd::R](R) reader structure"]
impl crate::Readable for MLB_MSD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MLB_MSD to value 0"]
impl crate::Resettable for MLB_MSD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
