#[doc = "Register `BRR` reader"]
pub type R = crate::R<BRR_SPEC>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BRR_SPEC>;
#[doc = "Field `DIV_Fraction` reader - DIV_Fraction"]
pub type DIV_FRACTION_R = crate::FieldReader;
#[doc = "Field `DIV_Fraction` writer - DIV_Fraction"]
pub type DIV_FRACTION_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIV_Mantissa` reader - DIV_Mantissa"]
pub type DIV_MANTISSA_R = crate::FieldReader<u16>;
#[doc = "Field `DIV_Mantissa` writer - DIV_Mantissa"]
pub type DIV_MANTISSA_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    pub fn div_fraction(&self) -> DIV_FRACTION_R {
        DIV_FRACTION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    pub fn div_mantissa(&self) -> DIV_MANTISSA_R {
        DIV_MANTISSA_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - DIV_Fraction"]
    #[inline(always)]
    #[must_use]
    pub fn div_fraction(&mut self) -> DIV_FRACTION_W<BRR_SPEC> {
        DIV_FRACTION_W::new(self, 0)
    }
    #[doc = "Bits 4:15 - DIV_Mantissa"]
    #[inline(always)]
    #[must_use]
    pub fn div_mantissa(&mut self) -> DIV_MANTISSA_W<BRR_SPEC> {
        DIV_MANTISSA_W::new(self, 4)
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
#[doc = "UART4_BRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BRR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BRR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
