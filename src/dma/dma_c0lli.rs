#[doc = "Register `DMA_C0LLI` reader"]
pub struct R(crate::R<DMA_C0LLI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_C0LLI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_C0LLI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_C0LLI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_C0LLI` writer"]
pub struct W(crate::W<DMA_C0LLI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_C0LLI_SPEC>;
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
impl From<crate::W<DMA_C0LLI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_C0LLI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLI` reader - "]
pub struct LLI_R(crate::FieldReader<u32, u32>);
impl LLI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LLI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLI_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLI` writer - "]
pub struct LLI_W<'a> {
    w: &'a mut W,
}
impl<'a> LLI_W<'a> {
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
    pub fn lli(&self) -> LLI_R {
        LLI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn lli(&mut self) -> LLI_W {
        LLI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_C0LLI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0lli](index.html) module"]
pub struct DMA_C0LLI_SPEC;
impl crate::RegisterSpec for DMA_C0LLI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_c0lli::R](R) reader structure"]
impl crate::Readable for DMA_C0LLI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_c0lli::W](W) writer structure"]
impl crate::Writable for DMA_C0LLI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_C0LLI to value 0"]
impl crate::Resettable for DMA_C0LLI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
