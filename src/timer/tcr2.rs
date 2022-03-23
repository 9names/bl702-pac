#[doc = "Register `TCR2` reader"]
pub struct R(crate::R<TCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR2` writer"]
pub struct W(crate::W<TCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR2_SPEC>;
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
impl From<crate::W<TCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcr` reader - "]
pub struct TCR_R(crate::FieldReader<u32, u32>);
impl TCR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tcr` writer - "]
pub struct TCR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&mut self) -> TCR_W {
        TCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr2](index.html) module"]
pub struct TCR2_SPEC;
impl crate::RegisterSpec for TCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr2::R](R) reader structure"]
impl crate::Readable for TCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr2::W](W) writer structure"]
impl crate::Writable for TCR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for TCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
