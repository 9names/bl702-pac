#[doc = "Register `DMA_SoftBReq` reader"]
pub struct R(crate::R<DMA_SOFT_BREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SOFT_BREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SOFT_BREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SOFT_BREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SoftBReq` writer"]
pub struct W(crate::W<DMA_SOFT_BREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SOFT_BREQ_SPEC>;
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
impl From<crate::W<DMA_SOFT_BREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SOFT_BREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SoftBReq` reader - "]
pub type SOFT_BREQ_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SoftBReq` writer - "]
pub type SOFT_BREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_SOFT_BREQ_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&self) -> SOFT_BREQ_R {
        SOFT_BREQ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SOFT_BREQ_W<0> {
        SOFT_BREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_SoftBReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_breq](index.html) module"]
pub struct DMA_SOFT_BREQ_SPEC;
impl crate::RegisterSpec for DMA_SOFT_BREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_soft_breq::R](R) reader structure"]
impl crate::Readable for DMA_SOFT_BREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_soft_breq::W](W) writer structure"]
impl crate::Writable for DMA_SOFT_BREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SoftBReq to value 0"]
impl crate::Resettable for DMA_SOFT_BREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
