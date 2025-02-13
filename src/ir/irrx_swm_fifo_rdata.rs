#[doc = "Register `irrx_swm_fifo_rdata` reader"]
pub struct R(crate::R<IRRX_SWM_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_SWM_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_SWM_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_SWM_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_swm_fifo_rdata` writer"]
pub struct W(crate::W<IRRX_SWM_FIFO_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_SWM_FIFO_RDATA_SPEC>;
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
impl From<crate::W<IRRX_SWM_FIFO_RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRRX_SWM_FIFO_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_fifo_rdata` reader - "]
pub type RX_FIFO_RDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rx_fifo_rdata` writer - "]
pub type RX_FIFO_RDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IRRX_SWM_FIFO_RDATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&self) -> RX_FIFO_RDATA_R {
        RX_FIFO_RDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rx_fifo_rdata(&mut self) -> RX_FIFO_RDATA_W<0> {
        RX_FIFO_RDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_swm_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_swm_fifo_rdata](index.html) module"]
pub struct IRRX_SWM_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for IRRX_SWM_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_swm_fifo_rdata::R](R) reader structure"]
impl crate::Readable for IRRX_SWM_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_swm_fifo_rdata::W](W) writer structure"]
impl crate::Writable for IRRX_SWM_FIFO_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irrx_swm_fifo_rdata to value 0"]
impl crate::Resettable for IRRX_SWM_FIFO_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
