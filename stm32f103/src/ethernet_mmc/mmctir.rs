#[doc = "Register `MMCTIR` reader"]
pub type R = crate::R<MMCTIR_SPEC>;
#[doc = "Register `MMCTIR` writer"]
pub type W = crate::W<MMCTIR_SPEC>;
#[doc = "Field `TGFSCS` reader - Transmitted good frames single collision status"]
pub type TGFSCS_R = crate::BitReader;
#[doc = "Field `TGFSCS` writer - Transmitted good frames single collision status"]
pub type TGFSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFMSCS` reader - Transmitted good frames more single collision status"]
pub type TGFMSCS_R = crate::BitReader;
#[doc = "Field `TGFMSCS` writer - Transmitted good frames more single collision status"]
pub type TGFMSCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGFS` reader - Transmitted good frames status"]
pub type TGFS_R = crate::BitReader;
#[doc = "Field `TGFS` writer - Transmitted good frames status"]
pub type TGFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    #[must_use]
    pub fn tgfscs(&mut self) -> TGFSCS_W<MMCTIR_SPEC> {
        TGFSCS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    #[must_use]
    pub fn tgfmscs(&mut self) -> TGFMSCS_W<MMCTIR_SPEC> {
        TGFMSCS_W::new(self, 15)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    #[must_use]
    pub fn tgfs(&mut self) -> TGFS_W<MMCTIR_SPEC> {
        TGFS_W::new(self, 21)
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
#[doc = "Ethernet MMC transmit interrupt register (ETH_MMCTIR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCTIR_SPEC;
impl crate::RegisterSpec for MMCTIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctir::R`](R) reader structure"]
impl crate::Readable for MMCTIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmctir::W`](W) writer structure"]
impl crate::Writable for MMCTIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCTIR to value 0"]
impl crate::Resettable for MMCTIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
