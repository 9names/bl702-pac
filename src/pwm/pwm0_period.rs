#[doc = "Register `pwm0_period` reader"]
pub struct R(crate::R<PWM0_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM0_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM0_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM0_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm0_period` writer"]
pub struct W(crate::W<PWM0_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM0_PERIOD_SPEC>;
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
impl From<crate::W<PWM0_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM0_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_period` reader - "]
pub type PWM_PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pwm_period` writer - "]
pub type PWM_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWM0_PERIOD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&self) -> PWM_PERIOD_R {
        PWM_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&mut self) -> PWM_PERIOD_W<0> {
        PWM_PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm0_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm0_period](index.html) module"]
pub struct PWM0_PERIOD_SPEC;
impl crate::RegisterSpec for PWM0_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm0_period::R](R) reader structure"]
impl crate::Readable for PWM0_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm0_period::W](W) writer structure"]
impl crate::Writable for PWM0_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pwm0_period to value 0"]
impl crate::Resettable for PWM0_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
