#[doc = "Register `CAN_FS1R` reader"]
pub type R = crate::R<CAN_FS1R_SPEC>;
#[doc = "Register `CAN_FS1R` writer"]
pub type W = crate::W<CAN_FS1R_SPEC>;
#[doc = "Field `FSC0` reader - Filter scale configuration"]
pub type FSC0_R = crate::BitReader;
#[doc = "Field `FSC0` writer - Filter scale configuration"]
pub type FSC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC1` reader - Filter scale configuration"]
pub type FSC1_R = crate::BitReader;
#[doc = "Field `FSC1` writer - Filter scale configuration"]
pub type FSC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC2` reader - Filter scale configuration"]
pub type FSC2_R = crate::BitReader;
#[doc = "Field `FSC2` writer - Filter scale configuration"]
pub type FSC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC3` reader - Filter scale configuration"]
pub type FSC3_R = crate::BitReader;
#[doc = "Field `FSC3` writer - Filter scale configuration"]
pub type FSC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC4` reader - Filter scale configuration"]
pub type FSC4_R = crate::BitReader;
#[doc = "Field `FSC4` writer - Filter scale configuration"]
pub type FSC4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC5` reader - Filter scale configuration"]
pub type FSC5_R = crate::BitReader;
#[doc = "Field `FSC5` writer - Filter scale configuration"]
pub type FSC5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC6` reader - Filter scale configuration"]
pub type FSC6_R = crate::BitReader;
#[doc = "Field `FSC6` writer - Filter scale configuration"]
pub type FSC6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC7` reader - Filter scale configuration"]
pub type FSC7_R = crate::BitReader;
#[doc = "Field `FSC7` writer - Filter scale configuration"]
pub type FSC7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC8` reader - Filter scale configuration"]
pub type FSC8_R = crate::BitReader;
#[doc = "Field `FSC8` writer - Filter scale configuration"]
pub type FSC8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC9` reader - Filter scale configuration"]
pub type FSC9_R = crate::BitReader;
#[doc = "Field `FSC9` writer - Filter scale configuration"]
pub type FSC9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC10` reader - Filter scale configuration"]
pub type FSC10_R = crate::BitReader;
#[doc = "Field `FSC10` writer - Filter scale configuration"]
pub type FSC10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC11` reader - Filter scale configuration"]
pub type FSC11_R = crate::BitReader;
#[doc = "Field `FSC11` writer - Filter scale configuration"]
pub type FSC11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC12` reader - Filter scale configuration"]
pub type FSC12_R = crate::BitReader;
#[doc = "Field `FSC12` writer - Filter scale configuration"]
pub type FSC12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSC13` reader - Filter scale configuration"]
pub type FSC13_R = crate::BitReader;
#[doc = "Field `FSC13` writer - Filter scale configuration"]
pub type FSC13_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc0(&self) -> FSC0_R {
        FSC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc1(&self) -> FSC1_R {
        FSC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc2(&self) -> FSC2_R {
        FSC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc3(&self) -> FSC3_R {
        FSC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc4(&self) -> FSC4_R {
        FSC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc5(&self) -> FSC5_R {
        FSC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc6(&self) -> FSC6_R {
        FSC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc7(&self) -> FSC7_R {
        FSC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc8(&self) -> FSC8_R {
        FSC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc9(&self) -> FSC9_R {
        FSC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc10(&self) -> FSC10_R {
        FSC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc11(&self) -> FSC11_R {
        FSC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc12(&self) -> FSC12_R {
        FSC12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    pub fn fsc13(&self) -> FSC13_R {
        FSC13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc0(&mut self) -> FSC0_W<CAN_FS1R_SPEC> {
        FSC0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc1(&mut self) -> FSC1_W<CAN_FS1R_SPEC> {
        FSC1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc2(&mut self) -> FSC2_W<CAN_FS1R_SPEC> {
        FSC2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc3(&mut self) -> FSC3_W<CAN_FS1R_SPEC> {
        FSC3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc4(&mut self) -> FSC4_W<CAN_FS1R_SPEC> {
        FSC4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc5(&mut self) -> FSC5_W<CAN_FS1R_SPEC> {
        FSC5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc6(&mut self) -> FSC6_W<CAN_FS1R_SPEC> {
        FSC6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc7(&mut self) -> FSC7_W<CAN_FS1R_SPEC> {
        FSC7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc8(&mut self) -> FSC8_W<CAN_FS1R_SPEC> {
        FSC8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc9(&mut self) -> FSC9_W<CAN_FS1R_SPEC> {
        FSC9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc10(&mut self) -> FSC10_W<CAN_FS1R_SPEC> {
        FSC10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc11(&mut self) -> FSC11_W<CAN_FS1R_SPEC> {
        FSC11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc12(&mut self) -> FSC12_W<CAN_FS1R_SPEC> {
        FSC12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter scale configuration"]
    #[inline(always)]
    #[must_use]
    pub fn fsc13(&mut self) -> FSC13_W<CAN_FS1R_SPEC> {
        FSC13_W::new(self, 13)
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
#[doc = "CAN_FS1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fs1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fs1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_FS1R_SPEC;
impl crate::RegisterSpec for CAN_FS1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_fs1r::R`](R) reader structure"]
impl crate::Readable for CAN_FS1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_fs1r::W`](W) writer structure"]
impl crate::Writable for CAN_FS1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_FS1R to value 0"]
impl crate::Resettable for CAN_FS1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
