#[doc = "Register `se_aes_0_sboot` reader"]
pub struct R(crate::R<SE_AES_0_SBOOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_SBOOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_AES_0_SBOOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_AES_0_SBOOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_sboot` writer"]
pub struct W(crate::W<SE_AES_0_SBOOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_SBOOT_SPEC>;
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
impl From<crate::W<SE_AES_0_SBOOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_AES_0_SBOOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_0_sboot_key_sel` reader - "]
pub type SE_AES_0_SBOOT_KEY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_0_sboot_key_sel` writer - "]
pub type SE_AES_0_SBOOT_KEY_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_SBOOT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_sboot_key_sel(&self) -> SE_AES_0_SBOOT_KEY_SEL_R {
        SE_AES_0_SBOOT_KEY_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_sboot_key_sel(&mut self) -> SE_AES_0_SBOOT_KEY_SEL_W<0> {
        SE_AES_0_SBOOT_KEY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_sboot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_sboot](index.html) module"]
pub struct SE_AES_0_SBOOT_SPEC;
impl crate::RegisterSpec for SE_AES_0_SBOOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_sboot::R](R) reader structure"]
impl crate::Readable for SE_AES_0_SBOOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_sboot::W](W) writer structure"]
impl crate::Writable for SE_AES_0_SBOOT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_aes_0_sboot to value 0"]
impl crate::Resettable for SE_AES_0_SBOOT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
