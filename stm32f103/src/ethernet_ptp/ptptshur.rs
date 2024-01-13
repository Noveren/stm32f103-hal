#[doc = "Register `PTPTSHUR` reader"]
pub type R = crate::R<PTPTSHUR_SPEC>;
#[doc = "Register `PTPTSHUR` writer"]
pub type W = crate::W<PTPTSHUR_SPEC>;
#[doc = "Field `TSUS` reader - Time stamp update second"]
pub type TSUS_R = crate::FieldReader<u32>;
#[doc = "Field `TSUS` writer - Time stamp update second"]
pub type TSUS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    pub fn tsus(&self) -> TSUS_R {
        TSUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp update second"]
    #[inline(always)]
    #[must_use]
    pub fn tsus(&mut self) -> TSUS_W<PTPTSHUR_SPEC> {
        TSUS_W::new(self, 0)
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
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPTSHUR_SPEC;
impl crate::RegisterSpec for PTPTSHUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshur::R`](R) reader structure"]
impl crate::Readable for PTPTSHUR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptptshur::W`](W) writer structure"]
impl crate::Writable for PTPTSHUR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPTSHUR to value 0"]
impl crate::Resettable for PTPTSHUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
