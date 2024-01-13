#[doc = "Register `DLEN` reader"]
pub type R = crate::R<DLEN_SPEC>;
#[doc = "Register `DLEN` writer"]
pub type W = crate::W<DLEN_SPEC>;
#[doc = "Field `DATALENGTH` reader - Data length value"]
pub type DATALENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `DATALENGTH` writer - Data length value"]
pub type DATALENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value"]
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<DLEN_SPEC> {
        DATALENGTH_W::new(self, 0)
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
#[doc = "Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLEN_SPEC;
impl crate::RegisterSpec for DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlen::R`](R) reader structure"]
impl crate::Readable for DLEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dlen::W`](W) writer structure"]
impl crate::Writable for DLEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLEN to value 0"]
impl crate::Resettable for DLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
