#[doc = "Register `FS_HCFG` reader"]
pub type R = crate::R<FS_HCFG_SPEC>;
#[doc = "Register `FS_HCFG` writer"]
pub type W = crate::W<FS_HCFG_SPEC>;
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub type FSLSPCS_R = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub type FSLSPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub type FSLSS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<FS_HCFG_SPEC> {
        FSLSPCS_W::new(self, 0)
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
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HCFG_SPEC;
impl crate::RegisterSpec for FS_HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hcfg::R`](R) reader structure"]
impl crate::Readable for FS_HCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_hcfg::W`](W) writer structure"]
impl crate::Writable for FS_HCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_HCFG to value 0"]
impl crate::Resettable for FS_HCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
