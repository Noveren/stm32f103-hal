#[doc = "Register `FS_CID` reader"]
pub type R = crate::R<FS_CID_SPEC>;
#[doc = "Register `FS_CID` writer"]
pub type W = crate::W<FS_CID_SPEC>;
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type PRODUCT_ID_R = crate::FieldReader<u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type PRODUCT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<FS_CID_SPEC> {
        PRODUCT_ID_W::new(self, 0)
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
#[doc = "core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_cid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_cid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_CID_SPEC;
impl crate::RegisterSpec for FS_CID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_cid::R`](R) reader structure"]
impl crate::Readable for FS_CID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fs_cid::W`](W) writer structure"]
impl crate::Writable for FS_CID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_CID to value 0x1000"]
impl crate::Resettable for FS_CID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
