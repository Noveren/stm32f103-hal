#[doc = "Register `DR6` reader"]
pub type R = crate::R<DR6_SPEC>;
#[doc = "Register `DR6` writer"]
pub type W = crate::W<DR6_SPEC>;
#[doc = "Field `D6` reader - Backup data"]
pub type D6_R = crate::FieldReader<u16>;
#[doc = "Field `D6` writer - Backup data"]
pub type D6_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn d6(&self) -> D6_R {
        D6_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn d6(&mut self) -> D6_W<DR6_SPEC> {
        D6_W::new(self, 0)
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
#[doc = "Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR6_SPEC;
impl crate::RegisterSpec for DR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr6::R`](R) reader structure"]
impl crate::Readable for DR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr6::W`](W) writer structure"]
impl crate::Writable for DR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR6 to value 0"]
impl crate::Resettable for DR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
