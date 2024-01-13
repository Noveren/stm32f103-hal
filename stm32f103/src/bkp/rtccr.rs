#[doc = "Register `RTCCR` reader"]
pub type R = crate::R<RTCCR_SPEC>;
#[doc = "Register `RTCCR` writer"]
pub type W = crate::W<RTCCR_SPEC>;
#[doc = "Field `CAL` reader - Calibration value"]
pub type CAL_R = crate::FieldReader;
#[doc = "Field `CAL` writer - Calibration value"]
pub type CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CCO` reader - Calibration Clock Output"]
pub type CCO_R = crate::BitReader;
#[doc = "Field `CCO` writer - Calibration Clock Output"]
pub type CCO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASOE` reader - Alarm or second output enable"]
pub type ASOE_R = crate::BitReader;
#[doc = "Field `ASOE` writer - Alarm or second output enable"]
pub type ASOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASOS` reader - Alarm or second output selection"]
pub type ASOS_R = crate::BitReader;
#[doc = "Field `ASOS` writer - Alarm or second output selection"]
pub type ASOS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    pub fn cco(&self) -> CCO_R {
        CCO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    pub fn asoe(&self) -> ASOE_R {
        ASOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    pub fn asos(&self) -> ASOS_R {
        ASOS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<RTCCR_SPEC> {
        CAL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn cco(&mut self) -> CCO_W<RTCCR_SPEC> {
        CCO_W::new(self, 7)
    }
    #[doc = "Bit 8 - Alarm or second output enable"]
    #[inline(always)]
    #[must_use]
    pub fn asoe(&mut self) -> ASOE_W<RTCCR_SPEC> {
        ASOE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm or second output selection"]
    #[inline(always)]
    #[must_use]
    pub fn asos(&mut self) -> ASOS_W<RTCCR_SPEC> {
        ASOS_W::new(self, 9)
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
#[doc = "RTC clock calibration register (BKP_RTCCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCCR_SPEC;
impl crate::RegisterSpec for RTCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtccr::R`](R) reader structure"]
impl crate::Readable for RTCCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtccr::W`](W) writer structure"]
impl crate::Writable for RTCCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCR to value 0"]
impl crate::Resettable for RTCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
