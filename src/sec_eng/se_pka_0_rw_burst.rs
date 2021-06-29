#[doc = "Register `se_pka_0_rw_burst` reader"]
pub struct R(crate::R<SE_PKA_0_RW_BURST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_PKA_0_RW_BURST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_PKA_0_RW_BURST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_PKA_0_RW_BURST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "se_pka_0_rw_burst.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_rw_burst](index.html) module"]
pub struct SE_PKA_0_RW_BURST_SPEC;
impl crate::RegisterSpec for SE_PKA_0_RW_BURST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_pka_0_rw_burst::R](R) reader structure"]
impl crate::Readable for SE_PKA_0_RW_BURST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets se_pka_0_rw_burst to value 0"]
impl crate::Resettable for SE_PKA_0_RW_BURST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
