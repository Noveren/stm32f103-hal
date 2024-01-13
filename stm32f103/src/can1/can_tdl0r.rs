#[doc = "Register `CAN_TDL0R` reader"]
pub type R = crate::R<CAN_TDL0R_SPEC>;
#[doc = "Register `CAN_TDL0R` writer"]
pub type W = crate::W<CAN_TDL0R_SPEC>;
#[doc = "Field `DATA0` reader - DATA0"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA0` writer - DATA0"]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - DATA2"]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA2` writer - DATA2"]
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - DATA3"]
pub type DATA3_R = crate::FieldReader;
#[doc = "Field `DATA3` writer - DATA3"]
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA0"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<CAN_TDL0R_SPEC> {
        DATA0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<CAN_TDL0R_SPEC> {
        DATA1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DATA2"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<CAN_TDL0R_SPEC> {
        DATA2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - DATA3"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<CAN_TDL0R_SPEC> {
        DATA3_W::new(self, 24)
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
#[doc = "CAN_TDL0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdl0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdl0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAN_TDL0R_SPEC;
impl crate::RegisterSpec for CAN_TDL0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_tdl0r::R`](R) reader structure"]
impl crate::Readable for CAN_TDL0R_SPEC {}
#[doc = "`write(|w| ..)` method takes [`can_tdl0r::W`](W) writer structure"]
impl crate::Writable for CAN_TDL0R_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAN_TDL0R to value 0"]
impl crate::Resettable for CAN_TDL0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
