#[doc = "Register `CAN_RF1R` reader"]
pub type R = crate::R<CAN_RF1R_SPEC>;
#[doc = "Register `CAN_RF1R` writer"]
pub type W = crate::W<CAN_RF1R_SPEC>;
#[doc = "Field `FMP1` reader - FMP1"]
pub type FMP1_R = crate::FieldReader;
#[doc = "Field `FULL1` reader - FULL1"]
pub type FULL1_R = crate::BitReader;
#[doc = "Field `FULL1` writer - FULL1"]
pub type FULL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVR1` reader - FOVR1"]
pub type FOVR1_R = crate::BitReader;
#[doc = "Field `FOVR1` writer - FOVR1"]
pub type FOVR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOM1` reader - RFOM1"]
pub type RFOM1_R = crate::BitReader;
#[doc = "Field `RFOM1` writer - RFOM1"]
pub type RFOM1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FMP1"]
    #[inline(always)]
    pub fn fmp1(&self) -> FMP1_R {
        FMP1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    pub fn full1(&self) -> FULL1_R {
        FULL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    pub fn fovr1(&self) -> FOVR1_R {
        FOVR1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    pub fn rfom1(&self) -> RFOM1_R {
        RFOM1_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FULL1"]
    #[inline(always)]
    #[must_use]
    pub fn full1(&mut self) -> FULL1_W<CAN_RF1R_SPEC> {
        FULL1_W::new(self, 3)
    }
    #[doc = "Bit 4 - FOVR1"]
    #[inline(always)]
    #[must_use]
    pub fn fovr1(&mut self) -> FOVR1_W<CAN_RF1R_SPEC> {
        FOVR1_W::new(self, 4)
    }
    #[doc = "Bit 5 - RFOM1"]
    #[inline(always)]
    #[must_use]
    pub fn rfom1(&mut self) -> RFOM1_W<CAN_RF1R_SPEC> {
        RFOM1_W::new(self, 5)
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
#[doc = "CAN_RF1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rf1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_rf1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_RF1R_SPEC;
impl crate::RegisterSpec for CAN_RF1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rf1r::R`](R) reader structure"]
impl crate::Readable for CAN_RF1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_rf1r::W`](W) writer structure"]
impl crate::Writable for CAN_RF1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_RF1R to value 0"]
impl crate::Resettable for CAN_RF1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
