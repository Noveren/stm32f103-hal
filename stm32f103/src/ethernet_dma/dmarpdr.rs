#[doc = "Register `DMARPDR` reader"]
pub type R = crate::R<DMARPDR_SPEC>;
#[doc = "Register `DMARPDR` writer"]
pub type W = crate::W<DMARPDR_SPEC>;
#[doc = "Field `RPD` reader - Receive poll demand"]
pub type RPD_R = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive poll demand"]
pub type RPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RPD_W<DMARPDR_SPEC> {
        RPD_W::new(self, 0)
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
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARPDR_SPEC;
impl crate::RegisterSpec for DMARPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarpdr::R`](R) reader structure"]
impl crate::Readable for DMARPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmarpdr::W`](W) writer structure"]
impl crate::Writable for DMARPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARPDR to value 0"]
impl crate::Resettable for DMARPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
