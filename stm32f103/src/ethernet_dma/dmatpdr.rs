#[doc = "Register `DMATPDR` reader"]
pub type R = crate::R<DMATPDR_SPEC>;
#[doc = "Register `DMATPDR` writer"]
pub type W = crate::W<DMATPDR_SPEC>;
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub type TPD_R = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TPD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TPD_W<DMATPDR_SPEC> {
        TPD_W::new(self, 0)
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
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATPDR_SPEC;
impl crate::RegisterSpec for DMATPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpdr::R`](R) reader structure"]
impl crate::Readable for DMATPDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatpdr::W`](W) writer structure"]
impl crate::Writable for DMATPDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATPDR to value 0"]
impl crate::Resettable for DMATPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
