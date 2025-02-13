#[doc = "Register `DMA_SoftLBReq` reader"]
pub struct R(crate::R<DMA_SOFT_LBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SOFT_LBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SOFT_LBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SOFT_LBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SoftLBReq` writer"]
pub struct W(crate::W<DMA_SOFT_LBREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SOFT_LBREQ_SPEC>;
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
impl From<crate::W<DMA_SOFT_LBREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SOFT_LBREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SoftLBReq` reader - "]
pub type SOFT_LBREQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SoftLBReq` writer - "]
pub type SOFT_LBREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_SOFT_LBREQ_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&self) -> SOFT_LBREQ_R {
        SOFT_LBREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&mut self) -> SOFT_LBREQ_W<0> {
        SOFT_LBREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_SoftLBReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_lbreq](index.html) module"]
pub struct DMA_SOFT_LBREQ_SPEC;
impl crate::RegisterSpec for DMA_SOFT_LBREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_soft_lbreq::R](R) reader structure"]
impl crate::Readable for DMA_SOFT_LBREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_soft_lbreq::W](W) writer structure"]
impl crate::Writable for DMA_SOFT_LBREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SoftLBReq to value 0"]
impl crate::Resettable for DMA_SOFT_LBREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
