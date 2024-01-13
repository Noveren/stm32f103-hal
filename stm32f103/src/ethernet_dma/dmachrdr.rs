#[doc = "Register `DMACHRDR` reader"]
pub type R = crate::R<DMACHRDR_SPEC>;
#[doc = "Field `HRDAP` reader - Host receive descriptor address pointer"]
pub type HRDAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host receive descriptor address pointer"]
    #[inline(always)]
    pub fn hrdap(&self) -> HRDAP_R {
        HRDAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host receive descriptor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachrdr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACHRDR_SPEC;
impl crate::RegisterSpec for DMACHRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachrdr::R`](R) reader structure"]
impl crate::Readable for DMACHRDR_SPEC {}
#[doc = "`reset()` method sets DMACHRDR to value 0"]
impl crate::Resettable for DMACHRDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
