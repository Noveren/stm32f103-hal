#[doc = "Register `CMAR3` reader"]
pub type R = crate::R<CMAR3_SPEC>;
#[doc = "Register `CMAR3` writer"]
pub type W = crate::W<CMAR3_SPEC>;
#[doc = "Field `MA` reader - Memory address"]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address"]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR3_SPEC> {
        MA_W::new(self, 0)
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
#[doc = "DMA channel 3 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMAR3_SPEC;
impl crate::RegisterSpec for CMAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar3::R`](R) reader structure"]
impl crate::Readable for CMAR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmar3::W`](W) writer structure"]
impl crate::Writable for CMAR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMAR3 to value 0"]
impl crate::Resettable for CMAR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
