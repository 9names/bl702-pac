#[doc = "Register `DMA_IntStatus` reader"]
pub struct R(crate::R<DMA_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IntStatus` writer"]
pub struct W(crate::W<DMA_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_STATUS_SPEC>;
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
impl From<crate::W<DMA_INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IntStatus` reader - "]
pub type INT_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IntStatus` writer - "]
pub type INT_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMA_INT_STATUS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_status(&self) -> INT_STATUS_R {
        INT_STATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_status(&mut self) -> INT_STATUS_W<0> {
        INT_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IntStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_status](index.html) module"]
pub struct DMA_INT_STATUS_SPEC;
impl crate::RegisterSpec for DMA_INT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_status::R](R) reader structure"]
impl crate::Readable for DMA_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_status::W](W) writer structure"]
impl crate::Writable for DMA_INT_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_IntStatus to value 0"]
impl crate::Resettable for DMA_INT_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
