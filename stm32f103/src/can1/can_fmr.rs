#[doc = "Register `CAN_FMR` reader"]
pub type R = crate::R<CAN_FMR_SPEC>;
#[doc = "Register `CAN_FMR` writer"]
pub type W = crate::W<CAN_FMR_SPEC>;
#[doc = "Field `FINIT` reader - FINIT"]
pub type FINIT_R = crate::BitReader;
#[doc = "Field `FINIT` writer - FINIT"]
pub type FINIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    pub fn finit(&self) -> FINIT_R {
        FINIT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FINIT"]
    #[inline(always)]
    #[must_use]
    pub fn finit(&mut self) -> FINIT_W<CAN_FMR_SPEC> {
        FINIT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CAN_FMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_FMR_SPEC;
impl crate::RegisterSpec for CAN_FMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fmr::R`](R) reader structure"]
impl crate::Readable for CAN_FMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_fmr::W`](W) writer structure"]
impl crate::Writable for CAN_FMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_FMR to value 0"]
impl crate::Resettable for CAN_FMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
