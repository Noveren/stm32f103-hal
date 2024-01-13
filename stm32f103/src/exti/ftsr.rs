#[doc = "Register `FTSR` reader"]
pub type R = crate::R<FTSR_SPEC>;
#[doc = "Register `FTSR` writer"]
pub type W = crate::W<FTSR_SPEC>;
#[doc = "Field `TR0` reader - Falling trigger event configuration of line 0"]
pub type TR0_R = crate::BitReader;
#[doc = "Field `TR0` writer - Falling trigger event configuration of line 0"]
pub type TR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR1` reader - Falling trigger event configuration of line 1"]
pub type TR1_R = crate::BitReader;
#[doc = "Field `TR1` writer - Falling trigger event configuration of line 1"]
pub type TR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR2` reader - Falling trigger event configuration of line 2"]
pub type TR2_R = crate::BitReader;
#[doc = "Field `TR2` writer - Falling trigger event configuration of line 2"]
pub type TR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR3` reader - Falling trigger event configuration of line 3"]
pub type TR3_R = crate::BitReader;
#[doc = "Field `TR3` writer - Falling trigger event configuration of line 3"]
pub type TR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR4` reader - Falling trigger event configuration of line 4"]
pub type TR4_R = crate::BitReader;
#[doc = "Field `TR4` writer - Falling trigger event configuration of line 4"]
pub type TR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR5` reader - Falling trigger event configuration of line 5"]
pub type TR5_R = crate::BitReader;
#[doc = "Field `TR5` writer - Falling trigger event configuration of line 5"]
pub type TR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR6` reader - Falling trigger event configuration of line 6"]
pub type TR6_R = crate::BitReader;
#[doc = "Field `TR6` writer - Falling trigger event configuration of line 6"]
pub type TR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR7` reader - Falling trigger event configuration of line 7"]
pub type TR7_R = crate::BitReader;
#[doc = "Field `TR7` writer - Falling trigger event configuration of line 7"]
pub type TR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR8` reader - Falling trigger event configuration of line 8"]
pub type TR8_R = crate::BitReader;
#[doc = "Field `TR8` writer - Falling trigger event configuration of line 8"]
pub type TR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR9` reader - Falling trigger event configuration of line 9"]
pub type TR9_R = crate::BitReader;
#[doc = "Field `TR9` writer - Falling trigger event configuration of line 9"]
pub type TR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR10` reader - Falling trigger event configuration of line 10"]
pub type TR10_R = crate::BitReader;
#[doc = "Field `TR10` writer - Falling trigger event configuration of line 10"]
pub type TR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR11` reader - Falling trigger event configuration of line 11"]
pub type TR11_R = crate::BitReader;
#[doc = "Field `TR11` writer - Falling trigger event configuration of line 11"]
pub type TR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR12` reader - Falling trigger event configuration of line 12"]
pub type TR12_R = crate::BitReader;
#[doc = "Field `TR12` writer - Falling trigger event configuration of line 12"]
pub type TR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR13` reader - Falling trigger event configuration of line 13"]
pub type TR13_R = crate::BitReader;
#[doc = "Field `TR13` writer - Falling trigger event configuration of line 13"]
pub type TR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR14` reader - Falling trigger event configuration of line 14"]
pub type TR14_R = crate::BitReader;
#[doc = "Field `TR14` writer - Falling trigger event configuration of line 14"]
pub type TR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR15` reader - Falling trigger event configuration of line 15"]
pub type TR15_R = crate::BitReader;
#[doc = "Field `TR15` writer - Falling trigger event configuration of line 15"]
pub type TR15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR16` reader - Falling trigger event configuration of line 16"]
pub type TR16_R = crate::BitReader;
#[doc = "Field `TR16` writer - Falling trigger event configuration of line 16"]
pub type TR16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR17` reader - Falling trigger event configuration of line 17"]
pub type TR17_R = crate::BitReader;
#[doc = "Field `TR17` writer - Falling trigger event configuration of line 17"]
pub type TR17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR18` reader - Falling trigger event configuration of line 18"]
pub type TR18_R = crate::BitReader;
#[doc = "Field `TR18` writer - Falling trigger event configuration of line 18"]
pub type TR18_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    pub fn tr0(&self) -> TR0_R {
        TR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    pub fn tr1(&self) -> TR1_R {
        TR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    pub fn tr2(&self) -> TR2_R {
        TR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    pub fn tr3(&self) -> TR3_R {
        TR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    pub fn tr4(&self) -> TR4_R {
        TR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    pub fn tr5(&self) -> TR5_R {
        TR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    pub fn tr6(&self) -> TR6_R {
        TR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    pub fn tr7(&self) -> TR7_R {
        TR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    pub fn tr8(&self) -> TR8_R {
        TR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    pub fn tr9(&self) -> TR9_R {
        TR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    pub fn tr10(&self) -> TR10_R {
        TR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    pub fn tr11(&self) -> TR11_R {
        TR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    pub fn tr12(&self) -> TR12_R {
        TR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    pub fn tr13(&self) -> TR13_R {
        TR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    pub fn tr14(&self) -> TR14_R {
        TR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    pub fn tr15(&self) -> TR15_R {
        TR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    pub fn tr16(&self) -> TR16_R {
        TR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    pub fn tr17(&self) -> TR17_R {
        TR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Falling trigger event configuration of line 18"]
    #[inline(always)]
    pub fn tr18(&self) -> TR18_R {
        TR18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling trigger event configuration of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn tr0(&mut self) -> TR0_W<FTSR_SPEC> {
        TR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling trigger event configuration of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn tr1(&mut self) -> TR1_W<FTSR_SPEC> {
        TR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling trigger event configuration of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn tr2(&mut self) -> TR2_W<FTSR_SPEC> {
        TR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling trigger event configuration of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn tr3(&mut self) -> TR3_W<FTSR_SPEC> {
        TR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling trigger event configuration of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn tr4(&mut self) -> TR4_W<FTSR_SPEC> {
        TR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling trigger event configuration of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn tr5(&mut self) -> TR5_W<FTSR_SPEC> {
        TR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling trigger event configuration of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn tr6(&mut self) -> TR6_W<FTSR_SPEC> {
        TR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling trigger event configuration of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn tr7(&mut self) -> TR7_W<FTSR_SPEC> {
        TR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling trigger event configuration of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn tr8(&mut self) -> TR8_W<FTSR_SPEC> {
        TR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling trigger event configuration of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn tr9(&mut self) -> TR9_W<FTSR_SPEC> {
        TR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling trigger event configuration of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn tr10(&mut self) -> TR10_W<FTSR_SPEC> {
        TR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling trigger event configuration of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn tr11(&mut self) -> TR11_W<FTSR_SPEC> {
        TR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling trigger event configuration of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn tr12(&mut self) -> TR12_W<FTSR_SPEC> {
        TR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling trigger event configuration of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn tr13(&mut self) -> TR13_W<FTSR_SPEC> {
        TR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling trigger event configuration of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn tr14(&mut self) -> TR14_W<FTSR_SPEC> {
        TR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling trigger event configuration of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn tr15(&mut self) -> TR15_W<FTSR_SPEC> {
        TR15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Falling trigger event configuration of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn tr16(&mut self) -> TR16_W<FTSR_SPEC> {
        TR16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Falling trigger event configuration of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn tr17(&mut self) -> TR17_W<FTSR_SPEC> {
        TR17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Falling trigger event configuration of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn tr18(&mut self) -> TR18_W<FTSR_SPEC> {
        TR18_W::new(self, 18)
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
#[doc = "Falling Trigger selection register (EXTI_FTSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FTSR_SPEC;
impl crate::RegisterSpec for FTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr::R`](R) reader structure"]
impl crate::Readable for FTSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ftsr::W`](W) writer structure"]
impl crate::Writable for FTSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
