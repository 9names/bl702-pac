#[doc = "Register `rf_singen_2` reader"]
pub struct R(crate::R<RF_SINGEN_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SINGEN_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SINGEN_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SINGEN_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_singen_2` writer"]
pub struct W(crate::W<RF_SINGEN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SINGEN_2_SPEC>;
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
impl From<crate::W<RF_SINGEN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SINGEN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_gain_i` reader - "]
pub type SINGEN_GAIN_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_gain_i` writer - "]
pub type SINGEN_GAIN_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_2_SPEC, u16, u16, 11, O>;
#[doc = "Field `singen_start_addr1_i` reader - "]
pub type SINGEN_START_ADDR1_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_start_addr1_i` writer - "]
pub type SINGEN_START_ADDR1_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_2_SPEC, u16, u16, 10, O>;
#[doc = "Field `singen_start_addr0_i` reader - "]
pub type SINGEN_START_ADDR0_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_start_addr0_i` writer - "]
pub type SINGEN_START_ADDR0_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_2_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&self) -> SINGEN_GAIN_I_R {
        SINGEN_GAIN_I_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&self) -> SINGEN_START_ADDR1_I_R {
        SINGEN_START_ADDR1_I_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&self) -> SINGEN_START_ADDR0_I_R {
        SINGEN_START_ADDR0_I_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&mut self) -> SINGEN_GAIN_I_W<0> {
        SINGEN_GAIN_I_W::new(self)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&mut self) -> SINGEN_START_ADDR1_I_W<12> {
        SINGEN_START_ADDR1_I_W::new(self)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&mut self) -> SINGEN_START_ADDR0_I_W<22> {
        SINGEN_START_ADDR0_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_singen_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_singen_2](index.html) module"]
pub struct RF_SINGEN_2_SPEC;
impl crate::RegisterSpec for RF_SINGEN_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_singen_2::R](R) reader structure"]
impl crate::Readable for RF_SINGEN_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_singen_2::W](W) writer structure"]
impl crate::Writable for RF_SINGEN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_singen_2 to value 0"]
impl crate::Resettable for RF_SINGEN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
