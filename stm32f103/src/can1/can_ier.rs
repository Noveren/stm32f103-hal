#[doc = "Register `CAN_IER` reader"]
pub type R = crate::R<CAN_IER_SPEC>;
#[doc = "Register `CAN_IER` writer"]
pub type W = crate::W<CAN_IER_SPEC>;
#[doc = "Field `TMEIE` reader - TMEIE"]
pub type TMEIE_R = crate::BitReader;
#[doc = "Field `TMEIE` writer - TMEIE"]
pub type TMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMPIE0` reader - FMPIE0"]
pub type FMPIE0_R = crate::BitReader;
#[doc = "Field `FMPIE0` writer - FMPIE0"]
pub type FMPIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFIE0` reader - FFIE0"]
pub type FFIE0_R = crate::BitReader;
#[doc = "Field `FFIE0` writer - FFIE0"]
pub type FFIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVIE0` reader - FOVIE0"]
pub type FOVIE0_R = crate::BitReader;
#[doc = "Field `FOVIE0` writer - FOVIE0"]
pub type FOVIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMPIE1` reader - FMPIE1"]
pub type FMPIE1_R = crate::BitReader;
#[doc = "Field `FMPIE1` writer - FMPIE1"]
pub type FMPIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFIE1` reader - FFIE1"]
pub type FFIE1_R = crate::BitReader;
#[doc = "Field `FFIE1` writer - FFIE1"]
pub type FFIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVIE1` reader - FOVIE1"]
pub type FOVIE1_R = crate::BitReader;
#[doc = "Field `FOVIE1` writer - FOVIE1"]
pub type FOVIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EWGIE` reader - EWGIE"]
pub type EWGIE_R = crate::BitReader;
#[doc = "Field `EWGIE` writer - EWGIE"]
pub type EWGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPVIE` reader - EPVIE"]
pub type EPVIE_R = crate::BitReader;
#[doc = "Field `EPVIE` writer - EPVIE"]
pub type EPVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOFIE` reader - BOFIE"]
pub type BOFIE_R = crate::BitReader;
#[doc = "Field `BOFIE` writer - BOFIE"]
pub type BOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LECIE` reader - LECIE"]
pub type LECIE_R = crate::BitReader;
#[doc = "Field `LECIE` writer - LECIE"]
pub type LECIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - ERRIE"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - ERRIE"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUIE` reader - WKUIE"]
pub type WKUIE_R = crate::BitReader;
#[doc = "Field `WKUIE` writer - WKUIE"]
pub type WKUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLKIE` reader - SLKIE"]
pub type SLKIE_R = crate::BitReader;
#[doc = "Field `SLKIE` writer - SLKIE"]
pub type SLKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TMEIE"]
    #[inline(always)]
    #[must_use]
    pub fn tmeie(&mut self) -> TMEIE_W<CAN_IER_SPEC> {
        TMEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - FMPIE0"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie0(&mut self) -> FMPIE0_W<CAN_IER_SPEC> {
        FMPIE0_W::new(self, 1)
    }
    #[doc = "Bit 2 - FFIE0"]
    #[inline(always)]
    #[must_use]
    pub fn ffie0(&mut self) -> FFIE0_W<CAN_IER_SPEC> {
        FFIE0_W::new(self, 2)
    }
    #[doc = "Bit 3 - FOVIE0"]
    #[inline(always)]
    #[must_use]
    pub fn fovie0(&mut self) -> FOVIE0_W<CAN_IER_SPEC> {
        FOVIE0_W::new(self, 3)
    }
    #[doc = "Bit 4 - FMPIE1"]
    #[inline(always)]
    #[must_use]
    pub fn fmpie1(&mut self) -> FMPIE1_W<CAN_IER_SPEC> {
        FMPIE1_W::new(self, 4)
    }
    #[doc = "Bit 5 - FFIE1"]
    #[inline(always)]
    #[must_use]
    pub fn ffie1(&mut self) -> FFIE1_W<CAN_IER_SPEC> {
        FFIE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - FOVIE1"]
    #[inline(always)]
    #[must_use]
    pub fn fovie1(&mut self) -> FOVIE1_W<CAN_IER_SPEC> {
        FOVIE1_W::new(self, 6)
    }
    #[doc = "Bit 8 - EWGIE"]
    #[inline(always)]
    #[must_use]
    pub fn ewgie(&mut self) -> EWGIE_W<CAN_IER_SPEC> {
        EWGIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - EPVIE"]
    #[inline(always)]
    #[must_use]
    pub fn epvie(&mut self) -> EPVIE_W<CAN_IER_SPEC> {
        EPVIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - BOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn bofie(&mut self) -> BOFIE_W<CAN_IER_SPEC> {
        BOFIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - LECIE"]
    #[inline(always)]
    #[must_use]
    pub fn lecie(&mut self) -> LECIE_W<CAN_IER_SPEC> {
        LECIE_W::new(self, 11)
    }
    #[doc = "Bit 15 - ERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CAN_IER_SPEC> {
        ERRIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - WKUIE"]
    #[inline(always)]
    #[must_use]
    pub fn wkuie(&mut self) -> WKUIE_W<CAN_IER_SPEC> {
        WKUIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - SLKIE"]
    #[inline(always)]
    #[must_use]
    pub fn slkie(&mut self) -> SLKIE_W<CAN_IER_SPEC> {
        SLKIE_W::new(self, 17)
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
#[doc = "CAN_IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_IER_SPEC;
impl crate::RegisterSpec for CAN_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ier::R`](R) reader structure"]
impl crate::Readable for CAN_IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_ier::W`](W) writer structure"]
impl crate::Writable for CAN_IER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_IER to value 0"]
impl crate::Resettable for CAN_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
