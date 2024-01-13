#[doc = "Register `ALRL` writer"]
pub type W = crate::W<ALRL_SPEC>;
#[doc = "Field `ALRL` writer - RTC alarm register low"]
pub type ALRL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    #[must_use]
    pub fn alrl(&mut self) -> ALRL_W<ALRL_SPEC> {
        ALRL_W::new(self, 0)
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
#[doc = "RTC Alarm Register Low\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRL_SPEC;
impl crate::RegisterSpec for ALRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrl::W`](W) writer structure"]
impl crate::Writable for ALRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRL to value 0xffff"]
impl crate::Resettable for ALRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
