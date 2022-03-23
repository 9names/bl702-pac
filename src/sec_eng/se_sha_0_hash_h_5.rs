#[doc = "Register `se_sha_0_hash_h_5` reader"]
pub struct R(crate::R<SE_SHA_0_HASH_H_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_SHA_0_HASH_H_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_SHA_0_HASH_H_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_SHA_0_HASH_H_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_sha_0_hash_h_5` writer"]
pub struct W(crate::W<SE_SHA_0_HASH_H_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_SHA_0_HASH_H_5_SPEC>;
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
impl From<crate::W<SE_SHA_0_HASH_H_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_SHA_0_HASH_H_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_sha_0_hash_h_5` reader - "]
pub struct SE_SHA_0_HASH_H_5_R(crate::FieldReader<u32, u32>);
impl SE_SHA_0_HASH_H_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        SE_SHA_0_HASH_H_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_HASH_H_5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_hash_h_5` writer - "]
pub struct SE_SHA_0_HASH_H_5_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_HASH_H_5_W<'a> {
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
    pub fn se_sha_0_hash_h_5(&self) -> SE_SHA_0_HASH_H_5_R {
        SE_SHA_0_HASH_H_5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_h_5(&mut self) -> SE_SHA_0_HASH_H_5_W {
        SE_SHA_0_HASH_H_5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_sha_0_hash_h_5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_5](index.html) module"]
pub struct SE_SHA_0_HASH_H_5_SPEC;
impl crate::RegisterSpec for SE_SHA_0_HASH_H_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_sha_0_hash_h_5::R](R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_sha_0_hash_h_5::W](W) writer structure"]
impl crate::Writable for SE_SHA_0_HASH_H_5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_sha_0_hash_h_5 to value 0"]
impl crate::Resettable for SE_SHA_0_HASH_H_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
