#[doc = "Register `tzc_rom0_r1` reader"]
pub struct R(crate::R<TZC_ROM0_R1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ROM0_R1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_ROM0_R1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_ROM0_R1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_rom0_r1` writer"]
pub struct W(crate::W<TZC_ROM0_R1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ROM0_R1_SPEC>;
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
impl From<crate::W<TZC_ROM0_R1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_ROM0_R1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_rom0_r1_end` reader - "]
pub type TZC_ROM0_R1_END_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_rom0_r1_end` writer - "]
pub type TZC_ROM0_R1_END_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_ROM0_R1_SPEC, u16, u16, 16, O>;
#[doc = "Field `tzc_rom0_r1_start` reader - "]
pub type TZC_ROM0_R1_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tzc_rom0_r1_start` writer - "]
pub type TZC_ROM0_R1_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TZC_ROM0_R1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r1_end(&self) -> TZC_ROM0_R1_END_R {
        TZC_ROM0_R1_END_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r1_start(&self) -> TZC_ROM0_R1_START_R {
        TZC_ROM0_R1_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tzc_rom0_r1_end(&mut self) -> TZC_ROM0_R1_END_W<0> {
        TZC_ROM0_R1_END_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tzc_rom0_r1_start(&mut self) -> TZC_ROM0_R1_START_W<16> {
        TZC_ROM0_R1_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_rom0_r1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom0_r1](index.html) module"]
pub struct TZC_ROM0_R1_SPEC;
impl crate::RegisterSpec for TZC_ROM0_R1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_rom0_r1::R](R) reader structure"]
impl crate::Readable for TZC_ROM0_R1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_rom0_r1::W](W) writer structure"]
impl crate::Writable for TZC_ROM0_R1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tzc_rom0_r1 to value 0"]
impl crate::Resettable for TZC_ROM0_R1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
