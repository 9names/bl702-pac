#[doc = "Register `ef_crc_ctrl_1` reader"]
pub struct R(crate::R<EF_CRC_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CRC_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_CRC_CTRL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_CRC_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_crc_ctrl_1` writer"]
pub struct W(crate::W<EF_CRC_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CRC_CTRL_1_SPEC>;
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
impl From<crate::W<EF_CRC_CTRL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_CRC_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_crc_data_0_en` reader - "]
pub struct EF_CRC_DATA_0_EN_R(crate::FieldReader<u32, u32>);
impl EF_CRC_DATA_0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        EF_CRC_DATA_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_DATA_0_EN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_data_0_en` writer - "]
pub struct EF_CRC_DATA_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DATA_0_EN_W<'a> {
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
    pub fn ef_crc_data_0_en(&self) -> EF_CRC_DATA_0_EN_R {
        EF_CRC_DATA_0_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_crc_data_0_en(&mut self) -> EF_CRC_DATA_0_EN_W {
        EF_CRC_DATA_0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_crc_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_1](index.html) module"]
pub struct EF_CRC_CTRL_1_SPEC;
impl crate::RegisterSpec for EF_CRC_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_crc_ctrl_1::R](R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_1::W](W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_crc_ctrl_1 to value 0"]
impl crate::Resettable for EF_CRC_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
