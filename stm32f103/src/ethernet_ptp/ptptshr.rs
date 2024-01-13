#[doc = "Register `PTPTSHR` reader"]
pub type R = crate::R<PTPTSHR_SPEC>;
#[doc = "Field `STS` reader - System time second"]
pub type STS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - System time second"]
    #[inline(always)]
    pub fn sts(&self) -> STS_R {
        STS_R::new(self.bits)
    }
}
#[doc = "Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSHR_SPEC;
impl crate::RegisterSpec for PTPTSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshr::R`](R) reader structure"]
impl crate::Readable for PTPTSHR_SPEC {}
#[doc = "`reset()` method sets PTPTSHR to value 0"]
impl crate::Resettable for PTPTSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
