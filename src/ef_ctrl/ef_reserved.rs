#[doc = "Register `ef_reserved` reader"]
pub struct R(crate::R<EF_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_reserved` writer"]
pub struct W(crate::W<EF_RESERVED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_RESERVED_SPEC>;
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
impl From<crate::W<EF_RESERVED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_RESERVED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_reserved` reader - "]
pub type EF_RESERVED_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ef_reserved` writer - "]
pub type EF_RESERVED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_RESERVED_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_reserved(&self) -> EF_RESERVED_R {
        EF_RESERVED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_reserved(&mut self) -> EF_RESERVED_W<0> {
        EF_RESERVED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_reserved](index.html) module"]
pub struct EF_RESERVED_SPEC;
impl crate::RegisterSpec for EF_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_reserved::R](R) reader structure"]
impl crate::Readable for EF_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_reserved::W](W) writer structure"]
impl crate::Writable for EF_RESERVED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_reserved to value 0"]
impl crate::Resettable for EF_RESERVED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
