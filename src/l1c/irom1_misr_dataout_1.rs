#[doc = "Register `irom1_misr_dataout_1` reader"]
pub struct R(crate::R<IROM1_MISR_DATAOUT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IROM1_MISR_DATAOUT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IROM1_MISR_DATAOUT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IROM1_MISR_DATAOUT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irom1_misr_dataout_1` writer"]
pub struct W(crate::W<IROM1_MISR_DATAOUT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IROM1_MISR_DATAOUT_1_SPEC>;
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
impl From<crate::W<IROM1_MISR_DATAOUT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IROM1_MISR_DATAOUT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irom1_misr_dataout_1` reader - "]
pub type IROM1_MISR_DATAOUT_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `irom1_misr_dataout_1` writer - "]
pub type IROM1_MISR_DATAOUT_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IROM1_MISR_DATAOUT_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_1(&self) -> IROM1_MISR_DATAOUT_1_R {
        IROM1_MISR_DATAOUT_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn irom1_misr_dataout_1(&mut self) -> IROM1_MISR_DATAOUT_1_W<0> {
        IROM1_MISR_DATAOUT_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irom1_misr_dataout_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irom1_misr_dataout_1](index.html) module"]
pub struct IROM1_MISR_DATAOUT_1_SPEC;
impl crate::RegisterSpec for IROM1_MISR_DATAOUT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irom1_misr_dataout_1::R](R) reader structure"]
impl crate::Readable for IROM1_MISR_DATAOUT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irom1_misr_dataout_1::W](W) writer structure"]
impl crate::Writable for IROM1_MISR_DATAOUT_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irom1_misr_dataout_1 to value 0"]
impl crate::Resettable for IROM1_MISR_DATAOUT_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
