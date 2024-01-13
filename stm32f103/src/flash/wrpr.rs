#[doc = "Register `WRPR` reader"]
pub type R = crate::R<WRPR_SPEC>;
#[doc = "Field `WRP` reader - Write protect"]
pub type WRP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write protect"]
    #[inline(always)]
    pub fn wrp(&self) -> WRP_R {
        WRP_R::new(self.bits)
    }
}
#[doc = "Write protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRPR_SPEC;
impl crate::RegisterSpec for WRPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr::R`](R) reader structure"]
impl crate::Readable for WRPR_SPEC {}
#[doc = "`reset()` method sets WRPR to value 0xffff_ffff"]
impl crate::Resettable for WRPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
