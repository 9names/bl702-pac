#[doc = "Register `HBN_RSV3` reader"]
pub struct R(crate::R<HBN_RSV3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_RSV3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_RSV3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_RSV3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_RSV3` writer"]
pub struct W(crate::W<HBN_RSV3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_RSV3_SPEC>;
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
impl From<crate::W<HBN_RSV3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_RSV3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HBN_RSV3` reader - "]
pub type HBN_RSV3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HBN_RSV3` writer - "]
pub type HBN_RSV3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HBN_RSV3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv3(&self) -> HBN_RSV3_R {
        HBN_RSV3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbn_rsv3(&mut self) -> HBN_RSV3_W<0> {
        HBN_RSV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_RSV3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_rsv3](index.html) module"]
pub struct HBN_RSV3_SPEC;
impl crate::RegisterSpec for HBN_RSV3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_rsv3::R](R) reader structure"]
impl crate::Readable for HBN_RSV3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_rsv3::W](W) writer structure"]
impl crate::Writable for HBN_RSV3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_RSV3 to value 0"]
impl crate::Resettable for HBN_RSV3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
