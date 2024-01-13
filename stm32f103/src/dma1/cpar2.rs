#[doc = "Register `CPAR2` reader"]
pub type R = crate::R<CPAR2_SPEC>;
#[doc = "Register `CPAR2` writer"]
pub type W = crate::W<CPAR2_SPEC>;
#[doc = "Field `PA` reader - Peripheral address"]
pub type PA_R = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<CPAR2_SPEC> {
        PA_W::new(self, 0)
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
#[doc = "DMA channel 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPAR2_SPEC;
impl crate::RegisterSpec for CPAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar2::R`](R) reader structure"]
impl crate::Readable for CPAR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpar2::W`](W) writer structure"]
impl crate::Writable for CPAR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPAR2 to value 0"]
impl crate::Resettable for CPAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
