#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `SBK` reader - SBK"]
pub type SBK_R = crate::BitReader;
#[doc = "Field `SBK` writer - SBK"]
pub type SBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWU` reader - RWU"]
pub type RWU_R = crate::BitReader;
#[doc = "Field `RWU` writer - RWU"]
pub type RWU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - RE"]
pub type RE_R = crate::BitReader;
#[doc = "Field `RE` writer - RE"]
pub type RE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - TE"]
pub type TE_R = crate::BitReader;
#[doc = "Field `TE` writer - TE"]
pub type TE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIE` reader - IDLEIE"]
pub type IDLEIE_R = crate::BitReader;
#[doc = "Field `IDLEIE` writer - IDLEIE"]
pub type IDLEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RXNEIE"]
pub type RXNEIE_R = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNEIE"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - TXEIE"]
pub type TXEIE_R = crate::BitReader;
#[doc = "Field `TXEIE` writer - TXEIE"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIE` reader - PEIE"]
pub type PEIE_R = crate::BitReader;
#[doc = "Field `PEIE` writer - PEIE"]
pub type PEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - PS"]
pub type PS_R = crate::BitReader;
#[doc = "Field `PS` writer - PS"]
pub type PS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - PCE"]
pub type PCE_R = crate::BitReader;
#[doc = "Field `PCE` writer - PCE"]
pub type PCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKE` reader - WAKE"]
pub type WAKE_R = crate::BitReader;
#[doc = "Field `WAKE` writer - WAKE"]
pub type WAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - M"]
pub type M_R = crate::BitReader;
#[doc = "Field `M` writer - M"]
pub type M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UE` reader - UE"]
pub type UE_R = crate::BitReader;
#[doc = "Field `UE` writer - UE"]
pub type UE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SBK"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RWU"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RE"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TE"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLEIE"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNEIE"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXEIE"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PS"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCE"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WAKE"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - M"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UE"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBK"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SBK_W<CR1_SPEC> {
        SBK_W::new(self, 0)
    }
    #[doc = "Bit 1 - RWU"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<CR1_SPEC> {
        RWU_W::new(self, 1)
    }
    #[doc = "Bit 2 - RE"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<CR1_SPEC> {
        RE_W::new(self, 2)
    }
    #[doc = "Bit 3 - TE"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<CR1_SPEC> {
        TE_W::new(self, 3)
    }
    #[doc = "Bit 4 - IDLEIE"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<CR1_SPEC> {
        IDLEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR1_SPEC> {
        RXNEIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CR1_SPEC> {
        TCIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - TXEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR1_SPEC> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - PEIE"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<CR1_SPEC> {
        PEIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - PS"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<CR1_SPEC> {
        PS_W::new(self, 9)
    }
    #[doc = "Bit 10 - PCE"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<CR1_SPEC> {
        PCE_W::new(self, 10)
    }
    #[doc = "Bit 11 - WAKE"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<CR1_SPEC> {
        WAKE_W::new(self, 11)
    }
    #[doc = "Bit 12 - M"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<CR1_SPEC> {
        M_W::new(self, 12)
    }
    #[doc = "Bit 13 - UE"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<CR1_SPEC> {
        UE_W::new(self, 13)
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
#[doc = "UART4_CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
