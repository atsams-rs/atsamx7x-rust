#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock Bits Word 0"]
    pub lockbit_word0: LOCKBIT_WORD0,
    #[doc = "0x04 - Lock Bits Word 1"]
    pub lockbit_word1: LOCKBIT_WORD1,
    #[doc = "0x08 - Lock Bits Word 2"]
    pub lockbit_word2: LOCKBIT_WORD2,
    #[doc = "0x0c - Lock Bits Word 3"]
    pub lockbit_word3: LOCKBIT_WORD3,
}
#[doc = "Lock Bits Word 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word0](lockbit_word0) module"]
pub type LOCKBIT_WORD0 = crate::Reg<u32, _LOCKBIT_WORD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKBIT_WORD0;
#[doc = "`read()` method returns [lockbit_word0::R](lockbit_word0::R) reader structure"]
impl crate::Readable for LOCKBIT_WORD0 {}
#[doc = "`write(|w| ..)` method takes [lockbit_word0::W](lockbit_word0::W) writer structure"]
impl crate::Writable for LOCKBIT_WORD0 {}
#[doc = "Lock Bits Word 0"]
pub mod lockbit_word0;
#[doc = "Lock Bits Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word1](lockbit_word1) module"]
pub type LOCKBIT_WORD1 = crate::Reg<u32, _LOCKBIT_WORD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKBIT_WORD1;
#[doc = "`read()` method returns [lockbit_word1::R](lockbit_word1::R) reader structure"]
impl crate::Readable for LOCKBIT_WORD1 {}
#[doc = "`write(|w| ..)` method takes [lockbit_word1::W](lockbit_word1::W) writer structure"]
impl crate::Writable for LOCKBIT_WORD1 {}
#[doc = "Lock Bits Word 1"]
pub mod lockbit_word1;
#[doc = "Lock Bits Word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word2](lockbit_word2) module"]
pub type LOCKBIT_WORD2 = crate::Reg<u32, _LOCKBIT_WORD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKBIT_WORD2;
#[doc = "`read()` method returns [lockbit_word2::R](lockbit_word2::R) reader structure"]
impl crate::Readable for LOCKBIT_WORD2 {}
#[doc = "`write(|w| ..)` method takes [lockbit_word2::W](lockbit_word2::W) writer structure"]
impl crate::Writable for LOCKBIT_WORD2 {}
#[doc = "Lock Bits Word 2"]
pub mod lockbit_word2;
#[doc = "Lock Bits Word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit_word3](lockbit_word3) module"]
pub type LOCKBIT_WORD3 = crate::Reg<u32, _LOCKBIT_WORD3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCKBIT_WORD3;
#[doc = "`read()` method returns [lockbit_word3::R](lockbit_word3::R) reader structure"]
impl crate::Readable for LOCKBIT_WORD3 {}
#[doc = "`write(|w| ..)` method takes [lockbit_word3::W](lockbit_word3::W) writer structure"]
impl crate::Writable for LOCKBIT_WORD3 {}
#[doc = "Lock Bits Word 3"]
pub mod lockbit_word3;
