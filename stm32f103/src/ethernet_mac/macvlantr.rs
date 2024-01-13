#[doc = "Register `MACVLANTR` reader"]
pub type R = crate::R<MACVLANTR_SPEC>;
#[doc = "Register `MACVLANTR` writer"]
pub type W = crate::W<MACVLANTR_SPEC>;
#[doc = "Field `VLANTI` reader - VLAN tag identifier (for receive frames)"]
pub type VLANTI_R = crate::FieldReader<u16>;
#[doc = "Field `VLANTI` writer - VLAN tag identifier (for receive frames)"]
pub type VLANTI_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLANTC` reader - 12-bit VLAN tag comparison"]
pub type VLANTC_R = crate::BitReader;
#[doc = "Field `VLANTC` writer - 12-bit VLAN tag comparison"]
pub type VLANTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    pub fn vlanti(&self) -> VLANTI_R {
        VLANTI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    pub fn vlantc(&self) -> VLANTC_R {
        VLANTC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLAN tag identifier (for receive frames)"]
    #[inline(always)]
    #[must_use]
    pub fn vlanti(&mut self) -> VLANTI_W<MACVLANTR_SPEC> {
        VLANTI_W::new(self, 0)
    }
    #[doc = "Bit 16 - 12-bit VLAN tag comparison"]
    #[inline(always)]
    #[must_use]
    pub fn vlantc(&mut self) -> VLANTC_W<MACVLANTR_SPEC> {
        VLANTC_W::new(self, 16)
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
#[doc = "Ethernet MAC VLAN tag register (ETH_MACVLANTR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlantr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlantr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACVLANTR_SPEC;
impl crate::RegisterSpec for MACVLANTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvlantr::R`](R) reader structure"]
impl crate::Readable for MACVLANTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macvlantr::W`](W) writer structure"]
impl crate::Writable for MACVLANTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACVLANTR to value 0"]
impl crate::Resettable for MACVLANTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
