#[doc = "Register `CAN_TDT1R` reader"]
pub type R = crate::R<CAN_TDT1R_SPEC>;
#[doc = "Register `CAN_TDT1R` writer"]
pub type W = crate::W<CAN_TDT1R_SPEC>;
#[doc = "Field `DLC` reader - DLC"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - DLC"]
pub type DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TGT` reader - TGT"]
pub type TGT_R = crate::BitReader;
#[doc = "Field `TGT` writer - TGT"]
pub type TGT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIME` reader - TIME"]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - TIME"]
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    pub fn tgt(&self) -> TGT_R {
        TGT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DLC"]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<CAN_TDT1R_SPEC> {
        DLC_W::new(self, 0)
    }
    #[doc = "Bit 8 - TGT"]
    #[inline(always)]
    #[must_use]
    pub fn tgt(&mut self) -> TGT_W<CAN_TDT1R_SPEC> {
        TGT_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - TIME"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<CAN_TDT1R_SPEC> {
        TIME_W::new(self, 16)
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
#[doc = "CAN_TDT1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdt1r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdt1r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_TDT1R_SPEC;
impl crate::RegisterSpec for CAN_TDT1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tdt1r::R`](R) reader structure"]
impl crate::Readable for CAN_TDT1R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_tdt1r::W`](W) writer structure"]
impl crate::Writable for CAN_TDT1R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_TDT1R to value 0"]
impl crate::Resettable for CAN_TDT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
