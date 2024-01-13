#[doc = "Register `MACA2LR` reader"]
pub type R = crate::R<MACA2LR_SPEC>;
#[doc = "Register `MACA2LR` writer"]
pub type W = crate::W<MACA2LR_SPEC>;
#[doc = "Field `MACA2L` reader - MAC address2 low"]
pub type MACA2L_R = crate::FieldReader<u32>;
#[doc = "Field `MACA2L` writer - MAC address2 low"]
pub type MACA2L_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - MAC address2 low"]
    #[inline(always)]
    pub fn maca2l(&self) -> MACA2L_R {
        MACA2L_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - MAC address2 low"]
    #[inline(always)]
    #[must_use]
    pub fn maca2l(&mut self) -> MACA2L_W<MACA2LR_SPEC> {
        MACA2L_W::new(self, 0)
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
#[doc = "Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2lr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2lr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA2LR_SPEC;
impl crate::RegisterSpec for MACA2LR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca2lr::R`](R) reader structure"]
impl crate::Readable for MACA2LR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca2lr::W`](W) writer structure"]
impl crate::Writable for MACA2LR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA2LR to value 0xffff_ffff"]
impl crate::Resettable for MACA2LR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
