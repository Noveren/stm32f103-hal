#[doc = "Register `PATT2` reader"]
pub type R = crate::R<PATT2_SPEC>;
#[doc = "Register `PATT2` writer"]
pub type W = crate::W<PATT2_SPEC>;
#[doc = "Field `ATTSETx` reader - Attribute memory x setup time"]
pub type ATTSETX_R = crate::FieldReader;
#[doc = "Field `ATTSETx` writer - Attribute memory x setup time"]
pub type ATTSETX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTWAITx` reader - Attribute memory x wait time"]
pub type ATTWAITX_R = crate::FieldReader;
#[doc = "Field `ATTWAITx` writer - Attribute memory x wait time"]
pub type ATTWAITX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHOLDx` reader - Attribute memory x hold time"]
pub type ATTHOLDX_R = crate::FieldReader;
#[doc = "Field `ATTHOLDx` writer - Attribute memory x hold time"]
pub type ATTHOLDX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ATTHIZx` reader - Attribute memory x databus HiZ time"]
pub type ATTHIZX_R = crate::FieldReader;
#[doc = "Field `ATTHIZx` writer - Attribute memory x databus HiZ time"]
pub type ATTHIZX_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    pub fn attsetx(&self) -> ATTSETX_R {
        ATTSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    pub fn attwaitx(&self) -> ATTWAITX_R {
        ATTWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    pub fn attholdx(&self) -> ATTHOLDX_R {
        ATTHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    pub fn atthizx(&self) -> ATTHIZX_R {
        ATTHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Attribute memory x setup time"]
    #[inline(always)]
    #[must_use]
    pub fn attsetx(&mut self) -> ATTSETX_W<PATT2_SPEC> {
        ATTSETX_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Attribute memory x wait time"]
    #[inline(always)]
    #[must_use]
    pub fn attwaitx(&mut self) -> ATTWAITX_W<PATT2_SPEC> {
        ATTWAITX_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Attribute memory x hold time"]
    #[inline(always)]
    #[must_use]
    pub fn attholdx(&mut self) -> ATTHOLDX_W<PATT2_SPEC> {
        ATTHOLDX_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Attribute memory x databus HiZ time"]
    #[inline(always)]
    #[must_use]
    pub fn atthizx(&mut self) -> ATTHIZX_W<PATT2_SPEC> {
        ATTHIZX_W::new(self, 24)
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
#[doc = "Attribute memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`patt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`patt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PATT2_SPEC;
impl crate::RegisterSpec for PATT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`patt2::R`](R) reader structure"]
impl crate::Readable for PATT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`patt2::W`](W) writer structure"]
impl crate::Writable for PATT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PATT2 to value 0xfcfc_fcfc"]
impl crate::Resettable for PATT2_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}
