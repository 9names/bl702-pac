#[doc = "Register `gpadc_reg_raw_result` reader"]
pub struct R(crate::R<GPADC_REG_RAW_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_RAW_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_RAW_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_RAW_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_raw_result` writer"]
pub struct W(crate::W<GPADC_REG_RAW_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_RAW_RESULT_SPEC>;
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
impl From<crate::W<GPADC_REG_RAW_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_RAW_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_raw_data` reader - "]
pub type GPADC_RAW_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `gpadc_raw_data` writer - "]
pub type GPADC_RAW_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_REG_RAW_RESULT_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GPADC_RAW_DATA_R {
        GPADC_RAW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&mut self) -> GPADC_RAW_DATA_W<0> {
        GPADC_RAW_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_raw_result.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_raw_result](index.html) module"]
pub struct GPADC_REG_RAW_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RAW_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_raw_result::R](R) reader structure"]
impl crate::Readable for GPADC_REG_RAW_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_raw_result::W](W) writer structure"]
impl crate::Writable for GPADC_REG_RAW_RESULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_raw_result to value 0"]
impl crate::Resettable for GPADC_REG_RAW_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
