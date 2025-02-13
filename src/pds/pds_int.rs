#[doc = "Register `PDS_INT` reader"]
pub struct R(crate::R<PDS_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_INT` writer"]
pub struct W(crate::W<PDS_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_INT_SPEC>;
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
impl From<crate::W<PDS_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ro_pds_wake_int` reader - "]
pub type RO_PDS_WAKE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ro_pds_wake_int` writer - "]
pub type RO_PDS_WAKE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub type RO_PDS_RF_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ro_pds_rf_done_int` writer - "]
pub type RO_PDS_RF_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `ro_pds_pll_done_int` reader - "]
pub type RO_PDS_PLL_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ro_pds_pll_done_int` writer - "]
pub type RO_PDS_PLL_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `pds_reset_event` reader - "]
pub type PDS_RESET_EVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_reset_event` writer - "]
pub type PDS_RESET_EVENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_INT_SPEC, u8, u8, 3, O>;
#[doc = "Field `pds_clr_reset_event` reader - "]
pub type PDS_CLR_RESET_EVENT_R = crate::BitReader<bool>;
#[doc = "Field `pds_clr_reset_event` writer - "]
pub type PDS_CLR_RESET_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub type CR_PDS_WAKE_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub type CR_PDS_WAKE_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub type CR_PDS_RF_DONE_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub type CR_PDS_RF_DONE_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_pll_done_int_mask` reader - "]
pub type CR_PDS_PLL_DONE_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_pll_done_int_mask` writer - "]
pub type CR_PDS_PLL_DONE_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_int_clr` reader - "]
pub type CR_PDS_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_int_clr` writer - "]
pub type CR_PDS_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_INT_SPEC, bool, O>;
#[doc = "Field `cr_pds_wakeup_src_en` reader - "]
pub type CR_PDS_WAKEUP_SRC_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_pds_wakeup_src_en` writer - "]
pub type CR_PDS_WAKEUP_SRC_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_INT_SPEC, u8, u8, 8, O>;
#[doc = "Field `ro_pds_wakeup_event` reader - "]
pub type RO_PDS_WAKEUP_EVENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ro_pds_wakeup_event` writer - "]
pub type RO_PDS_WAKEUP_EVENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_INT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RO_PDS_WAKE_INT_R {
        RO_PDS_WAKE_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RO_PDS_RF_DONE_INT_R {
        RO_PDS_RF_DONE_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&self) -> RO_PDS_PLL_DONE_INT_R {
        RO_PDS_PLL_DONE_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_reset_event(&self) -> PDS_RESET_EVENT_R {
        PDS_RESET_EVENT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&self) -> PDS_CLR_RESET_EVENT_R {
        PDS_CLR_RESET_EVENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CR_PDS_WAKE_INT_MASK_R {
        CR_PDS_WAKE_INT_MASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CR_PDS_RF_DONE_INT_MASK_R {
        CR_PDS_RF_DONE_INT_MASK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&self) -> CR_PDS_PLL_DONE_INT_MASK_R {
        CR_PDS_PLL_DONE_INT_MASK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CR_PDS_INT_CLR_R {
        CR_PDS_INT_CLR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&self) -> CR_PDS_WAKEUP_SRC_EN_R {
        CR_PDS_WAKEUP_SRC_EN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&self) -> RO_PDS_WAKEUP_EVENT_R {
        RO_PDS_WAKEUP_EVENT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&mut self) -> RO_PDS_WAKE_INT_W<0> {
        RO_PDS_WAKE_INT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&mut self) -> RO_PDS_RF_DONE_INT_W<2> {
        RO_PDS_RF_DONE_INT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&mut self) -> RO_PDS_PLL_DONE_INT_W<3> {
        RO_PDS_PLL_DONE_INT_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_reset_event(&mut self) -> PDS_RESET_EVENT_W<4> {
        PDS_RESET_EVENT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&mut self) -> PDS_CLR_RESET_EVENT_W<7> {
        PDS_CLR_RESET_EVENT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&mut self) -> CR_PDS_WAKE_INT_MASK_W<8> {
        CR_PDS_WAKE_INT_MASK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CR_PDS_RF_DONE_INT_MASK_W<10> {
        CR_PDS_RF_DONE_INT_MASK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&mut self) -> CR_PDS_PLL_DONE_INT_MASK_W<11> {
        CR_PDS_PLL_DONE_INT_MASK_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&mut self) -> CR_PDS_INT_CLR_W<15> {
        CR_PDS_INT_CLR_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&mut self) -> CR_PDS_WAKEUP_SRC_EN_W<16> {
        CR_PDS_WAKEUP_SRC_EN_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&mut self) -> RO_PDS_WAKEUP_EVENT_W<24> {
        RO_PDS_WAKEUP_EVENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_INT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_int](index.html) module"]
pub struct PDS_INT_SPEC;
impl crate::RegisterSpec for PDS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_int::R](R) reader structure"]
impl crate::Readable for PDS_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_int::W](W) writer structure"]
impl crate::Writable for PDS_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_INT to value 0"]
impl crate::Resettable for PDS_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
