#[doc = "Register `RESPI1` reader"]
pub type R = crate::R<RESPI1_SPEC>;
#[doc = "Field `CARDSTATUS1` reader - CARDSTATUS1"]
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CARDSTATUS1"]
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
#[doc = "Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`respi1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESPI1_SPEC;
impl crate::RegisterSpec for RESPI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`respi1::R`](R) reader structure"]
impl crate::Readable for RESPI1_SPEC {}
#[doc = "`reset()` method sets RESPI1 to value 0"]
impl crate::Resettable for RESPI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
