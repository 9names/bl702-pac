#[doc = "Register `spi_prd_1` reader"]
pub struct R(crate::R<SPI_PRD_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_PRD_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_PRD_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_PRD_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_prd_1` writer"]
pub struct W(crate::W<SPI_PRD_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_PRD_1_SPEC>;
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
impl From<crate::W<SPI_PRD_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_PRD_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_prd_i` reader - "]
pub type CR_SPI_PRD_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_prd_i` writer - "]
pub type CR_SPI_PRD_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_PRD_1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&self) -> CR_SPI_PRD_I_R {
        CR_SPI_PRD_I_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&mut self) -> CR_SPI_PRD_I_W<0> {
        CR_SPI_PRD_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_prd_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_prd_1](index.html) module"]
pub struct SPI_PRD_1_SPEC;
impl crate::RegisterSpec for SPI_PRD_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_prd_1::R](R) reader structure"]
impl crate::Readable for SPI_PRD_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_prd_1::W](W) writer structure"]
impl crate::Writable for SPI_PRD_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_prd_1 to value 0"]
impl crate::Resettable for SPI_PRD_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
