#[doc = "Register `MACHTLR` reader"]
pub type R = crate::R<MACHTLR_SPEC>;
#[doc = "Register `MACHTLR` writer"]
pub type W = crate::W<MACHTLR_SPEC>;
#[doc = "Field `HTL` reader - Hash table low"]
pub type HTL_R = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash table low"]
pub type HTL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    pub fn htl(&self) -> HTL_R {
        HTL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HTL_W<MACHTLR_SPEC> {
        HTL_W::new(self, 0)
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
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACHTLR_SPEC;
impl crate::RegisterSpec for MACHTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machtlr::R`](R) reader structure"]
impl crate::Readable for MACHTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`machtlr::W`](W) writer structure"]
impl crate::Writable for MACHTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACHTLR to value 0"]
impl crate::Resettable for MACHTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
