#[doc = "Register `BTABLE` reader"]
pub type R = crate::R<BTABLE_SPEC>;
#[doc = "Register `BTABLE` writer"]
pub type W = crate::W<BTABLE_SPEC>;
#[doc = "Field `BTABLE` reader - Buffer table"]
pub type BTABLE_R = crate::FieldReader<u16>;
#[doc = "Field `BTABLE` writer - Buffer table"]
pub type BTABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer table"]
    #[inline(always)]
    #[must_use]
    pub fn btable(&mut self) -> BTABLE_W<BTABLE_SPEC> {
        BTABLE_W::new(self, 3)
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
#[doc = "Buffer table address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTABLE_SPEC;
impl crate::RegisterSpec for BTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btable::R`](R) reader structure"]
impl crate::Readable for BTABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btable::W`](W) writer structure"]
impl crate::Writable for BTABLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTABLE to value 0"]
impl crate::Resettable for BTABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
