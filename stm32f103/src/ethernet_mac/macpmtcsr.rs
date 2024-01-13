#[doc = "Register `MACPMTCSR` reader"]
pub type R = crate::R<MACPMTCSR_SPEC>;
#[doc = "Register `MACPMTCSR` writer"]
pub type W = crate::W<MACPMTCSR_SPEC>;
#[doc = "Field `PD` reader - Power down"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - Power down"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPE` reader - Magic Packet enable"]
pub type MPE_R = crate::BitReader;
#[doc = "Field `MPE` writer - Magic Packet enable"]
pub type MPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFE` reader - Wakeup frame enable"]
pub type WFE_R = crate::BitReader;
#[doc = "Field `WFE` writer - Wakeup frame enable"]
pub type WFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPR` reader - Magic packet received"]
pub type MPR_R = crate::BitReader;
#[doc = "Field `MPR` writer - Magic packet received"]
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFR` reader - Wakeup frame received"]
pub type WFR_R = crate::BitReader;
#[doc = "Field `WFR` writer - Wakeup frame received"]
pub type WFR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GU` reader - Global unicast"]
pub type GU_R = crate::BitReader;
#[doc = "Field `GU` writer - Global unicast"]
pub type GU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFFRPR` reader - Wakeup frame filter register pointer reset"]
pub type WFFRPR_R = crate::BitReader;
#[doc = "Field `WFFRPR` writer - Wakeup frame filter register pointer reset"]
pub type WFFRPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    pub fn wfr(&self) -> WFR_R {
        WFR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn wffrpr(&self) -> WFFRPR_R {
        WFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<MACPMTCSR_SPEC> {
        PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<MACPMTCSR_SPEC> {
        MPE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup frame enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<MACPMTCSR_SPEC> {
        WFE_W::new(self, 2)
    }
    #[doc = "Bit 5 - Magic packet received"]
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<MACPMTCSR_SPEC> {
        MPR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Wakeup frame received"]
    #[inline(always)]
    #[must_use]
    pub fn wfr(&mut self) -> WFR_W<MACPMTCSR_SPEC> {
        WFR_W::new(self, 6)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    #[must_use]
    pub fn gu(&mut self) -> GU_W<MACPMTCSR_SPEC> {
        GU_W::new(self, 9)
    }
    #[doc = "Bit 31 - Wakeup frame filter register pointer reset"]
    #[inline(always)]
    #[must_use]
    pub fn wffrpr(&mut self) -> WFFRPR_W<MACPMTCSR_SPEC> {
        WFFRPR_W::new(self, 31)
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
#[doc = "Ethernet MAC PMT control and status register (ETH_MACPMTCSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtcsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtcsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACPMTCSR_SPEC;
impl crate::RegisterSpec for MACPMTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpmtcsr::R`](R) reader structure"]
impl crate::Readable for MACPMTCSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macpmtcsr::W`](W) writer structure"]
impl crate::Writable for MACPMTCSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACPMTCSR to value 0"]
impl crate::Resettable for MACPMTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
