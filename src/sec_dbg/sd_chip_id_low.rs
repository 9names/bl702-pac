#[doc = "Register `sd_chip_id_low` reader"]
pub struct R(crate::R<SD_CHIP_ID_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_CHIP_ID_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_CHIP_ID_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_CHIP_ID_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_chip_id_low` writer"]
pub struct W(crate::W<SD_CHIP_ID_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_CHIP_ID_LOW_SPEC>;
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
impl From<crate::W<SD_CHIP_ID_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_CHIP_ID_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sd_chip_id_low` reader - "]
pub type SD_CHIP_ID_LOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sd_chip_id_low` writer - "]
pub type SD_CHIP_ID_LOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SD_CHIP_ID_LOW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_low(&self) -> SD_CHIP_ID_LOW_R {
        SD_CHIP_ID_LOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_chip_id_low(&mut self) -> SD_CHIP_ID_LOW_W<0> {
        SD_CHIP_ID_LOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sd_chip_id_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_chip_id_low](index.html) module"]
pub struct SD_CHIP_ID_LOW_SPEC;
impl crate::RegisterSpec for SD_CHIP_ID_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_chip_id_low::R](R) reader structure"]
impl crate::Readable for SD_CHIP_ID_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_chip_id_low::W](W) writer structure"]
impl crate::Writable for SD_CHIP_ID_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sd_chip_id_low to value 0"]
impl crate::Resettable for SD_CHIP_ID_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
