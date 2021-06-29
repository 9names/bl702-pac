#[doc = "Register `swrst_cfg3` reader"]
pub struct R(crate::R<SWRST_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRST_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRST_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "swrst_cfg3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg3](index.html) module"]
pub struct SWRST_CFG3_SPEC;
impl crate::RegisterSpec for SWRST_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg3::R](R) reader structure"]
impl crate::Readable for SWRST_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets swrst_cfg3 to value 0"]
impl crate::Resettable for SWRST_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
