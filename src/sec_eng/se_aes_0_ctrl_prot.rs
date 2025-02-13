#[doc = "Register `se_aes_0_ctrl_prot` reader"]
pub struct R(crate::R<SE_AES_0_CTRL_PROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_CTRL_PROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_AES_0_CTRL_PROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_AES_0_CTRL_PROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_ctrl_prot` writer"]
pub struct W(crate::W<SE_AES_0_CTRL_PROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_CTRL_PROT_SPEC>;
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
impl From<crate::W<SE_AES_0_CTRL_PROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_AES_0_CTRL_PROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_prot_en` reader - "]
pub type SE_AES_PROT_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_prot_en` writer - "]
pub type SE_AES_PROT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_PROT_SPEC, bool, O>;
#[doc = "Field `se_aes_id0_en` reader - "]
pub type SE_AES_ID0_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_id0_en` writer - "]
pub type SE_AES_ID0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_PROT_SPEC, bool, O>;
#[doc = "Field `se_aes_id1_en` reader - "]
pub type SE_AES_ID1_EN_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_id1_en` writer - "]
pub type SE_AES_ID1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_AES_0_CTRL_PROT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_prot_en(&self) -> SE_AES_PROT_EN_R {
        SE_AES_PROT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_id0_en(&self) -> SE_AES_ID0_EN_R {
        SE_AES_ID0_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_id1_en(&self) -> SE_AES_ID1_EN_R {
        SE_AES_ID1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_prot_en(&mut self) -> SE_AES_PROT_EN_W<0> {
        SE_AES_PROT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_id0_en(&mut self) -> SE_AES_ID0_EN_W<1> {
        SE_AES_ID0_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_id1_en(&mut self) -> SE_AES_ID1_EN_W<2> {
        SE_AES_ID1_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_ctrl_prot.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_ctrl_prot](index.html) module"]
pub struct SE_AES_0_CTRL_PROT_SPEC;
impl crate::RegisterSpec for SE_AES_0_CTRL_PROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_ctrl_prot::R](R) reader structure"]
impl crate::Readable for SE_AES_0_CTRL_PROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_ctrl_prot::W](W) writer structure"]
impl crate::Writable for SE_AES_0_CTRL_PROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_aes_0_ctrl_prot to value 0"]
impl crate::Resettable for SE_AES_0_CTRL_PROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
