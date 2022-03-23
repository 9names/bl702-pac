#[doc = "Register `spi_fifo_rdata` reader"]
pub struct R(crate::R<SPI_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_fifo_rdata` writer"]
pub struct W(crate::W<SPI_FIFO_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FIFO_RDATA_SPEC>;
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
impl From<crate::W<SPI_FIFO_RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FIFO_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_fifo_rdata` reader - "]
pub struct SPI_FIFO_RDATA_R(crate::FieldReader<u32, u32>);
impl SPI_FIFO_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SPI_FIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_FIFO_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_fifo_rdata` writer - "]
pub struct SPI_FIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FIFO_RDATA_W<'a> {
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
    pub fn spi_fifo_rdata(&self) -> SPI_FIFO_RDATA_R {
        SPI_FIFO_RDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&mut self) -> SPI_FIFO_RDATA_W {
        SPI_FIFO_RDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_rdata](index.html) module"]
pub struct SPI_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for SPI_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fifo_rdata::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fifo_rdata::W](W) writer structure"]
impl crate::Writable for SPI_FIFO_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_fifo_rdata to value 0"]
impl crate::Resettable for SPI_FIFO_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
