#[doc = "Register `LOAD_` reader"]
pub type R = crate::R<LOAD__SPEC>;
#[doc = "Register `LOAD_` writer"]
pub type W = crate::W<LOAD__SPEC>;
#[doc = "Field `RELOAD` reader - RELOAD value"]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - RELOAD value"]
pub type RELOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - RELOAD value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - RELOAD value"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<LOAD__SPEC> {
        RELOAD_W::new(self, 0)
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
#[doc = "SysTick reload value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`load_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOAD__SPEC;
impl crate::RegisterSpec for LOAD__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load_::R`](R) reader structure"]
impl crate::Readable for LOAD__SPEC {}
#[doc = "`write(|w| ..)` method takes [`load_::W`](W) writer structure"]
impl crate::Writable for LOAD__SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOAD_ to value 0"]
impl crate::Resettable for LOAD__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
