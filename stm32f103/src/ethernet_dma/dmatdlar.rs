#[doc = "Register `DMATDLAR` reader"]
pub type R = crate::R<DMATDLAR_SPEC>;
#[doc = "Register `DMATDLAR` writer"]
pub type W = crate::W<DMATDLAR_SPEC>;
#[doc = "Field `STL` reader - Start of transmit list"]
pub type STL_R = crate::FieldReader<u32>;
#[doc = "Field `STL` writer - Start of transmit list"]
pub type STL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    pub fn stl(&self) -> STL_R {
        STL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    #[must_use]
    pub fn stl(&mut self) -> STL_W<DMATDLAR_SPEC> {
        STL_W::new(self, 0)
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
#[doc = "Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdlar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATDLAR_SPEC;
impl crate::RegisterSpec for DMATDLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdlar::R`](R) reader structure"]
impl crate::Readable for DMATDLAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatdlar::W`](W) writer structure"]
impl crate::Writable for DMATDLAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATDLAR to value 0"]
impl crate::Resettable for DMATDLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
