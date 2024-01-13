#[doc = "Register `CAN_ESR` reader"]
pub type R = crate::R<CAN_ESR_SPEC>;
#[doc = "Register `CAN_ESR` writer"]
pub type W = crate::W<CAN_ESR_SPEC>;
#[doc = "Field `EWGF` reader - EWGF"]
pub type EWGF_R = crate::BitReader;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EPVF_R = crate::BitReader;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BOFF_R = crate::BitReader;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader;
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - REC"]
pub type REC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<CAN_ESR_SPEC> {
        LEC_W::new(self, 4)
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
#[doc = "CAN_ESR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_esr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_esr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_ESR_SPEC;
impl crate::RegisterSpec for CAN_ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_esr::R`](R) reader structure"]
impl crate::Readable for CAN_ESR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_esr::W`](W) writer structure"]
impl crate::Writable for CAN_ESR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_ESR to value 0"]
impl crate::Resettable for CAN_ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
