#[doc = "Register `DMAOMR` reader"]
pub type R = crate::R<DMAOMR_SPEC>;
#[doc = "Register `DMAOMR` writer"]
pub type W = crate::W<DMAOMR_SPEC>;
#[doc = "Field `SR` reader - SR"]
pub type SR_R = crate::BitReader;
#[doc = "Field `SR` writer - SR"]
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - OSF"]
pub type OSF_R = crate::BitReader;
#[doc = "Field `OSF` writer - OSF"]
pub type OSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - RTC"]
pub type RTC_R = crate::FieldReader;
#[doc = "Field `RTC` writer - RTC"]
pub type RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUGF` reader - FUGF"]
pub type FUGF_R = crate::BitReader;
#[doc = "Field `FUGF` writer - FUGF"]
pub type FUGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - FEF"]
pub type FEF_R = crate::BitReader;
#[doc = "Field `FEF` writer - FEF"]
pub type FEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - TTC"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - TTC"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - FTF"]
pub type FTF_R = crate::BitReader;
#[doc = "Field `FTF` writer - FTF"]
pub type FTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - TSF"]
pub type TSF_R = crate::BitReader;
#[doc = "Field `TSF` writer - TSF"]
pub type TSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFRF` reader - DFRF"]
pub type DFRF_R = crate::BitReader;
#[doc = "Field `DFRF` writer - DFRF"]
pub type DFRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - RSF"]
pub type RSF_R = crate::BitReader;
#[doc = "Field `RSF` writer - RSF"]
pub type RSF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCEFD` reader - DTCEFD"]
pub type DTCEFD_R = crate::BitReader;
#[doc = "Field `DTCEFD` writer - DTCEFD"]
pub type DTCEFD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SR"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OSF"]
    #[inline(always)]
    pub fn osf(&self) -> OSF_R {
        OSF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline(always)]
    pub fn fugf(&self) -> FUGF_R {
        FUGF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FEF"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - FTF"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TSF"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline(always)]
    pub fn dfrf(&self) -> DFRF_R {
        DFRF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RSF"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline(always)]
    pub fn dtcefd(&self) -> DTCEFD_R {
        DTCEFD_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SR"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<DMAOMR_SPEC> {
        SR_W::new(self, 1)
    }
    #[doc = "Bit 2 - OSF"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OSF_W<DMAOMR_SPEC> {
        OSF_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - RTC"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<DMAOMR_SPEC> {
        RTC_W::new(self, 3)
    }
    #[doc = "Bit 6 - FUGF"]
    #[inline(always)]
    #[must_use]
    pub fn fugf(&mut self) -> FUGF_W<DMAOMR_SPEC> {
        FUGF_W::new(self, 6)
    }
    #[doc = "Bit 7 - FEF"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<DMAOMR_SPEC> {
        FEF_W::new(self, 7)
    }
    #[doc = "Bit 13 - ST"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<DMAOMR_SPEC> {
        ST_W::new(self, 13)
    }
    #[doc = "Bits 14:16 - TTC"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<DMAOMR_SPEC> {
        TTC_W::new(self, 14)
    }
    #[doc = "Bit 20 - FTF"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<DMAOMR_SPEC> {
        FTF_W::new(self, 20)
    }
    #[doc = "Bit 21 - TSF"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<DMAOMR_SPEC> {
        TSF_W::new(self, 21)
    }
    #[doc = "Bit 24 - DFRF"]
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DFRF_W<DMAOMR_SPEC> {
        DFRF_W::new(self, 24)
    }
    #[doc = "Bit 25 - RSF"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<DMAOMR_SPEC> {
        RSF_W::new(self, 25)
    }
    #[doc = "Bit 26 - DTCEFD"]
    #[inline(always)]
    #[must_use]
    pub fn dtcefd(&mut self) -> DTCEFD_W<DMAOMR_SPEC> {
        DTCEFD_W::new(self, 26)
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
#[doc = "Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaomr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaomr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAOMR_SPEC;
impl crate::RegisterSpec for DMAOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaomr::R`](R) reader structure"]
impl crate::Readable for DMAOMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaomr::W`](W) writer structure"]
impl crate::Writable for DMAOMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAOMR to value 0"]
impl crate::Resettable for DMAOMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
