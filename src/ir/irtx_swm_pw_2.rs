#[doc = "Register `irtx_swm_pw_2` reader"]
pub struct R(crate::R<IRTX_SWM_PW_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_SWM_PW_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRTX_SWM_PW_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRTX_SWM_PW_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_swm_pw_2` writer"]
pub struct W(crate::W<IRTX_SWM_PW_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_SWM_PW_2_SPEC>;
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
impl From<crate::W<IRTX_SWM_PW_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRTX_SWM_PW_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_swm_pw_2` reader - "]
pub type CR_IRTX_SWM_PW_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `cr_irtx_swm_pw_2` writer - "]
pub type CR_IRTX_SWM_PW_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRTX_SWM_PW_2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_2(&self) -> CR_IRTX_SWM_PW_2_R {
        CR_IRTX_SWM_PW_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_2(&mut self) -> CR_IRTX_SWM_PW_2_W<0> {
        CR_IRTX_SWM_PW_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_swm_pw_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_2](index.html) module"]
pub struct IRTX_SWM_PW_2_SPEC;
impl crate::RegisterSpec for IRTX_SWM_PW_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_swm_pw_2::R](R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_2::W](W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irtx_swm_pw_2 to value 0"]
impl crate::Resettable for IRTX_SWM_PW_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
