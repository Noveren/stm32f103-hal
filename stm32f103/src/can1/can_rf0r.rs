#[doc = "Register `CAN_RF0R` reader"]
pub type R = crate::R<CAN_RF0R_SPEC>;
#[doc = "Register `CAN_RF0R` writer"]
pub type W = crate::W<CAN_RF0R_SPEC>;
#[doc = "Field `FMP0` reader - FMP0"]
pub type FMP0_R = crate::FieldReader;
#[doc = "Field `FULL0` reader - FULL0"]
pub type FULL0_R = crate::BitReader;
#[doc = "Field `FULL0` writer - FULL0"]
pub type FULL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOVR0` reader - FOVR0"]
pub type FOVR0_R = crate::BitReader;
#[doc = "Field `FOVR0` writer - FOVR0"]
pub type FOVR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFOM0` reader - RFOM0"]
pub type RFOM0_R = crate::BitReader;
#[doc = "Field `RFOM0` writer - RFOM0"]
pub type RFOM0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp0(&self) -> FMP0_R {
        FMP0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full0(&self) -> FULL0_R {
        FULL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr0(&self) -> FOVR0_R {
        FOVR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom0(&self) -> RFOM0_R {
        RFOM0_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    #[must_use]
    pub fn full0(&mut self) -> FULL0_W<CAN_RF0R_SPEC> {
        FULL0_W::new(self, 3)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    #[must_use]
    pub fn fovr0(&mut self) -> FOVR0_W<CAN_RF0R_SPEC> {
        FOVR0_W::new(self, 4)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    #[must_use]
    pub fn rfom0(&mut self) -> RFOM0_W<CAN_RF0R_SPEC> {
        RFOM0_W::new(self, 5)
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
#[doc = "CAN_RF0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rf0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_rf0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_RF0R_SPEC;
impl crate::RegisterSpec for CAN_RF0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_rf0r::R`](R) reader structure"]
impl crate::Readable for CAN_RF0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_rf0r::W`](W) writer structure"]
impl crate::Writable for CAN_RF0R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_RF0R to value 0"]
impl crate::Resettable for CAN_RF0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
