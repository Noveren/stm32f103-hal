#[doc = "Register `VAL` reader"]
pub type R = crate::R<VAL_SPEC>;
#[doc = "Register `VAL` writer"]
pub type W = crate::W<VAL_SPEC>;
#[doc = "Field `CURRENT` reader - Current counter value"]
pub type CURRENT_R = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - Current counter value"]
pub type CURRENT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Current counter value"]
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Current counter value"]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<VAL_SPEC> {
        CURRENT_W::new(self, 0)
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
#[doc = "SysTick current value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VAL_SPEC;
impl crate::RegisterSpec for VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`val::R`](R) reader structure"]
impl crate::Readable for VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`val::W`](W) writer structure"]
impl crate::Writable for VAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VAL to value 0"]
impl crate::Resettable for VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
