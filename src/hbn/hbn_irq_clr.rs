#[doc = "Register `HBN_IRQ_CLR` reader"]
pub struct R(crate::R<HBN_IRQ_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_IRQ_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_IRQ_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_IRQ_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_IRQ_CLR` writer"]
pub struct W(crate::W<HBN_IRQ_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_IRQ_CLR_SPEC>;
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
impl From<crate::W<HBN_IRQ_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_IRQ_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq_clr` reader - "]
pub type IRQ_CLR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `irq_clr` writer - "]
pub type IRQ_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_IRQ_CLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_clr(&self) -> IRQ_CLR_R {
        IRQ_CLR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_clr(&mut self) -> IRQ_CLR_W<0> {
        IRQ_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_IRQ_CLR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_clr](index.html) module"]
pub struct HBN_IRQ_CLR_SPEC;
impl crate::RegisterSpec for HBN_IRQ_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_irq_clr::R](R) reader structure"]
impl crate::Readable for HBN_IRQ_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_irq_clr::W](W) writer structure"]
impl crate::Writable for HBN_IRQ_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_IRQ_CLR to value 0"]
impl crate::Resettable for HBN_IRQ_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
