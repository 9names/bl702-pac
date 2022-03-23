#[doc = "Register `mjpeg_paket_head_tail` reader"]
pub struct R(crate::R<MJPEG_PAKET_HEAD_TAIL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_PAKET_HEAD_TAIL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_PAKET_HEAD_TAIL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_PAKET_HEAD_TAIL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_paket_head_tail` writer"]
pub struct W(crate::W<MJPEG_PAKET_HEAD_TAIL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_PAKET_HEAD_TAIL_SPEC>;
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
impl From<crate::W<MJPEG_PAKET_HEAD_TAIL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_PAKET_HEAD_TAIL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_pket_tail_byte` reader - "]
pub struct REG_PKET_TAIL_BYTE_R(crate::FieldReader<u16, u16>);
impl REG_PKET_TAIL_BYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_PKET_TAIL_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PKET_TAIL_BYTE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pket_tail_byte` writer - "]
pub struct REG_PKET_TAIL_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PKET_TAIL_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `reg_pket_head_byte` reader - "]
pub struct REG_PKET_HEAD_BYTE_R(crate::FieldReader<u16, u16>);
impl REG_PKET_HEAD_BYTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_PKET_HEAD_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PKET_HEAD_BYTE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pket_head_byte` writer - "]
pub struct REG_PKET_HEAD_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PKET_HEAD_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn reg_pket_tail_byte(&self) -> REG_PKET_TAIL_BYTE_R {
        REG_PKET_TAIL_BYTE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_pket_head_byte(&self) -> REG_PKET_HEAD_BYTE_R {
        REG_PKET_HEAD_BYTE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn reg_pket_tail_byte(&mut self) -> REG_PKET_TAIL_BYTE_W {
        REG_PKET_TAIL_BYTE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_pket_head_byte(&mut self) -> REG_PKET_HEAD_BYTE_W {
        REG_PKET_HEAD_BYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_paket_head_tail.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_paket_head_tail](index.html) module"]
pub struct MJPEG_PAKET_HEAD_TAIL_SPEC;
impl crate::RegisterSpec for MJPEG_PAKET_HEAD_TAIL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_paket_head_tail::R](R) reader structure"]
impl crate::Readable for MJPEG_PAKET_HEAD_TAIL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_paket_head_tail::W](W) writer structure"]
impl crate::Writable for MJPEG_PAKET_HEAD_TAIL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_paket_head_tail to value 0"]
impl crate::Resettable for MJPEG_PAKET_HEAD_TAIL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
