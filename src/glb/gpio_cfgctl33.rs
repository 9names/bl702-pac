#[doc = "Register `GPIO_CFGCTL33` reader"]
pub struct R(crate::R<GPIO_CFGCTL33_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL33_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL33_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL33_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL33` writer"]
pub struct W(crate::W<GPIO_CFGCTL33_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL33_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL33_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL33_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL33.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl33](index.html) module"]
pub struct GPIO_CFGCTL33_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl33::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL33_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl33::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL33_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL33 to value 0"]
impl crate::Resettable for GPIO_CFGCTL33_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
