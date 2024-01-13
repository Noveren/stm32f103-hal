#[doc = "Register `MACMIIDR` reader"]
pub type R = crate::R<MACMIIDR_SPEC>;
#[doc = "Register `MACMIIDR` writer"]
pub type W = crate::W<MACMIIDR_SPEC>;
#[doc = "Field `MD` reader - MII data"]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII data"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<MACMIIDR_SPEC> {
        MD_W::new(self, 0)
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
#[doc = "Ethernet MAC MII data register (ETH_MACMIIDR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIDR_SPEC;
impl crate::RegisterSpec for MACMIIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiidr::R`](R) reader structure"]
impl crate::Readable for MACMIIDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmiidr::W`](W) writer structure"]
impl crate::Writable for MACMIIDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMIIDR to value 0"]
impl crate::Resettable for MACMIIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
