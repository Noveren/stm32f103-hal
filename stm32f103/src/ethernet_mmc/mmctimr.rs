#[doc = "Register `MMCTIMR` reader"]
pub type R = crate::R<MMCTIMR_SPEC>;
#[doc = "Register `MMCTIMR` writer"]
pub type W = crate::W<MMCTIMR_SPEC>;
#[doc = "Field `TGFSCM` reader - Transmitted good frames single collision mask"]
pub type TGFSCM_R = crate::BitReader;
#[doc = "Field `TGFSCM` writer - Transmitted good frames single collision mask"]
pub type TGFSCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSCM` reader - Transmitted good frames more single collision mask"]
pub type TGFMSCM_R = crate::BitReader;
#[doc = "Field `TGFMSCM` writer - Transmitted good frames more single collision mask"]
pub type TGFMSCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFM` reader - Transmitted good frames mask"]
pub type TGFM_R = crate::BitReader;
#[doc = "Field `TGFM` writer - Transmitted good frames mask"]
pub type TGFM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    pub fn tgfscm(&self) -> TGFSCM_R {
        TGFSCM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision mask"]
    #[inline(always)]
    pub fn tgfmscm(&self) -> TGFMSCM_R {
        TGFMSCM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    pub fn tgfm(&self) -> TGFM_R {
        TGFM_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfscm(&mut self) -> TGFSCM_W<MMCTIMR_SPEC> {
        TGFSCM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfmscm(&mut self) -> TGFMSCM_W<MMCTIMR_SPEC> {
        TGFMSCM_W::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames mask"]
    #[inline(always)]
    #[must_use]
    pub fn tgfm(&mut self) -> TGFM_W<MMCTIMR_SPEC> {
        TGFM_W::new(self, 21)
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
#[doc = "Ethernet MMC transmit interrupt mask register (ETH_MMCTIMR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTIMR_SPEC;
impl crate::RegisterSpec for MMCTIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctimr::R`](R) reader structure"]
impl crate::Readable for MMCTIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmctimr::W`](W) writer structure"]
impl crate::Writable for MMCTIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCTIMR to value 0"]
impl crate::Resettable for MMCTIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
