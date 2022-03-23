#[doc = "Register `dg_testbus_0` reader"]
pub struct R(crate::R<DG_TESTBUS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DG_TESTBUS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DG_TESTBUS_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DG_TESTBUS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dg_testbus_0` writer"]
pub struct W(crate::W<DG_TESTBUS_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DG_TESTBUS_0_SPEC>;
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
impl From<crate::W<DG_TESTBUS_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DG_TESTBUS_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_testbus_read` reader - "]
pub struct RF_TESTBUS_READ_R(crate::FieldReader<u32, u32>);
impl RF_TESTBUS_READ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RF_TESTBUS_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TESTBUS_READ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_testbus_read` writer - "]
pub struct RF_TESTBUS_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TESTBUS_READ_W<'a> {
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
    pub fn rf_testbus_read(&self) -> RF_TESTBUS_READ_R {
        RF_TESTBUS_READ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rf_testbus_read(&mut self) -> RF_TESTBUS_READ_W {
        RF_TESTBUS_READ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dg_testbus_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dg_testbus_0](index.html) module"]
pub struct DG_TESTBUS_0_SPEC;
impl crate::RegisterSpec for DG_TESTBUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dg_testbus_0::R](R) reader structure"]
impl crate::Readable for DG_TESTBUS_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dg_testbus_0::W](W) writer structure"]
impl crate::Writable for DG_TESTBUS_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dg_testbus_0 to value 0"]
impl crate::Resettable for DG_TESTBUS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
