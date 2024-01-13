#[doc = "Register `CAN_BTR` reader"]
pub type R = crate::R<CAN_BTR_SPEC>;
#[doc = "Register `CAN_BTR` writer"]
pub type W = crate::W<CAN_BTR_SPEC>;
#[doc = "Field `BRP` reader - BRP"]
pub type BRP_R = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - BRP"]
pub type BRP_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TS1` reader - TS1"]
pub type TS1_R = crate::FieldReader;
#[doc = "Field `TS1` writer - TS1"]
pub type TS1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS2` reader - TS2"]
pub type TS2_R = crate::FieldReader;
#[doc = "Field `TS2` writer - TS2"]
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SJW` reader - SJW"]
pub type SJW_R = crate::FieldReader;
#[doc = "Field `SJW` writer - SJW"]
pub type SJW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBKM` reader - LBKM"]
pub type LBKM_R = crate::BitReader;
#[doc = "Field `LBKM` writer - LBKM"]
pub type LBKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SILM` reader - SILM"]
pub type SILM_R = crate::BitReader;
#[doc = "Field `SILM` writer - SILM"]
pub type SILM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    pub fn lbkm(&self) -> LBKM_R {
        LBKM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    pub fn silm(&self) -> SILM_R {
        SILM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - BRP"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<CAN_BTR_SPEC> {
        BRP_W::new(self, 0)
    }
    #[doc = "Bits 16:19 - TS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<CAN_BTR_SPEC> {
        TS1_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - TS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<CAN_BTR_SPEC> {
        TS2_W::new(self, 20)
    }
    #[doc = "Bits 24:25 - SJW"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<CAN_BTR_SPEC> {
        SJW_W::new(self, 24)
    }
    #[doc = "Bit 30 - LBKM"]
    #[inline(always)]
    #[must_use]
    pub fn lbkm(&mut self) -> LBKM_W<CAN_BTR_SPEC> {
        LBKM_W::new(self, 30)
    }
    #[doc = "Bit 31 - SILM"]
    #[inline(always)]
    #[must_use]
    pub fn silm(&mut self) -> SILM_W<CAN_BTR_SPEC> {
        SILM_W::new(self, 31)
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
#[doc = "CAN_BTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_btr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_btr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_BTR_SPEC;
impl crate::RegisterSpec for CAN_BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_btr::R`](R) reader structure"]
impl crate::Readable for CAN_BTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_btr::W`](W) writer structure"]
impl crate::Writable for CAN_BTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_BTR to value 0"]
impl crate::Resettable for CAN_BTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
