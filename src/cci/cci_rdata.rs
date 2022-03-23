#[doc = "Register `cci_rdata` reader"]
pub struct R(crate::R<CCI_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCI_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCI_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cci_rdata` writer"]
pub struct W(crate::W<CCI_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCI_RDATA_SPEC>;
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
impl From<crate::W<CCI_RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCI_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `apb_cci_rdata` reader - "]
pub struct APB_CCI_RDATA_R(crate::FieldReader<u32, u32>);
impl APB_CCI_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        APB_CCI_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_CCI_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `apb_cci_rdata` writer - "]
pub struct APB_CCI_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CCI_RDATA_W<'a> {
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
    pub fn apb_cci_rdata(&self) -> APB_CCI_RDATA_R {
        APB_CCI_RDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_rdata(&mut self) -> APB_CCI_RDATA_W {
        APB_CCI_RDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cci_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_rdata](index.html) module"]
pub struct CCI_RDATA_SPEC;
impl crate::RegisterSpec for CCI_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_rdata::R](R) reader structure"]
impl crate::Readable for CCI_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cci_rdata::W](W) writer structure"]
impl crate::Writable for CCI_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cci_rdata to value 0"]
impl crate::Resettable for CCI_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
