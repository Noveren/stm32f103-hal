#[doc = "Register `CAN_RI1R` reader"]
pub type R = crate::R<CAN_RI1R_SPEC>;
#[doc = "Field `RTR` reader - RTR"]
pub type RTR_R = crate::BitReader;
#[doc = "Field `IDE` reader - IDE"]
pub type IDE_R = crate::BitReader;
#[doc = "Field `EXID` reader - EXID"]
pub type EXID_R = crate::FieldReader<u32>;
#[doc = "Field `STID` reader - STID"]
pub type STID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 1 - RTR"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IDE"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - EXID"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - STID"]
    #[inline(always)]
    pub fn stid(&self) -> STID_R {
        STID_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
#[doc = "CAN_RI1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ri1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_RI1R_SPEC;
impl crate::RegisterSpec for CAN_RI1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ri1r::R`](R) reader structure"]
impl crate::Readable for CAN_RI1R_SPEC {}
#[doc = "`reset()` method sets CAN_RI1R to value 0"]
impl crate::Resettable for CAN_RI1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
