#[doc = "Register `MMCCR` reader"]
pub type R = crate::R<MMCCR_SPEC>;
#[doc = "Register `MMCCR` writer"]
pub type W = crate::W<MMCCR_SPEC>;
#[doc = "Field `CR` reader - Counter reset"]
pub type CR_R = crate::BitReader;
#[doc = "Field `CR` writer - Counter reset"]
pub type CR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - Counter stop rollover"]
pub type CSR_R = crate::BitReader;
#[doc = "Field `CSR` writer - Counter stop rollover"]
pub type CSR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROR` reader - Reset on read"]
pub type ROR_R = crate::BitReader;
#[doc = "Field `ROR` writer - Reset on read"]
pub type ROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCF` reader - MMC counter freeze"]
pub type MCF_R = crate::BitReader;
#[doc = "Field `MCF` writer - MMC counter freeze"]
pub type MCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    pub fn ror(&self) -> ROR_R {
        ROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    pub fn mcf(&self) -> MCF_R {
        MCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<MMCCR_SPEC> {
        CR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter stop rollover"]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<MMCCR_SPEC> {
        CSR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reset on read"]
    #[inline(always)]
    #[must_use]
    pub fn ror(&mut self) -> ROR_W<MMCCR_SPEC> {
        ROR_W::new(self, 2)
    }
    #[doc = "Bit 31 - MMC counter freeze"]
    #[inline(always)]
    #[must_use]
    pub fn mcf(&mut self) -> MCF_W<MMCCR_SPEC> {
        MCF_W::new(self, 31)
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
#[doc = "Ethernet MMC control register (ETH_MMCCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCCR_SPEC;
impl crate::RegisterSpec for MMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmccr::R`](R) reader structure"]
impl crate::Readable for MMCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmccr::W`](W) writer structure"]
impl crate::Writable for MMCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCCR to value 0"]
impl crate::Resettable for MMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
