#[doc = "Register `ALRH` writer"]
pub type W = crate::W<ALRH_SPEC>;
#[doc = "Field `ALRH` writer - RTC alarm register high"]
pub type ALRH_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RTC alarm register high"]
    #[inline(always)]
    #[must_use]
    pub fn alrh(&mut self) -> ALRH_W<ALRH_SPEC> {
        ALRH_W::new(self, 0)
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
#[doc = "RTC Alarm Register High\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRH_SPEC;
impl crate::RegisterSpec for ALRH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`alrh::W`](W) writer structure"]
impl crate::Writable for ALRH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRH to value 0xffff"]
impl crate::Resettable for ALRH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
