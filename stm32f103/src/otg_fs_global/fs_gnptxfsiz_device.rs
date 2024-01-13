#[doc = "Register `FS_GNPTXFSIZ_Device` reader"]
pub type R = crate::R<FS_GNPTXFSIZ_DEVICE_SPEC>;
#[doc = "Register `FS_GNPTXFSIZ_Device` writer"]
pub type W = crate::W<FS_GNPTXFSIZ_DEVICE_SPEC>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<FS_GNPTXFSIZ_DEVICE_SPEC> {
        TX0FSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<FS_GNPTXFSIZ_DEVICE_SPEC> {
        TX0FD_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gnptxfsiz_device::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gnptxfsiz_device::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GNPTXFSIZ_DEVICE_SPEC;
impl crate::RegisterSpec for FS_GNPTXFSIZ_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gnptxfsiz_device::R`](R) reader structure"]
impl crate::Readable for FS_GNPTXFSIZ_DEVICE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_gnptxfsiz_device::W`](W) writer structure"]
impl crate::Writable for FS_GNPTXFSIZ_DEVICE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_GNPTXFSIZ_Device to value 0x0200"]
impl crate::Resettable for FS_GNPTXFSIZ_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
