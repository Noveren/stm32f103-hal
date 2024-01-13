#[doc = "Register `MMCTGFCR` reader"]
pub type R = crate::R<MMCTGFCR_SPEC>;
#[doc = "Field `TGFC` reader - Transmitted good frames counter"]
pub type TGFC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted good frames counter"]
    #[inline(always)]
    pub fn tgfc(&self) -> TGFC_R {
        TGFC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctgfcr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTGFCR_SPEC;
impl crate::RegisterSpec for MMCTGFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctgfcr::R`](R) reader structure"]
impl crate::Readable for MMCTGFCR_SPEC {}
#[doc = "`reset()` method sets MMCTGFCR to value 0"]
impl crate::Resettable for MMCTGFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
