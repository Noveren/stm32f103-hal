#[doc = "Register `MMCRIR` reader"]
pub type R = crate::R<MMCRIR_SPEC>;
#[doc = "Register `MMCRIR` writer"]
pub type W = crate::W<MMCRIR_SPEC>;
#[doc = "Field `RFCES` reader - Received frames CRC error status"]
pub type RFCES_R = crate::BitReader;
#[doc = "Field `RFCES` writer - Received frames CRC error status"]
pub type RFCES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFAES` reader - Received frames alignment error status"]
pub type RFAES_R = crate::BitReader;
#[doc = "Field `RFAES` writer - Received frames alignment error status"]
pub type RFAES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RGUFS` reader - Received Good Unicast Frames Status"]
pub type RGUFS_R = crate::BitReader;
#[doc = "Field `RGUFS` writer - Received Good Unicast Frames Status"]
pub type RGUFS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Received frames CRC error status"]
    #[inline(always)]
    pub fn rfces(&self) -> RFCES_R {
        RFCES_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received frames alignment error status"]
    #[inline(always)]
    pub fn rfaes(&self) -> RFAES_R {
        RFAES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - Received Good Unicast Frames Status"]
    #[inline(always)]
    pub fn rgufs(&self) -> RGUFS_R {
        RGUFS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Received frames CRC error status"]
    #[inline(always)]
    #[must_use]
    pub fn rfces(&mut self) -> RFCES_W<MMCRIR_SPEC> {
        RFCES_W::new(self, 5)
    }
    #[doc = "Bit 6 - Received frames alignment error status"]
    #[inline(always)]
    #[must_use]
    pub fn rfaes(&mut self) -> RFAES_W<MMCRIR_SPEC> {
        RFAES_W::new(self, 6)
    }
    #[doc = "Bit 17 - Received Good Unicast Frames Status"]
    #[inline(always)]
    #[must_use]
    pub fn rgufs(&mut self) -> RGUFS_W<MMCRIR_SPEC> {
        RGUFS_W::new(self, 17)
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
#[doc = "Ethernet MMC receive interrupt register (ETH_MMCRIR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMCRIR_SPEC;
impl crate::RegisterSpec for MMCRIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmcrir::R`](R) reader structure"]
impl crate::Readable for MMCRIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmcrir::W`](W) writer structure"]
impl crate::Writable for MMCRIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCRIR to value 0"]
impl crate::Resettable for MMCRIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
