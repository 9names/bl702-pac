#[doc = "Register `mjpeg_UV_frame_write_status` reader"]
pub struct R(crate::R<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_UV_frame_write_status` writer"]
pub struct W(crate::W<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>;
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
impl From<crate::W<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uv_mem_hblk_w` reader - "]
pub type UV_MEM_HBLK_W_R = crate::FieldReader<u16, u16>;
#[doc = "Field `uv_mem_hblk_w` writer - "]
pub type UV_MEM_HBLK_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_UV_FRAME_WRITE_STATUS_SPEC, u16, u16, 13, O>;
#[doc = "Field `uv_mem_rnd_w` reader - "]
pub type UV_MEM_RND_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uv_mem_rnd_w` writer - "]
pub type UV_MEM_RND_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_UV_FRAME_WRITE_STATUS_SPEC, u8, u8, 8, O>;
#[doc = "Field `uv_frm_cnt_w` reader - "]
pub type UV_FRM_CNT_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `uv_frm_cnt_w` writer - "]
pub type UV_FRM_CNT_W_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_UV_FRAME_WRITE_STATUS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn uv_mem_hblk_w(&self) -> UV_MEM_HBLK_W_R {
        UV_MEM_HBLK_W_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn uv_mem_rnd_w(&self) -> UV_MEM_RND_W_R {
        UV_MEM_RND_W_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn uv_frm_cnt_w(&self) -> UV_FRM_CNT_W_R {
        UV_FRM_CNT_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn uv_mem_hblk_w(&mut self) -> UV_MEM_HBLK_W_W<0> {
        UV_MEM_HBLK_W_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn uv_mem_rnd_w(&mut self) -> UV_MEM_RND_W_W<16> {
        UV_MEM_RND_W_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn uv_frm_cnt_w(&mut self) -> UV_FRM_CNT_W_W<24> {
        UV_FRM_CNT_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_UV_frame_write_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_uv_frame_write_status](index.html) module"]
pub struct MJPEG_UV_FRAME_WRITE_STATUS_SPEC;
impl crate::RegisterSpec for MJPEG_UV_FRAME_WRITE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_uv_frame_write_status::R](R) reader structure"]
impl crate::Readable for MJPEG_UV_FRAME_WRITE_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_uv_frame_write_status::W](W) writer structure"]
impl crate::Writable for MJPEG_UV_FRAME_WRITE_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_UV_frame_write_status to value 0"]
impl crate::Resettable for MJPEG_UV_FRAME_WRITE_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
