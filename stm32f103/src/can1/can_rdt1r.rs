#[doc = "Register `CAN_RDT1R` reader"]
pub type R = crate::R<CAN_RDT1R_SPEC>;
#[doc = "Field `DLC` reader - DLC"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `FMI` reader - FMI"]
pub type FMI_R = crate::FieldReader;
#[doc = "Field `TIME` reader - TIME"]
pub type TIME_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - FMI"]
    #[inline(always)]
    pub fn fmi(&self) -> FMI_R {
        FMI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "CAN_RDT1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdt1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_RDT1R_SPEC;
impl crate::RegisterSpec for CAN_RDT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rdt1r::R`](R) reader structure"]
impl crate::Readable for CAN_RDT1R_SPEC {}
#[doc = "`reset()` method sets CAN_RDT1R to value 0"]
impl crate::Resettable for CAN_RDT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
