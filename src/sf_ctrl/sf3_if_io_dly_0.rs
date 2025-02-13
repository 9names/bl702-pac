#[doc = "Register `sf3_if_io_dly_0` reader"]
pub struct R(crate::R<SF3_IF_IO_DLY_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF3_IF_IO_DLY_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF3_IF_IO_DLY_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF3_IF_IO_DLY_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf3_if_io_dly_0` writer"]
pub struct W(crate::W<SF3_IF_IO_DLY_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF3_IF_IO_DLY_0_SPEC>;
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
impl From<crate::W<SF3_IF_IO_DLY_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF3_IF_IO_DLY_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf3_cs_dly_sel` reader - "]
pub type SF3_CS_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_cs_dly_sel` writer - "]
pub type SF3_CS_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_cs2_dly_sel` reader - "]
pub type SF3_CS2_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_cs2_dly_sel` writer - "]
pub type SF3_CS2_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_clk_out_dly_sel` reader - "]
pub type SF3_CLK_OUT_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_clk_out_dly_sel` writer - "]
pub type SF3_CLK_OUT_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_dqs_oe_dly_sel` reader - "]
pub type SF3_DQS_OE_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_dqs_oe_dly_sel` writer - "]
pub type SF3_DQS_OE_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_dqs_di_dly_sel` reader - "]
pub type SF3_DQS_DI_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_dqs_di_dly_sel` writer - "]
pub type SF3_DQS_DI_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_dqs_do_dly_sel` reader - "]
pub type SF3_DQS_DO_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_dqs_do_dly_sel` writer - "]
pub type SF3_DQS_DO_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_cs_dly_sel(&self) -> SF3_CS_DLY_SEL_R {
        SF3_CS_DLY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf3_cs2_dly_sel(&self) -> SF3_CS2_DLY_SEL_R {
        SF3_CS2_DLY_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_clk_out_dly_sel(&self) -> SF3_CLK_OUT_DLY_SEL_R {
        SF3_CLK_OUT_DLY_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf3_dqs_oe_dly_sel(&self) -> SF3_DQS_OE_DLY_SEL_R {
        SF3_DQS_OE_DLY_SEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf3_dqs_di_dly_sel(&self) -> SF3_DQS_DI_DLY_SEL_R {
        SF3_DQS_DI_DLY_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf3_dqs_do_dly_sel(&self) -> SF3_DQS_DO_DLY_SEL_R {
        SF3_DQS_DO_DLY_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_cs_dly_sel(&mut self) -> SF3_CS_DLY_SEL_W<0> {
        SF3_CS_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sf3_cs2_dly_sel(&mut self) -> SF3_CS2_DLY_SEL_W<2> {
        SF3_CS2_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_clk_out_dly_sel(&mut self) -> SF3_CLK_OUT_DLY_SEL_W<8> {
        SF3_CLK_OUT_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn sf3_dqs_oe_dly_sel(&mut self) -> SF3_DQS_OE_DLY_SEL_W<26> {
        SF3_DQS_OE_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sf3_dqs_di_dly_sel(&mut self) -> SF3_DQS_DI_DLY_SEL_W<28> {
        SF3_DQS_DI_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn sf3_dqs_do_dly_sel(&mut self) -> SF3_DQS_DO_DLY_SEL_W<30> {
        SF3_DQS_DO_DLY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf3_if_io_dly_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_0](index.html) module"]
pub struct SF3_IF_IO_DLY_0_SPEC;
impl crate::RegisterSpec for SF3_IF_IO_DLY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf3_if_io_dly_0::R](R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_0::W](W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf3_if_io_dly_0 to value 0"]
impl crate::Resettable for SF3_IF_IO_DLY_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
