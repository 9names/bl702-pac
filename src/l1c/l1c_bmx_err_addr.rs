#[doc = "Register `l1c_bmx_err_addr` reader"]
pub struct R(crate::R<L1C_BMX_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_BMX_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1C_BMX_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1C_BMX_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l1c_bmx_err_addr` writer"]
pub struct W(crate::W<L1C_BMX_ERR_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1C_BMX_ERR_ADDR_SPEC>;
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
impl From<crate::W<L1C_BMX_ERR_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1C_BMX_ERR_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l1c_bmx_err_addr` reader - "]
pub type L1C_BMX_ERR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `l1c_bmx_err_addr` writer - "]
pub type L1C_BMX_ERR_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, L1C_BMX_ERR_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&self) -> L1C_BMX_ERR_ADDR_R {
        L1C_BMX_ERR_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn l1c_bmx_err_addr(&mut self) -> L1C_BMX_ERR_ADDR_W<0> {
        L1C_BMX_ERR_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "l1c_bmx_err_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_bmx_err_addr](index.html) module"]
pub struct L1C_BMX_ERR_ADDR_SPEC;
impl crate::RegisterSpec for L1C_BMX_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_bmx_err_addr::R](R) reader structure"]
impl crate::Readable for L1C_BMX_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1c_bmx_err_addr::W](W) writer structure"]
impl crate::Writable for L1C_BMX_ERR_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets l1c_bmx_err_addr to value 0"]
impl crate::Resettable for L1C_BMX_ERR_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
