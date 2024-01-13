#[doc = "Register `CAN_FM1R` reader"]
pub type R = crate::R<CAN_FM1R_SPEC>;
#[doc = "Register `CAN_FM1R` writer"]
pub type W = crate::W<CAN_FM1R_SPEC>;
#[doc = "Field `FBM0` reader - Filter mode"]
pub type FBM0_R = crate::BitReader;
#[doc = "Field `FBM0` writer - Filter mode"]
pub type FBM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM1` reader - Filter mode"]
pub type FBM1_R = crate::BitReader;
#[doc = "Field `FBM1` writer - Filter mode"]
pub type FBM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM2` reader - Filter mode"]
pub type FBM2_R = crate::BitReader;
#[doc = "Field `FBM2` writer - Filter mode"]
pub type FBM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM3` reader - Filter mode"]
pub type FBM3_R = crate::BitReader;
#[doc = "Field `FBM3` writer - Filter mode"]
pub type FBM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM4` reader - Filter mode"]
pub type FBM4_R = crate::BitReader;
#[doc = "Field `FBM4` writer - Filter mode"]
pub type FBM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM5` reader - Filter mode"]
pub type FBM5_R = crate::BitReader;
#[doc = "Field `FBM5` writer - Filter mode"]
pub type FBM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM6` reader - Filter mode"]
pub type FBM6_R = crate::BitReader;
#[doc = "Field `FBM6` writer - Filter mode"]
pub type FBM6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM7` reader - Filter mode"]
pub type FBM7_R = crate::BitReader;
#[doc = "Field `FBM7` writer - Filter mode"]
pub type FBM7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM8` reader - Filter mode"]
pub type FBM8_R = crate::BitReader;
#[doc = "Field `FBM8` writer - Filter mode"]
pub type FBM8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM9` reader - Filter mode"]
pub type FBM9_R = crate::BitReader;
#[doc = "Field `FBM9` writer - Filter mode"]
pub type FBM9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM10` reader - Filter mode"]
pub type FBM10_R = crate::BitReader;
#[doc = "Field `FBM10` writer - Filter mode"]
pub type FBM10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM11` reader - Filter mode"]
pub type FBM11_R = crate::BitReader;
#[doc = "Field `FBM11` writer - Filter mode"]
pub type FBM11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM12` reader - Filter mode"]
pub type FBM12_R = crate::BitReader;
#[doc = "Field `FBM12` writer - Filter mode"]
pub type FBM12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBM13` reader - Filter mode"]
pub type FBM13_R = crate::BitReader;
#[doc = "Field `FBM13` writer - Filter mode"]
pub type FBM13_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    pub fn fbm0(&self) -> FBM0_R {
        FBM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    pub fn fbm1(&self) -> FBM1_R {
        FBM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    pub fn fbm2(&self) -> FBM2_R {
        FBM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    pub fn fbm3(&self) -> FBM3_R {
        FBM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    pub fn fbm4(&self) -> FBM4_R {
        FBM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    pub fn fbm5(&self) -> FBM5_R {
        FBM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    pub fn fbm6(&self) -> FBM6_R {
        FBM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    pub fn fbm7(&self) -> FBM7_R {
        FBM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    pub fn fbm8(&self) -> FBM8_R {
        FBM8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    pub fn fbm9(&self) -> FBM9_R {
        FBM9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    pub fn fbm10(&self) -> FBM10_R {
        FBM10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    pub fn fbm11(&self) -> FBM11_R {
        FBM11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    pub fn fbm12(&self) -> FBM12_R {
        FBM12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    pub fn fbm13(&self) -> FBM13_R {
        FBM13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm0(&mut self) -> FBM0_W<CAN_FM1R_SPEC> {
        FBM0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm1(&mut self) -> FBM1_W<CAN_FM1R_SPEC> {
        FBM1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm2(&mut self) -> FBM2_W<CAN_FM1R_SPEC> {
        FBM2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm3(&mut self) -> FBM3_W<CAN_FM1R_SPEC> {
        FBM3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm4(&mut self) -> FBM4_W<CAN_FM1R_SPEC> {
        FBM4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm5(&mut self) -> FBM5_W<CAN_FM1R_SPEC> {
        FBM5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm6(&mut self) -> FBM6_W<CAN_FM1R_SPEC> {
        FBM6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm7(&mut self) -> FBM7_W<CAN_FM1R_SPEC> {
        FBM7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm8(&mut self) -> FBM8_W<CAN_FM1R_SPEC> {
        FBM8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm9(&mut self) -> FBM9_W<CAN_FM1R_SPEC> {
        FBM9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm10(&mut self) -> FBM10_W<CAN_FM1R_SPEC> {
        FBM10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm11(&mut self) -> FBM11_W<CAN_FM1R_SPEC> {
        FBM11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm12(&mut self) -> FBM12_W<CAN_FM1R_SPEC> {
        FBM12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter mode"]
    #[inline(always)]
    #[must_use]
    pub fn fbm13(&mut self) -> FBM13_W<CAN_FM1R_SPEC> {
        FBM13_W::new(self, 13)
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
#[doc = "CAN_FM1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fm1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fm1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_FM1R_SPEC;
impl crate::RegisterSpec for CAN_FM1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fm1r::R`](R) reader structure"]
impl crate::Readable for CAN_FM1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_fm1r::W`](W) writer structure"]
impl crate::Writable for CAN_FM1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_FM1R to value 0"]
impl crate::Resettable for CAN_FM1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
