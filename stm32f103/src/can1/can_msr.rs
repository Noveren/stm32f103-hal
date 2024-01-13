#[doc = "Register `CAN_MSR` reader"]
pub type R = crate::R<CAN_MSR_SPEC>;
#[doc = "Register `CAN_MSR` writer"]
pub type W = crate::W<CAN_MSR_SPEC>;
#[doc = "Field `INAK` reader - INAK"]
pub type INAK_R = crate::BitReader;
#[doc = "Field `SLAK` reader - SLAK"]
pub type SLAK_R = crate::BitReader;
#[doc = "Field `ERRI` reader - ERRI"]
pub type ERRI_R = crate::BitReader;
#[doc = "Field `ERRI` writer - ERRI"]
pub type ERRI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUI` reader - WKUI"]
pub type WKUI_R = crate::BitReader;
#[doc = "Field `WKUI` writer - WKUI"]
pub type WKUI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAKI` reader - SLAKI"]
pub type SLAKI_R = crate::BitReader;
#[doc = "Field `SLAKI` writer - SLAKI"]
pub type SLAKI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXM` reader - TXM"]
pub type TXM_R = crate::BitReader;
#[doc = "Field `RXM` reader - RXM"]
pub type RXM_R = crate::BitReader;
#[doc = "Field `SAMP` reader - SAMP"]
pub type SAMP_R = crate::BitReader;
#[doc = "Field `RX` reader - RX"]
pub type RX_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - INAK"]
    #[inline(always)]
    pub fn inak(&self) -> INAK_R {
        INAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SLAK"]
    #[inline(always)]
    pub fn slak(&self) -> SLAK_R {
        SLAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    pub fn wkui(&self) -> WKUI_R {
        WKUI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    pub fn slaki(&self) -> SLAKI_R {
        SLAKI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - TXM"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXM"]
    #[inline(always)]
    pub fn rxm(&self) -> RXM_R {
        RXM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SAMP"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RX"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - ERRI"]
    #[inline(always)]
    #[must_use]
    pub fn erri(&mut self) -> ERRI_W<CAN_MSR_SPEC> {
        ERRI_W::new(self, 2)
    }
    #[doc = "Bit 3 - WKUI"]
    #[inline(always)]
    #[must_use]
    pub fn wkui(&mut self) -> WKUI_W<CAN_MSR_SPEC> {
        WKUI_W::new(self, 3)
    }
    #[doc = "Bit 4 - SLAKI"]
    #[inline(always)]
    #[must_use]
    pub fn slaki(&mut self) -> SLAKI_W<CAN_MSR_SPEC> {
        SLAKI_W::new(self, 4)
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
#[doc = "CAN_MSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_msr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_msr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_MSR_SPEC;
impl crate::RegisterSpec for CAN_MSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_msr::R`](R) reader structure"]
impl crate::Readable for CAN_MSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_msr::W`](W) writer structure"]
impl crate::Writable for CAN_MSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_MSR to value 0"]
impl crate::Resettable for CAN_MSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
