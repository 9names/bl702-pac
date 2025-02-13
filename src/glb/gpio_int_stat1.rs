#[doc = "Register `GPIO_INT_STAT1` reader"]
pub struct R(crate::R<GPIO_INT_STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_STAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_STAT1` writer"]
pub struct W(crate::W<GPIO_INT_STAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_STAT1_SPEC>;
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
impl From<crate::W<GPIO_INT_STAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_STAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpio_int_stat1` reader - "]
pub type GPIO_INT_STAT1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `gpio_int_stat1` writer - "]
pub type GPIO_INT_STAT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_INT_STAT1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_int_stat1(&self) -> GPIO_INT_STAT1_R {
        GPIO_INT_STAT1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpio_int_stat1(&mut self) -> GPIO_INT_STAT1_W<0> {
        GPIO_INT_STAT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_INT_STAT1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_stat1](index.html) module"]
pub struct GPIO_INT_STAT1_SPEC;
impl crate::RegisterSpec for GPIO_INT_STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_stat1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_STAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_stat1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_STAT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_STAT1 to value 0"]
impl crate::Resettable for GPIO_INT_STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
