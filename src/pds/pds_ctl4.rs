#[doc = "Register `PDS_CTL4` reader"]
pub struct R(crate::R<PDS_CTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_CTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_CTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_CTL4` writer"]
pub struct W(crate::W<PDS_CTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL4_SPEC>;
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
impl From<crate::W<PDS_CTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_CTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_np_pwr_off` reader - "]
pub type CR_PDS_NP_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_np_pwr_off` writer - "]
pub type CR_PDS_NP_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_np_reset` reader - "]
pub type CR_PDS_NP_RESET_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_np_reset` writer - "]
pub type CR_PDS_NP_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_np_mem_stby` reader - "]
pub type CR_PDS_NP_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_np_mem_stby` writer - "]
pub type CR_PDS_NP_MEM_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_np_gate_clk` reader - "]
pub type CR_PDS_NP_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_np_gate_clk` writer - "]
pub type CR_PDS_NP_GATE_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_bz_pwr_off` reader - "]
pub type CR_PDS_BZ_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_bz_pwr_off` writer - "]
pub type CR_PDS_BZ_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_bz_reset` reader - "]
pub type CR_PDS_BZ_RESET_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_bz_reset` writer - "]
pub type CR_PDS_BZ_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_bz_mem_stby` reader - "]
pub type CR_PDS_BZ_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_bz_mem_stby` writer - "]
pub type CR_PDS_BZ_MEM_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_bz_gate_clk` reader - "]
pub type CR_PDS_BZ_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_bz_gate_clk` writer - "]
pub type CR_PDS_BZ_GATE_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_ble_pwr_off` reader - "]
pub type CR_PDS_BLE_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ble_pwr_off` writer - "]
pub type CR_PDS_BLE_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_ble_reset` reader - "]
pub type CR_PDS_BLE_RESET_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ble_reset` writer - "]
pub type CR_PDS_BLE_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_ble_mem_stby` reader - "]
pub type CR_PDS_BLE_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ble_mem_stby` writer - "]
pub type CR_PDS_BLE_MEM_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_ble_gate_clk` reader - "]
pub type CR_PDS_BLE_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_ble_gate_clk` writer - "]
pub type CR_PDS_BLE_GATE_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_usb_pwr_off` reader - "]
pub type CR_PDS_USB_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_usb_pwr_off` writer - "]
pub type CR_PDS_USB_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_usb_reset` reader - "]
pub type CR_PDS_USB_RESET_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_usb_reset` writer - "]
pub type CR_PDS_USB_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_usb_mem_stby` reader - "]
pub type CR_PDS_USB_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_usb_mem_stby` writer - "]
pub type CR_PDS_USB_MEM_STBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_usb_gate_clk` reader - "]
pub type CR_PDS_USB_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_usb_gate_clk` writer - "]
pub type CR_PDS_USB_GATE_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_pwr_off` reader - "]
pub type CR_PDS_MISC_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_pwr_off` writer - "]
pub type CR_PDS_MISC_PWR_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_reset` reader - "]
pub type CR_PDS_MISC_RESET_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_reset` writer - "]
pub type CR_PDS_MISC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_mem_stby` reader - "]
pub type CR_PDS_MISC_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_mem_stby` writer - "]
pub type CR_PDS_MISC_MEM_STBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_gate_clk` reader - "]
pub type CR_PDS_MISC_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_gate_clk` writer - "]
pub type CR_PDS_MISC_GATE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_ana_pwr_off` reader - "]
pub type CR_PDS_MISC_ANA_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_ana_pwr_off` writer - "]
pub type CR_PDS_MISC_ANA_PWR_OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
#[doc = "Field `cr_pds_misc_dig_pwr_off` reader - "]
pub type CR_PDS_MISC_DIG_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_misc_dig_pwr_off` writer - "]
pub type CR_PDS_MISC_DIG_PWR_OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&self) -> CR_PDS_NP_PWR_OFF_R {
        CR_PDS_NP_PWR_OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&self) -> CR_PDS_NP_RESET_R {
        CR_PDS_NP_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&self) -> CR_PDS_NP_MEM_STBY_R {
        CR_PDS_NP_MEM_STBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&self) -> CR_PDS_NP_GATE_CLK_R {
        CR_PDS_NP_GATE_CLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_bz_pwr_off(&self) -> CR_PDS_BZ_PWR_OFF_R {
        CR_PDS_BZ_PWR_OFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_bz_reset(&self) -> CR_PDS_BZ_RESET_R {
        CR_PDS_BZ_RESET_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_bz_mem_stby(&self) -> CR_PDS_BZ_MEM_STBY_R {
        CR_PDS_BZ_MEM_STBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_bz_gate_clk(&self) -> CR_PDS_BZ_GATE_CLK_R {
        CR_PDS_BZ_GATE_CLK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_ble_pwr_off(&self) -> CR_PDS_BLE_PWR_OFF_R {
        CR_PDS_BLE_PWR_OFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_ble_reset(&self) -> CR_PDS_BLE_RESET_R {
        CR_PDS_BLE_RESET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ble_mem_stby(&self) -> CR_PDS_BLE_MEM_STBY_R {
        CR_PDS_BLE_MEM_STBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_ble_gate_clk(&self) -> CR_PDS_BLE_GATE_CLK_R {
        CR_PDS_BLE_GATE_CLK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_usb_pwr_off(&self) -> CR_PDS_USB_PWR_OFF_R {
        CR_PDS_USB_PWR_OFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_usb_reset(&self) -> CR_PDS_USB_RESET_R {
        CR_PDS_USB_RESET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_usb_mem_stby(&self) -> CR_PDS_USB_MEM_STBY_R {
        CR_PDS_USB_MEM_STBY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_pds_usb_gate_clk(&self) -> CR_PDS_USB_GATE_CLK_R {
        CR_PDS_USB_GATE_CLK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&self) -> CR_PDS_MISC_PWR_OFF_R {
        CR_PDS_MISC_PWR_OFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&self) -> CR_PDS_MISC_RESET_R {
        CR_PDS_MISC_RESET_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&self) -> CR_PDS_MISC_MEM_STBY_R {
        CR_PDS_MISC_MEM_STBY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&self) -> CR_PDS_MISC_GATE_CLK_R {
        CR_PDS_MISC_GATE_CLK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_ana_pwr_off(&self) -> CR_PDS_MISC_ANA_PWR_OFF_R {
        CR_PDS_MISC_ANA_PWR_OFF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_misc_dig_pwr_off(&self) -> CR_PDS_MISC_DIG_PWR_OFF_R {
        CR_PDS_MISC_DIG_PWR_OFF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&mut self) -> CR_PDS_NP_PWR_OFF_W<0> {
        CR_PDS_NP_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&mut self) -> CR_PDS_NP_RESET_W<1> {
        CR_PDS_NP_RESET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&mut self) -> CR_PDS_NP_MEM_STBY_W<2> {
        CR_PDS_NP_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&mut self) -> CR_PDS_NP_GATE_CLK_W<3> {
        CR_PDS_NP_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_bz_pwr_off(&mut self) -> CR_PDS_BZ_PWR_OFF_W<12> {
        CR_PDS_BZ_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_bz_reset(&mut self) -> CR_PDS_BZ_RESET_W<13> {
        CR_PDS_BZ_RESET_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_bz_mem_stby(&mut self) -> CR_PDS_BZ_MEM_STBY_W<14> {
        CR_PDS_BZ_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_bz_gate_clk(&mut self) -> CR_PDS_BZ_GATE_CLK_W<15> {
        CR_PDS_BZ_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_ble_pwr_off(&mut self) -> CR_PDS_BLE_PWR_OFF_W<16> {
        CR_PDS_BLE_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_ble_reset(&mut self) -> CR_PDS_BLE_RESET_W<17> {
        CR_PDS_BLE_RESET_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_ble_mem_stby(&mut self) -> CR_PDS_BLE_MEM_STBY_W<18> {
        CR_PDS_BLE_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_ble_gate_clk(&mut self) -> CR_PDS_BLE_GATE_CLK_W<19> {
        CR_PDS_BLE_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_pds_usb_pwr_off(&mut self) -> CR_PDS_USB_PWR_OFF_W<20> {
        CR_PDS_USB_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_pds_usb_reset(&mut self) -> CR_PDS_USB_RESET_W<21> {
        CR_PDS_USB_RESET_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_pds_usb_mem_stby(&mut self) -> CR_PDS_USB_MEM_STBY_W<22> {
        CR_PDS_USB_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_pds_usb_gate_clk(&mut self) -> CR_PDS_USB_GATE_CLK_W<23> {
        CR_PDS_USB_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&mut self) -> CR_PDS_MISC_PWR_OFF_W<24> {
        CR_PDS_MISC_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&mut self) -> CR_PDS_MISC_RESET_W<25> {
        CR_PDS_MISC_RESET_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&mut self) -> CR_PDS_MISC_MEM_STBY_W<26> {
        CR_PDS_MISC_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&mut self) -> CR_PDS_MISC_GATE_CLK_W<27> {
        CR_PDS_MISC_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_pds_misc_ana_pwr_off(&mut self) -> CR_PDS_MISC_ANA_PWR_OFF_W<30> {
        CR_PDS_MISC_ANA_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_pds_misc_dig_pwr_off(&mut self) -> CR_PDS_MISC_DIG_PWR_OFF_W<31> {
        CR_PDS_MISC_DIG_PWR_OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl4](index.html) module"]
pub struct PDS_CTL4_SPEC;
impl crate::RegisterSpec for PDS_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl4::R](R) reader structure"]
impl crate::Readable for PDS_CTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl4::W](W) writer structure"]
impl crate::Writable for PDS_CTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_CTL4 to value 0"]
impl crate::Resettable for PDS_CTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
