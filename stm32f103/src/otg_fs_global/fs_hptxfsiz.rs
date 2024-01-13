#[doc = "Register `FS_HPTXFSIZ` reader"]
pub type R = crate::R<FS_HPTXFSIZ_SPEC>;
#[doc = "Register `FS_HPTXFSIZ` writer"]
pub type W = crate::W<FS_HPTXFSIZ_SPEC>;
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PTXFSIZ` reader - Host periodic TxFIFO depth"]
pub type PTXFSIZ_R = crate::FieldReader<u16>;
#[doc = "Field `PTXFSIZ` writer - Host periodic TxFIFO depth"]
pub type PTXFSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxsa(&mut self) -> PTXSA_W<FS_HPTXFSIZ_SPEC> {
        PTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W<FS_HPTXFSIZ_SPEC> {
        PTXFSIZ_W::new(self, 16)
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
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hptxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hptxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HPTXFSIZ_SPEC;
impl crate::RegisterSpec for FS_HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hptxfsiz::R`](R) reader structure"]
impl crate::Readable for FS_HPTXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_hptxfsiz::W`](W) writer structure"]
impl crate::Writable for FS_HPTXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for FS_HPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0600;
}
