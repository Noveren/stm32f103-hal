#[doc = "Register `DCOUNT` reader"]
pub type R = crate::R<DCOUNT_SPEC>;
#[doc = "Field `DATACOUNT` reader - Data count value"]
pub type DATACOUNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value"]
    #[inline(always)]
    pub fn datacount(&self) -> DATACOUNT_R {
        DATACOUNT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcount::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCOUNT_SPEC;
impl crate::RegisterSpec for DCOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcount::R`](R) reader structure"]
impl crate::Readable for DCOUNT_SPEC {}
#[doc = "`reset()` method sets DCOUNT to value 0"]
impl crate::Resettable for DCOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
