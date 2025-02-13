#[doc = "Register `rc32k_ctrl0` reader"]
pub struct R(crate::R<RC32K_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32K_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32K_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32K_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rc32k_ctrl0` writer"]
pub struct W(crate::W<RC32K_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32K_CTRL0_SPEC>;
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
impl From<crate::W<RC32K_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32K_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rc32k_cal_done` reader - "]
pub type RC32K_CAL_DONE_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_cal_done` writer - "]
pub type RC32K_CAL_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_rdy` reader - "]
pub type RC32K_RDY_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_rdy` writer - "]
pub type RC32K_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_cal_inprogress` reader - "]
pub type RC32K_CAL_INPROGRESS_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_cal_inprogress` writer - "]
pub type RC32K_CAL_INPROGRESS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_cal_div` reader - "]
pub type RC32K_CAL_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rc32k_cal_div` writer - "]
pub type RC32K_CAL_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RC32K_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `rc32k_cal_precharge` reader - "]
pub type RC32K_CAL_PRECHARGE_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_cal_precharge` writer - "]
pub type RC32K_CAL_PRECHARGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_dig_code_fr_cal` reader - "]
pub type RC32K_DIG_CODE_FR_CAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rc32k_dig_code_fr_cal` writer - "]
pub type RC32K_DIG_CODE_FR_CAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RC32K_CTRL0_SPEC, u16, u16, 10, O>;
#[doc = "Field `rc32k_vref_dly` reader - "]
pub type RC32K_VREF_DLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rc32k_vref_dly` writer - "]
pub type RC32K_VREF_DLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RC32K_CTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `rc32k_allow_cal` reader - "]
pub type RC32K_ALLOW_CAL_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_allow_cal` writer - "]
pub type RC32K_ALLOW_CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_ext_code_en` reader - "]
pub type RC32K_EXT_CODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_ext_code_en` writer - "]
pub type RC32K_EXT_CODE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_cal_en` reader - "]
pub type RC32K_CAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rc32k_cal_en` writer - "]
pub type RC32K_CAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32K_CTRL0_SPEC, bool, O>;
#[doc = "Field `rc32k_code_fr_ext` reader - "]
pub type RC32K_CODE_FR_EXT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rc32k_code_fr_ext` writer - "]
pub type RC32K_CODE_FR_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RC32K_CTRL0_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32k_cal_done(&self) -> RC32K_CAL_DONE_R {
        RC32K_CAL_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32k_rdy(&self) -> RC32K_RDY_R {
        RC32K_RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32k_cal_inprogress(&self) -> RC32K_CAL_INPROGRESS_R {
        RC32K_CAL_INPROGRESS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&self) -> RC32K_CAL_DIV_R {
        RC32K_CAL_DIV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_cal_precharge(&self) -> RC32K_CAL_PRECHARGE_R {
        RC32K_CAL_PRECHARGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rc32k_dig_code_fr_cal(&self) -> RC32K_DIG_CODE_FR_CAL_R {
        RC32K_DIG_CODE_FR_CAL_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&self) -> RC32K_VREF_DLY_R {
        RC32K_VREF_DLY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&self) -> RC32K_ALLOW_CAL_R {
        RC32K_ALLOW_CAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&self) -> RC32K_EXT_CODE_EN_R {
        RC32K_EXT_CODE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&self) -> RC32K_CAL_EN_R {
        RC32K_CAL_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&self) -> RC32K_CODE_FR_EXT_R {
        RC32K_CODE_FR_EXT_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rc32k_cal_done(&mut self) -> RC32K_CAL_DONE_W<0> {
        RC32K_CAL_DONE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rc32k_rdy(&mut self) -> RC32K_RDY_W<1> {
        RC32K_RDY_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rc32k_cal_inprogress(&mut self) -> RC32K_CAL_INPROGRESS_W<2> {
        RC32K_CAL_INPROGRESS_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rc32k_cal_div(&mut self) -> RC32K_CAL_DIV_W<3> {
        RC32K_CAL_DIV_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rc32k_cal_precharge(&mut self) -> RC32K_CAL_PRECHARGE_W<5> {
        RC32K_CAL_PRECHARGE_W::new(self)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rc32k_dig_code_fr_cal(&mut self) -> RC32K_DIG_CODE_FR_CAL_W<6> {
        RC32K_DIG_CODE_FR_CAL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rc32k_vref_dly(&mut self) -> RC32K_VREF_DLY_W<16> {
        RC32K_VREF_DLY_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rc32k_allow_cal(&mut self) -> RC32K_ALLOW_CAL_W<18> {
        RC32K_ALLOW_CAL_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rc32k_ext_code_en(&mut self) -> RC32K_EXT_CODE_EN_W<19> {
        RC32K_EXT_CODE_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rc32k_cal_en(&mut self) -> RC32K_CAL_EN_W<20> {
        RC32K_CAL_EN_W::new(self)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rc32k_code_fr_ext(&mut self) -> RC32K_CODE_FR_EXT_W<22> {
        RC32K_CODE_FR_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rc32k_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32k_ctrl0](index.html) module"]
pub struct RC32K_CTRL0_SPEC;
impl crate::RegisterSpec for RC32K_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32k_ctrl0::R](R) reader structure"]
impl crate::Readable for RC32K_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32k_ctrl0::W](W) writer structure"]
impl crate::Writable for RC32K_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rc32k_ctrl0 to value 0"]
impl crate::Resettable for RC32K_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
