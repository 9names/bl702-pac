#[doc = "Register `dvp2ahb_mem_bcnt_1` reader"]
pub struct R(crate::R<DVP2AHB_MEM_BCNT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP2AHB_MEM_BCNT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP2AHB_MEM_BCNT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP2AHB_MEM_BCNT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp2ahb_mem_bcnt_1` writer"]
pub struct W(crate::W<DVP2AHB_MEM_BCNT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP2AHB_MEM_BCNT_1_SPEC>;
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
impl From<crate::W<DVP2AHB_MEM_BCNT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP2AHB_MEM_BCNT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mem_burst_cnt_1` reader - "]
pub struct REG_MEM_BURST_CNT_1_R(crate::FieldReader<u32, u32>);
impl REG_MEM_BURST_CNT_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_MEM_BURST_CNT_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MEM_BURST_CNT_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mem_burst_cnt_1` writer - "]
pub struct REG_MEM_BURST_CNT_1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MEM_BURST_CNT_1_W<'a> {
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
    pub fn reg_mem_burst_cnt_1(&self) -> REG_MEM_BURST_CNT_1_R {
        REG_MEM_BURST_CNT_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_mem_burst_cnt_1(&mut self) -> REG_MEM_BURST_CNT_1_W {
        REG_MEM_BURST_CNT_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp2ahb_mem_bcnt_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp2ahb_mem_bcnt_1](index.html) module"]
pub struct DVP2AHB_MEM_BCNT_1_SPEC;
impl crate::RegisterSpec for DVP2AHB_MEM_BCNT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp2ahb_mem_bcnt_1::R](R) reader structure"]
impl crate::Readable for DVP2AHB_MEM_BCNT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp2ahb_mem_bcnt_1::W](W) writer structure"]
impl crate::Writable for DVP2AHB_MEM_BCNT_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp2ahb_mem_bcnt_1 to value 0"]
impl crate::Resettable for DVP2AHB_MEM_BCNT_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
