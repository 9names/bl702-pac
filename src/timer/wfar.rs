#[doc = "Register `WFAR` reader"]
pub struct R(crate::R<WFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WFAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WFAR` writer"]
pub struct W(crate::W<WFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WFAR_SPEC>;
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
impl From<crate::W<WFAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WFAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wfar` reader - "]
pub type WFAR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wfar` writer - "]
pub type WFAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WFAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&self) -> WFAR_R {
        WFAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&mut self) -> WFAR_W<0> {
        WFAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WFAR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wfar](index.html) module"]
pub struct WFAR_SPEC;
impl crate::RegisterSpec for WFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wfar::R](R) reader structure"]
impl crate::Readable for WFAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wfar::W](W) writer structure"]
impl crate::Writable for WFAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WFAR to value 0"]
impl crate::Resettable for WFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
