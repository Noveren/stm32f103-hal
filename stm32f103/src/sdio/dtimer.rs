#[doc = "Register `DTIMER` reader"]
pub type R = crate::R<DTIMER_SPEC>;
#[doc = "Register `DTIMER` writer"]
pub type W = crate::W<DTIMER_SPEC>;
#[doc = "Field `DATATIME` reader - Data timeout period"]
pub type DATATIME_R = crate::FieldReader<u32>;
#[doc = "Field `DATATIME` writer - Data timeout period"]
pub type DATATIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    #[must_use]
    pub fn datatime(&mut self) -> DATATIME_W<DTIMER_SPEC> {
        DATATIME_W::new(self, 0)
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
#[doc = "Bits 31:0 = DATATIME: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtimer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtimer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTIMER_SPEC;
impl crate::RegisterSpec for DTIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtimer::R`](R) reader structure"]
impl crate::Readable for DTIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtimer::W`](W) writer structure"]
impl crate::Writable for DTIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DTIMER to value 0"]
impl crate::Resettable for DTIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
