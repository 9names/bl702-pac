#[doc = "Register `ks_int_en` reader"]
pub struct R(crate::R<KS_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KS_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KS_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KS_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ks_int_en` writer"]
pub struct W(crate::W<KS_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KS_INT_EN_SPEC>;
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
impl From<crate::W<KS_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KS_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ks_int_en` reader - "]
pub struct KS_INT_EN_R(crate::FieldReader<bool, bool>);
impl KS_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KS_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KS_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ks_int_en` writer - "]
pub struct KS_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KS_INT_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ks_int_en(&self) -> KS_INT_EN_R {
        KS_INT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ks_int_en(&mut self) -> KS_INT_EN_W {
        KS_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ks_int_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ks_int_en](index.html) module"]
pub struct KS_INT_EN_SPEC;
impl crate::RegisterSpec for KS_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ks_int_en::R](R) reader structure"]
impl crate::Readable for KS_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ks_int_en::W](W) writer structure"]
impl crate::Writable for KS_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ks_int_en to value 0"]
impl crate::Resettable for KS_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
