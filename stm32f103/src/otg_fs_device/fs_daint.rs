#[doc = "Register `FS_DAINT` reader"]
pub type R = crate::R<FS_DAINT_SPEC>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OEPINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_DAINT_SPEC;
impl crate::RegisterSpec for FS_DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_daint::R`](R) reader structure"]
impl crate::Readable for FS_DAINT_SPEC {}
#[doc = "`reset()` method sets FS_DAINT to value 0"]
impl crate::Resettable for FS_DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
