#[doc = "Register `PTPSSIR` reader"]
pub type R = crate::R<PTPSSIR_SPEC>;
#[doc = "Register `PTPSSIR` writer"]
pub type W = crate::W<PTPSSIR_SPEC>;
#[doc = "Field `STSSI` reader - System time subsecond increment"]
pub type STSSI_R = crate::FieldReader;
#[doc = "Field `STSSI` writer - System time subsecond increment"]
pub type STSSI_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    pub fn stssi(&self) -> STSSI_R {
        STSSI_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - System time subsecond increment"]
    #[inline(always)]
    #[must_use]
    pub fn stssi(&mut self) -> STSSI_W<PTPSSIR_SPEC> {
        STSSI_W::new(self, 0)
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
#[doc = "Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTPSSIR_SPEC;
impl crate::RegisterSpec for PTPSSIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpssir::R`](R) reader structure"]
impl crate::Readable for PTPSSIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptpssir::W`](W) writer structure"]
impl crate::Writable for PTPSSIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTPSSIR to value 0"]
impl crate::Resettable for PTPSSIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
