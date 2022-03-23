#[doc = "Register `ep4_config` reader"]
pub struct R(crate::R<EP4_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP4_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP4_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP4_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep4_config` writer"]
pub struct W(crate::W<EP4_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP4_CONFIG_SPEC>;
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
impl From<crate::W<EP4_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP4_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_ep4_rdy` reader - "]
pub struct STS_EP4_RDY_R(crate::FieldReader<bool, bool>);
impl STS_EP4_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_EP4_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_EP4_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_ep4_rdy` writer - "]
pub struct STS_EP4_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_EP4_RDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `cr_ep4_rdy` reader - "]
pub struct CR_EP4_RDY_R(crate::FieldReader<bool, bool>);
impl CR_EP4_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP4_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_rdy` writer - "]
pub struct CR_EP4_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_RDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cr_ep4_nack` reader - "]
pub struct CR_EP4_NACK_R(crate::FieldReader<bool, bool>);
impl CR_EP4_NACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP4_NACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_NACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_nack` writer - "]
pub struct CR_EP4_NACK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_NACK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `cr_ep4_stall` reader - "]
pub struct CR_EP4_STALL_R(crate::FieldReader<bool, bool>);
impl CR_EP4_STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP4_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_stall` writer - "]
pub struct CR_EP4_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_STALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cr_ep4_type` reader - "]
pub struct CR_EP4_TYPE_R(crate::FieldReader<u8, u8>);
impl CR_EP4_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_EP4_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_TYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_type` writer - "]
pub struct CR_EP4_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_TYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "Field `cr_ep4_dir` reader - "]
pub struct CR_EP4_DIR_R(crate::FieldReader<u8, u8>);
impl CR_EP4_DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_EP4_DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_dir` writer - "]
pub struct CR_EP4_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `cr_ep4_size` reader - "]
pub struct CR_EP4_SIZE_R(crate::FieldReader<u16, u16>);
impl CR_EP4_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CR_EP4_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_SIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_size` writer - "]
pub struct CR_EP4_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sts_ep4_rdy(&self) -> STS_EP4_RDY_R {
        STS_EP4_RDY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep4_rdy(&self) -> CR_EP4_RDY_R {
        CR_EP4_RDY_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep4_nack(&self) -> CR_EP4_NACK_R {
        CR_EP4_NACK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep4_stall(&self) -> CR_EP4_STALL_R {
        CR_EP4_STALL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn cr_ep4_type(&self) -> CR_EP4_TYPE_R {
        CR_EP4_TYPE_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn cr_ep4_dir(&self) -> CR_EP4_DIR_R {
        CR_EP4_DIR_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_ep4_size(&self) -> CR_EP4_SIZE_R {
        CR_EP4_SIZE_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sts_ep4_rdy(&mut self) -> STS_EP4_RDY_W {
        STS_EP4_RDY_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep4_rdy(&mut self) -> CR_EP4_RDY_W {
        CR_EP4_RDY_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep4_nack(&mut self) -> CR_EP4_NACK_W {
        CR_EP4_NACK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep4_stall(&mut self) -> CR_EP4_STALL_W {
        CR_EP4_STALL_W { w: self }
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn cr_ep4_type(&mut self) -> CR_EP4_TYPE_W {
        CR_EP4_TYPE_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn cr_ep4_dir(&mut self) -> CR_EP4_DIR_W {
        CR_EP4_DIR_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_ep4_size(&mut self) -> CR_EP4_SIZE_W {
        CR_EP4_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep4_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4_config](index.html) module"]
pub struct EP4_CONFIG_SPEC;
impl crate::RegisterSpec for EP4_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep4_config::R](R) reader structure"]
impl crate::Readable for EP4_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep4_config::W](W) writer structure"]
impl crate::Writable for EP4_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ep4_config to value 0"]
impl crate::Resettable for EP4_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
