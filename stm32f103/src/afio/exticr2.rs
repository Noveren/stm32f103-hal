#[doc = "Register `EXTICR2` reader"]
pub type R = crate::R<EXTICR2_SPEC>;
#[doc = "Register `EXTICR2` writer"]
pub type W = crate::W<EXTICR2_SPEC>;
#[doc = "Field `EXTI4` reader - EXTI4 configuration"]
pub type EXTI4_R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI4 configuration"]
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI5` reader - EXTI5 configuration"]
pub type EXTI5_R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI5 configuration"]
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI6` reader - EXTI6 configuration"]
pub type EXTI6_R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI6 configuration"]
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI7` reader - EXTI7 configuration"]
pub type EXTI7_R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI7 configuration"]
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI4 configuration"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI5 configuration"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI6 configuration"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI7 configuration"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI4 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2_SPEC> {
        EXTI4_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI5 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2_SPEC> {
        EXTI5_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI6 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2_SPEC> {
        EXTI6_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI7 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTICR2_SPEC> {
        EXTI7_W::new(self, 12)
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
#[doc = "External interrupt configuration register 2 (AFIO_EXTICR2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR2_SPEC;
impl crate::RegisterSpec for EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr2::R`](R) reader structure"]
impl crate::Readable for EXTICR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exticr2::W`](W) writer structure"]
impl crate::Writable for EXTICR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR2 to value 0"]
impl crate::Resettable for EXTICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
