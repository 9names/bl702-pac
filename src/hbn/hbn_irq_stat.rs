#[doc = "Register `HBN_IRQ_STAT` reader"]
pub struct R(crate::R<HBN_IRQ_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_IRQ_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_IRQ_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_IRQ_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_IRQ_STAT` writer"]
pub struct W(crate::W<HBN_IRQ_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_IRQ_STAT_SPEC>;
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
impl From<crate::W<HBN_IRQ_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_IRQ_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irq_stat` reader - "]
pub struct IRQ_STAT_R(crate::FieldReader<u32, u32>);
impl IRQ_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        IRQ_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRQ_STAT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irq_stat` writer - "]
pub struct IRQ_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQ_STAT_W<'a> {
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
    pub fn irq_stat(&self) -> IRQ_STAT_R {
        IRQ_STAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irq_stat(&mut self) -> IRQ_STAT_W {
        IRQ_STAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_IRQ_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_irq_stat](index.html) module"]
pub struct HBN_IRQ_STAT_SPEC;
impl crate::RegisterSpec for HBN_IRQ_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_irq_stat::R](R) reader structure"]
impl crate::Readable for HBN_IRQ_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_irq_stat::W](W) writer structure"]
impl crate::Writable for HBN_IRQ_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_IRQ_STAT to value 0"]
impl crate::Resettable for HBN_IRQ_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
