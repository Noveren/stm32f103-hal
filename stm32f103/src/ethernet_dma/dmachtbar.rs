#[doc = "Register `DMACHTBAR` reader"]
pub type R = crate::R<DMACHTBAR_SPEC>;
#[doc = "Field `HTBAP` reader - Host transmit buffer address pointer"]
pub type HTBAP_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host transmit buffer address pointer"]
    #[inline(always)]
    pub fn htbap(&self) -> HTBAP_R {
        HTBAP_R::new(self.bits)
    }
}
#[doc = "Ethernet DMA current host transmit buffer address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachtbar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACHTBAR_SPEC;
impl crate::RegisterSpec for DMACHTBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachtbar::R`](R) reader structure"]
impl crate::Readable for DMACHTBAR_SPEC {}
#[doc = "`reset()` method sets DMACHTBAR to value 0"]
impl crate::Resettable for DMACHTBAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
