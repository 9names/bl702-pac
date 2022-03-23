#[doc = "Register `dvp_frame_fifo_pop` reader"]
pub struct R(crate::R<DVP_FRAME_FIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_FRAME_FIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_FRAME_FIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_FRAME_FIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp_frame_fifo_pop` writer"]
pub struct W(crate::W<DVP_FRAME_FIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_FRAME_FIFO_POP_SPEC>;
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
impl From<crate::W<DVP_FRAME_FIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_FRAME_FIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_int_fifo_clr_1` reader - "]
pub struct REG_INT_FIFO_CLR_1_R(crate::FieldReader<bool, bool>);
impl REG_INT_FIFO_CLR_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FIFO_CLR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FIFO_CLR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_fifo_clr_1` writer - "]
pub struct REG_INT_FIFO_CLR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FIFO_CLR_1_W<'a> {
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
#[doc = "Field `reg_int_frame_clr_1` reader - "]
pub struct REG_INT_FRAME_CLR_1_R(crate::FieldReader<bool, bool>);
impl REG_INT_FRAME_CLR_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FRAME_CLR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FRAME_CLR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_frame_clr_1` writer - "]
pub struct REG_INT_FRAME_CLR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FRAME_CLR_1_W<'a> {
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
#[doc = "Field `reg_int_mem_clr_1` reader - "]
pub struct REG_INT_MEM_CLR_1_R(crate::FieldReader<bool, bool>);
impl REG_INT_MEM_CLR_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_MEM_CLR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_MEM_CLR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_mem_clr_1` writer - "]
pub struct REG_INT_MEM_CLR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_MEM_CLR_1_W<'a> {
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
#[doc = "Field `reg_int_normal_clr_1` reader - "]
pub struct REG_INT_NORMAL_CLR_1_R(crate::FieldReader<bool, bool>);
impl REG_INT_NORMAL_CLR_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_NORMAL_CLR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_NORMAL_CLR_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_normal_clr_1` writer - "]
pub struct REG_INT_NORMAL_CLR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_NORMAL_CLR_1_W<'a> {
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
#[doc = "Field `reg_int_vcnt_clr_0` reader - "]
pub struct REG_INT_VCNT_CLR_0_R(crate::FieldReader<bool, bool>);
impl REG_INT_VCNT_CLR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_VCNT_CLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_VCNT_CLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_vcnt_clr_0` writer - "]
pub struct REG_INT_VCNT_CLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_VCNT_CLR_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `reg_int_hcnt_clr_0` reader - "]
pub struct REG_INT_HCNT_CLR_0_R(crate::FieldReader<bool, bool>);
impl REG_INT_HCNT_CLR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_HCNT_CLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_HCNT_CLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_hcnt_clr_0` writer - "]
pub struct REG_INT_HCNT_CLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_HCNT_CLR_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `reg_int_fifo_clr_0` reader - "]
pub struct REG_INT_FIFO_CLR_0_R(crate::FieldReader<bool, bool>);
impl REG_INT_FIFO_CLR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FIFO_CLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FIFO_CLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_fifo_clr_0` writer - "]
pub struct REG_INT_FIFO_CLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FIFO_CLR_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `reg_int_frame_clr_0` reader - "]
pub struct REG_INT_FRAME_CLR_0_R(crate::FieldReader<bool, bool>);
impl REG_INT_FRAME_CLR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FRAME_CLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FRAME_CLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_frame_clr_0` writer - "]
pub struct REG_INT_FRAME_CLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FRAME_CLR_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `reg_int_mem_clr_0` reader - "]
pub struct REG_INT_MEM_CLR_0_R(crate::FieldReader<bool, bool>);
impl REG_INT_MEM_CLR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_MEM_CLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_MEM_CLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_mem_clr_0` writer - "]
pub struct REG_INT_MEM_CLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_MEM_CLR_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `reg_int_normal_clr_0` reader - "]
pub struct REG_INT_NORMAL_CLR_0_R(crate::FieldReader<bool, bool>);
impl REG_INT_NORMAL_CLR_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_NORMAL_CLR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_NORMAL_CLR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_normal_clr_0` writer - "]
pub struct REG_INT_NORMAL_CLR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_NORMAL_CLR_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `rfifo_pop_1` reader - "]
pub struct RFIFO_POP_1_R(crate::FieldReader<bool, bool>);
impl RFIFO_POP_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFIFO_POP_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIFO_POP_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfifo_pop_1` writer - "]
pub struct RFIFO_POP_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIFO_POP_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `rfifo_pop_0` reader - "]
pub struct RFIFO_POP_0_R(crate::FieldReader<bool, bool>);
impl RFIFO_POP_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFIFO_POP_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIFO_POP_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfifo_pop_0` writer - "]
pub struct RFIFO_POP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIFO_POP_0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_int_fifo_clr_1(&self) -> REG_INT_FIFO_CLR_1_R {
        REG_INT_FIFO_CLR_1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_int_frame_clr_1(&self) -> REG_INT_FRAME_CLR_1_R {
        REG_INT_FRAME_CLR_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_int_mem_clr_1(&self) -> REG_INT_MEM_CLR_1_R {
        REG_INT_MEM_CLR_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_int_normal_clr_1(&self) -> REG_INT_NORMAL_CLR_1_R {
        REG_INT_NORMAL_CLR_1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_int_vcnt_clr_0(&self) -> REG_INT_VCNT_CLR_0_R {
        REG_INT_VCNT_CLR_0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_int_hcnt_clr_0(&self) -> REG_INT_HCNT_CLR_0_R {
        REG_INT_HCNT_CLR_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_int_fifo_clr_0(&self) -> REG_INT_FIFO_CLR_0_R {
        REG_INT_FIFO_CLR_0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_int_frame_clr_0(&self) -> REG_INT_FRAME_CLR_0_R {
        REG_INT_FRAME_CLR_0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_int_mem_clr_0(&self) -> REG_INT_MEM_CLR_0_R {
        REG_INT_MEM_CLR_0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_int_normal_clr_0(&self) -> REG_INT_NORMAL_CLR_0_R {
        REG_INT_NORMAL_CLR_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfifo_pop_1(&self) -> RFIFO_POP_1_R {
        RFIFO_POP_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfifo_pop_0(&self) -> RFIFO_POP_0_R {
        RFIFO_POP_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_int_fifo_clr_1(&mut self) -> REG_INT_FIFO_CLR_1_W {
        REG_INT_FIFO_CLR_1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_int_frame_clr_1(&mut self) -> REG_INT_FRAME_CLR_1_W {
        REG_INT_FRAME_CLR_1_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_int_mem_clr_1(&mut self) -> REG_INT_MEM_CLR_1_W {
        REG_INT_MEM_CLR_1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_int_normal_clr_1(&mut self) -> REG_INT_NORMAL_CLR_1_W {
        REG_INT_NORMAL_CLR_1_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_int_vcnt_clr_0(&mut self) -> REG_INT_VCNT_CLR_0_W {
        REG_INT_VCNT_CLR_0_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_int_hcnt_clr_0(&mut self) -> REG_INT_HCNT_CLR_0_W {
        REG_INT_HCNT_CLR_0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_int_fifo_clr_0(&mut self) -> REG_INT_FIFO_CLR_0_W {
        REG_INT_FIFO_CLR_0_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_int_frame_clr_0(&mut self) -> REG_INT_FRAME_CLR_0_W {
        REG_INT_FRAME_CLR_0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_int_mem_clr_0(&mut self) -> REG_INT_MEM_CLR_0_W {
        REG_INT_MEM_CLR_0_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_int_normal_clr_0(&mut self) -> REG_INT_NORMAL_CLR_0_W {
        REG_INT_NORMAL_CLR_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfifo_pop_1(&mut self) -> RFIFO_POP_1_W {
        RFIFO_POP_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfifo_pop_0(&mut self) -> RFIFO_POP_0_W {
        RFIFO_POP_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp_frame_fifo_pop.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_frame_fifo_pop](index.html) module"]
pub struct DVP_FRAME_FIFO_POP_SPEC;
impl crate::RegisterSpec for DVP_FRAME_FIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp_frame_fifo_pop::R](R) reader structure"]
impl crate::Readable for DVP_FRAME_FIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_frame_fifo_pop::W](W) writer structure"]
impl crate::Writable for DVP_FRAME_FIFO_POP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp_frame_fifo_pop to value 0"]
impl crate::Resettable for DVP_FRAME_FIFO_POP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
