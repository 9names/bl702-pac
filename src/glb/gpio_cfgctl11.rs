#[doc = "Register `GPIO_CFGCTL11` reader"]
pub struct R(crate::R<GPIO_CFGCTL11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL11` writer"]
pub struct W(crate::W<GPIO_CFGCTL11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL11_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_23_func_sel` reader - "]
pub struct REG_GPIO_23_FUNC_SEL_R(crate::FieldReader<u8, u8>);
impl REG_GPIO_23_FUNC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_23_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_23_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_func_sel` writer - "]
pub struct REG_GPIO_23_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_FUNC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `reg_gpio_23_pd` reader - "]
pub struct REG_GPIO_23_PD_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_23_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_23_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_pd` writer - "]
pub struct REG_GPIO_23_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `reg_gpio_23_pu` reader - "]
pub struct REG_GPIO_23_PU_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_23_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_23_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_pu` writer - "]
pub struct REG_GPIO_23_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `reg_gpio_23_drv` reader - "]
pub struct REG_GPIO_23_DRV_R(crate::FieldReader<u8, u8>);
impl REG_GPIO_23_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_23_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_23_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_drv` writer - "]
pub struct REG_GPIO_23_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `reg_gpio_23_smt` reader - "]
pub struct REG_GPIO_23_SMT_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_23_SMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_SMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_23_SMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_smt` writer - "]
pub struct REG_GPIO_23_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_SMT_W<'a> {
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
#[doc = "Field `reg_gpio_23_ie` reader - "]
pub struct REG_GPIO_23_IE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_23_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_23_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_ie` writer - "]
pub struct REG_GPIO_23_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_IE_W<'a> {
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
#[doc = "Field `reg_gpio_22_func_sel` reader - "]
pub struct REG_GPIO_22_FUNC_SEL_R(crate::FieldReader<u8, u8>);
impl REG_GPIO_22_FUNC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_22_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_func_sel` writer - "]
pub struct REG_GPIO_22_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_FUNC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `reg_gpio_22_pd` reader - "]
pub struct REG_GPIO_22_PD_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_22_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_pd` writer - "]
pub struct REG_GPIO_22_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_PD_W<'a> {
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
#[doc = "Field `reg_gpio_22_pu` reader - "]
pub struct REG_GPIO_22_PU_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_22_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_pu` writer - "]
pub struct REG_GPIO_22_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_PU_W<'a> {
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
#[doc = "Field `reg_gpio_22_drv` reader - "]
pub struct REG_GPIO_22_DRV_R(crate::FieldReader<u8, u8>);
impl REG_GPIO_22_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_22_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_drv` writer - "]
pub struct REG_GPIO_22_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `reg_gpio_22_smt` reader - "]
pub struct REG_GPIO_22_SMT_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_22_SMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_SMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_SMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_smt` writer - "]
pub struct REG_GPIO_22_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_SMT_W<'a> {
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
#[doc = "Field `reg_gpio_22_ie` reader - "]
pub struct REG_GPIO_22_IE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_22_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_ie` writer - "]
pub struct REG_GPIO_22_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_IE_W<'a> {
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
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn reg_gpio_23_func_sel(&self) -> REG_GPIO_23_FUNC_SEL_R {
        REG_GPIO_23_FUNC_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&self) -> REG_GPIO_23_PD_R {
        REG_GPIO_23_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&self) -> REG_GPIO_23_PU_R {
        REG_GPIO_23_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&self) -> REG_GPIO_23_DRV_R {
        REG_GPIO_23_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&self) -> REG_GPIO_23_SMT_R {
        REG_GPIO_23_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&self) -> REG_GPIO_23_IE_R {
        REG_GPIO_23_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&self) -> REG_GPIO_22_FUNC_SEL_R {
        REG_GPIO_22_FUNC_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&self) -> REG_GPIO_22_PD_R {
        REG_GPIO_22_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&self) -> REG_GPIO_22_PU_R {
        REG_GPIO_22_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&self) -> REG_GPIO_22_DRV_R {
        REG_GPIO_22_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&self) -> REG_GPIO_22_SMT_R {
        REG_GPIO_22_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&self) -> REG_GPIO_22_IE_R {
        REG_GPIO_22_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn reg_gpio_23_func_sel(&mut self) -> REG_GPIO_23_FUNC_SEL_W {
        REG_GPIO_23_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_23_pd(&mut self) -> REG_GPIO_23_PD_W {
        REG_GPIO_23_PD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_23_pu(&mut self) -> REG_GPIO_23_PU_W {
        REG_GPIO_23_PU_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_23_drv(&mut self) -> REG_GPIO_23_DRV_W {
        REG_GPIO_23_DRV_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_23_smt(&mut self) -> REG_GPIO_23_SMT_W {
        REG_GPIO_23_SMT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_23_ie(&mut self) -> REG_GPIO_23_IE_W {
        REG_GPIO_23_IE_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_22_func_sel(&mut self) -> REG_GPIO_22_FUNC_SEL_W {
        REG_GPIO_22_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_22_pd(&mut self) -> REG_GPIO_22_PD_W {
        REG_GPIO_22_PD_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_22_pu(&mut self) -> REG_GPIO_22_PU_W {
        REG_GPIO_22_PU_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_22_drv(&mut self) -> REG_GPIO_22_DRV_W {
        REG_GPIO_22_DRV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_22_smt(&mut self) -> REG_GPIO_22_SMT_W {
        REG_GPIO_22_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_22_ie(&mut self) -> REG_GPIO_22_IE_W {
        REG_GPIO_22_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl11](index.html) module"]
pub struct GPIO_CFGCTL11_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl11::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl11::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL11 to value 0"]
impl crate::Resettable for GPIO_CFGCTL11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
