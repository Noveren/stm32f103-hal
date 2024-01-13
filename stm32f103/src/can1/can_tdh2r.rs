#[doc = "Register `CAN_TDH2R` reader"]
pub type R = crate::R<CAN_TDH2R_SPEC>;
#[doc = "Register `CAN_TDH2R` writer"]
pub type W = crate::W<CAN_TDH2R_SPEC>;
#[doc = "Field `DATA4` reader - DATA4"]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - DATA4"]
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA5` reader - DATA5"]
pub type DATA5_R = crate::FieldReader;
#[doc = "Field `DATA5` writer - DATA5"]
pub type DATA5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA6` reader - DATA6"]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - DATA6"]
pub type DATA6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA7` reader - DATA7"]
pub type DATA7_R = crate::FieldReader;
#[doc = "Field `DATA7` writer - DATA7"]
pub type DATA7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA4"]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<CAN_TDH2R_SPEC> {
        DATA4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA5"]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<CAN_TDH2R_SPEC> {
        DATA5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA6"]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<CAN_TDH2R_SPEC> {
        DATA6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA7"]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<CAN_TDH2R_SPEC> {
        DATA7_W::new(self, 24)
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
#[doc = "CAN_TDH2R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdh2r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdh2r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_TDH2R_SPEC;
impl crate::RegisterSpec for CAN_TDH2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tdh2r::R`](R) reader structure"]
impl crate::Readable for CAN_TDH2R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_tdh2r::W`](W) writer structure"]
impl crate::Writable for CAN_TDH2R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_TDH2R to value 0"]
impl crate::Resettable for CAN_TDH2R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
