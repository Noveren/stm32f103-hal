#[doc = "Register `CNDTR7` reader"]
pub type R = crate::R<CNDTR7_SPEC>;
#[doc = "Register `CNDTR7` writer"]
pub type W = crate::W<CNDTR7_SPEC>;
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    #[must_use]
    pub fn ndt(&mut self) -> NDT_W<CNDTR7_SPEC> {
        NDT_W::new(self, 0)
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
#[doc = "DMA channel 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cndtr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cndtr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNDTR7_SPEC;
impl crate::RegisterSpec for CNDTR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr7::R`](R) reader structure"]
impl crate::Readable for CNDTR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cndtr7::W`](W) writer structure"]
impl crate::Writable for CNDTR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNDTR7 to value 0"]
impl crate::Resettable for CNDTR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
