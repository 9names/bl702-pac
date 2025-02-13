#[doc = "Register `reg_key_slot_11_w0` reader"]
pub struct R(crate::R<REG_KEY_SLOT_11_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_KEY_SLOT_11_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_KEY_SLOT_11_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_KEY_SLOT_11_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `reg_key_slot_11_w0` writer"]
pub struct W(crate::W<REG_KEY_SLOT_11_W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_KEY_SLOT_11_W0_SPEC>;
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
impl From<crate::W<REG_KEY_SLOT_11_W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_KEY_SLOT_11_W0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reg_key_slot_11_w0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_key_slot_11_w0](index.html) module"]
pub struct REG_KEY_SLOT_11_W0_SPEC;
impl crate::RegisterSpec for REG_KEY_SLOT_11_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_key_slot_11_w0::R](R) reader structure"]
impl crate::Readable for REG_KEY_SLOT_11_W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_key_slot_11_w0::W](W) writer structure"]
impl crate::Writable for REG_KEY_SLOT_11_W0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets reg_key_slot_11_w0 to value 0"]
impl crate::Resettable for REG_KEY_SLOT_11_W0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
