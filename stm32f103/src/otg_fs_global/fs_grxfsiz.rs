#[doc = "Register `FS_GRXFSIZ` reader"]
pub type R = crate::R<FS_GRXFSIZ_SPEC>;
#[doc = "Register `FS_GRXFSIZ` writer"]
pub type W = crate::W<FS_GRXFSIZ_SPEC>;
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RXFD_R = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<FS_GRXFSIZ_SPEC> {
        RXFD_W::new(self, 0)
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
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_grxfsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_grxfsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GRXFSIZ_SPEC;
impl crate::RegisterSpec for FS_GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_grxfsiz::R`](R) reader structure"]
impl crate::Readable for FS_GRXFSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_grxfsiz::W`](W) writer structure"]
impl crate::Writable for FS_GRXFSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_GRXFSIZ to value 0x0200"]
impl crate::Resettable for FS_GRXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
