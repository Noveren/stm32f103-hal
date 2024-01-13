#[doc = "Register `PTPTTHR` reader"]
pub type R = crate::R<PTPTTHR_SPEC>;
#[doc = "Register `PTPTTHR` writer"]
pub type W = crate::W<PTPTTHR_SPEC>;
#[doc = "Field `TTSH` reader - Target time stamp high"]
pub type TTSH_R = crate::FieldReader<u32>;
#[doc = "Field `TTSH` writer - Target time stamp high"]
pub type TTSH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target time stamp high"]
    #[inline(always)]
    pub fn ttsh(&self) -> TTSH_R {
        TTSH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Target time stamp high"]
    #[inline(always)]
    #[must_use]
    pub fn ttsh(&mut self) -> TTSH_W<PTPTTHR_SPEC> {
        TTSH_W::new(self, 0)
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
#[doc = "Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptthr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptthr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTTHR_SPEC;
impl crate::RegisterSpec for PTPTTHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptthr::R`](R) reader structure"]
impl crate::Readable for PTPTTHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptthr::W`](W) writer structure"]
impl crate::Writable for PTPTTHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTTHR to value 0"]
impl crate::Resettable for PTPTTHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
