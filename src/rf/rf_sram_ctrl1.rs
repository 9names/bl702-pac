#[doc = "Register `rf_sram_ctrl1` reader"]
pub struct R(crate::R<RF_SRAM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SRAM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SRAM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SRAM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_sram_ctrl1` writer"]
pub struct W(crate::W<RF_SRAM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SRAM_CTRL1_SPEC>;
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
impl From<crate::W<RF_SRAM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SRAM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_sram_addr_end` reader - "]
pub type RF_SRAM_ADDR_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_sram_addr_end` writer - "]
pub type RF_SRAM_ADDR_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SRAM_CTRL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `rf_sram_addr_start` reader - "]
pub type RF_SRAM_ADDR_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_sram_addr_start` writer - "]
pub type RF_SRAM_ADDR_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SRAM_CTRL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_addr_end(&self) -> RF_SRAM_ADDR_END_R {
        RF_SRAM_ADDR_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_addr_start(&self) -> RF_SRAM_ADDR_START_R {
        RF_SRAM_ADDR_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_sram_addr_end(&mut self) -> RF_SRAM_ADDR_END_W<0> {
        RF_SRAM_ADDR_END_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_sram_addr_start(&mut self) -> RF_SRAM_ADDR_START_W<16> {
        RF_SRAM_ADDR_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_sram_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_sram_ctrl1](index.html) module"]
pub struct RF_SRAM_CTRL1_SPEC;
impl crate::RegisterSpec for RF_SRAM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_sram_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_SRAM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_sram_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_SRAM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_sram_ctrl1 to value 0"]
impl crate::Resettable for RF_SRAM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
