#[doc = "Register `CAN_RDL1R` reader"]
pub type R = crate::R<CAN_RDL1R_SPEC>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA1` reader - DATA1"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA2` reader - DATA2"]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA3` reader - DATA3"]
pub type DATA3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CAN_RDL1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdl1r::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_RDL1R_SPEC;
impl crate::RegisterSpec for CAN_RDL1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rdl1r::R`](R) reader structure"]
impl crate::Readable for CAN_RDL1R_SPEC {}
#[doc = "`reset()` method sets CAN_RDL1R to value 0"]
impl crate::Resettable for CAN_RDL1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
