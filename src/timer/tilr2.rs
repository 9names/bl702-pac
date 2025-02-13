#[doc = "Register `TILR2` reader"]
pub struct R(crate::R<TILR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TILR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TILR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TILR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TILR2` writer"]
pub struct W(crate::W<TILR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TILR2_SPEC>;
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
impl From<crate::W<TILR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TILR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tilr_0` reader - "]
pub type TILR_0_R = crate::BitReader<bool>;
#[doc = "Field `tilr_0` writer - "]
pub type TILR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TILR2_SPEC, bool, O>;
#[doc = "Field `tilr_1` reader - "]
pub type TILR_1_R = crate::BitReader<bool>;
#[doc = "Field `tilr_1` writer - "]
pub type TILR_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TILR2_SPEC, bool, O>;
#[doc = "Field `tilr_2` reader - "]
pub type TILR_2_R = crate::BitReader<bool>;
#[doc = "Field `tilr_2` writer - "]
pub type TILR_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TILR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&self) -> TILR_0_R {
        TILR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&self) -> TILR_1_R {
        TILR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&self) -> TILR_2_R {
        TILR_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tilr_0(&mut self) -> TILR_0_W<0> {
        TILR_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tilr_1(&mut self) -> TILR_1_W<1> {
        TILR_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tilr_2(&mut self) -> TILR_2_W<2> {
        TILR_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TILR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tilr2](index.html) module"]
pub struct TILR2_SPEC;
impl crate::RegisterSpec for TILR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tilr2::R](R) reader structure"]
impl crate::Readable for TILR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tilr2::W](W) writer structure"]
impl crate::Writable for TILR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TILR2 to value 0"]
impl crate::Resettable for TILR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
