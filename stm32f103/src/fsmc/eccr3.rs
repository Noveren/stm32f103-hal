#[doc = "Register `ECCR3` reader"]
pub type R = crate::R<ECCR3_SPEC>;
#[doc = "Field `ECCx` reader - ECCx"]
pub type ECCX_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ECCx"]
    #[inline(always)]
    pub fn eccx(&self) -> ECCX_R {
        ECCX_R::new(self.bits)
    }
}
#[doc = "ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCR3_SPEC;
impl crate::RegisterSpec for ECCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr3::R`](R) reader structure"]
impl crate::Readable for ECCR3_SPEC {}
#[doc = "`reset()` method sets ECCR3 to value 0"]
impl crate::Resettable for ECCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
