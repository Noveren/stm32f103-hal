#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Field `DATA` reader - Regular data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `ADC2DATA` reader - ADC2 data"]
pub type ADC2DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Regular data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADC2 data"]
    #[inline(always)]
    pub fn adc2data(&self) -> ADC2DATA_R {
        ADC2DATA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "regular data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
