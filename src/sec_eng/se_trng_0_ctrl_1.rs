#[doc = "Register `se_trng_0_ctrl_1` reader"]
pub struct R(crate::R<SE_TRNG_0_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_ctrl_1` writer"]
pub struct W(crate::W<SE_TRNG_0_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SE_TRNG_0_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_TRNG_0_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_reseed_n_lsb` reader - "]
pub type SE_TRNG_0_RESEED_N_LSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `se_trng_0_reseed_n_lsb` writer - "]
pub type SE_TRNG_0_RESEED_N_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_TRNG_0_CTRL_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_lsb(&self) -> SE_TRNG_0_RESEED_N_LSB_R {
        SE_TRNG_0_RESEED_N_LSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_reseed_n_lsb(&mut self) -> SE_TRNG_0_RESEED_N_LSB_W<0> {
        SE_TRNG_0_RESEED_N_LSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_1](index.html) module"]
pub struct SE_TRNG_0_CTRL_1_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_ctrl_1::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_1::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_1 to value 0"]
impl crate::Resettable for SE_TRNG_0_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
