#[doc = "Register `GPIO_INT_CLR1` reader"]
pub struct R(crate::R<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_CLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_CLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_CLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_CLR1` writer"]
pub struct W(crate::W<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_CLR1_SPEC>;
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
impl From<crate::W<GPIO_INT_CLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_CLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_int_clr1` reader - "]
pub type REG_GPIO_INT_CLR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_gpio_int_clr1` writer - "]
pub type REG_GPIO_INT_CLR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_INT_CLR1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_clr1(&self) -> REG_GPIO_INT_CLR1_R {
        REG_GPIO_INT_CLR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_gpio_int_clr1(&mut self) -> REG_GPIO_INT_CLR1_W<0> {
        REG_GPIO_INT_CLR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_INT_CLR1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_clr1](index.html) module"]
pub struct GPIO_INT_CLR1_SPEC;
impl crate::RegisterSpec for GPIO_INT_CLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_clr1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_CLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_clr1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_CLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_CLR1 to value 0"]
impl crate::Resettable for GPIO_INT_CLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
