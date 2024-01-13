#[doc = "Register `MMCRFAECR` reader"]
pub type R = crate::R<MMCRFAECR_SPEC>;
#[doc = "Field `RFAEC` reader - Received frames with alignment error counter"]
pub type RFAEC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received frames with alignment error counter"]
    #[inline(always)]
    pub fn rfaec(&self) -> RFAEC_R {
        RFAEC_R::new(self.bits)
    }
}
#[doc = "Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfaecr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRFAECR_SPEC;
impl crate::RegisterSpec for MMCRFAECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrfaecr::R`](R) reader structure"]
impl crate::Readable for MMCRFAECR_SPEC {}
#[doc = "`reset()` method sets MMCRFAECR to value 0"]
impl crate::Resettable for MMCRFAECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
