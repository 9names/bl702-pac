#[doc = "Register `pds_ram1` reader"]
pub struct R(crate::R<PDS_RAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_RAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_RAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_RAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_ram1` writer"]
pub struct W(crate::W<PDS_RAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_RAM1_SPEC>;
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
impl From<crate::W<PDS_RAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_RAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_ram_ret1n` reader - "]
pub type CR_PDS_RAM_RET1N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ram_ret1n` writer - "]
pub type CR_PDS_RAM_RET1N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_pds_ram_ret2n` reader - "]
pub type CR_PDS_RAM_RET2N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ram_ret2n` writer - "]
pub type CR_PDS_RAM_RET2N_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_pds_ram_pgen` reader - "]
pub type CR_PDS_RAM_PGEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_ram_pgen` writer - "]
pub type CR_PDS_RAM_PGEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_RAM1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_pds_ram_ret1n(&self) -> CR_PDS_RAM_RET1N_R {
        CR_PDS_RAM_RET1N_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_pds_ram_ret2n(&self) -> CR_PDS_RAM_RET2N_R {
        CR_PDS_RAM_RET2N_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_pds_ram_pgen(&self) -> CR_PDS_RAM_PGEN_R {
        CR_PDS_RAM_PGEN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_pds_ram_ret1n(&mut self) -> CR_PDS_RAM_RET1N_W<0> {
        CR_PDS_RAM_RET1N_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_pds_ram_ret2n(&mut self) -> CR_PDS_RAM_RET2N_W<4> {
        CR_PDS_RAM_RET2N_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_pds_ram_pgen(&mut self) -> CR_PDS_RAM_PGEN_W<8> {
        CR_PDS_RAM_PGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_ram1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ram1](index.html) module"]
pub struct PDS_RAM1_SPEC;
impl crate::RegisterSpec for PDS_RAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ram1::R](R) reader structure"]
impl crate::Readable for PDS_RAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ram1::W](W) writer structure"]
impl crate::Writable for PDS_RAM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pds_ram1 to value 0"]
impl crate::Resettable for PDS_RAM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
