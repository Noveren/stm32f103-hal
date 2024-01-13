#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    dr1: DR1,
    dr2: DR2,
    dr3: DR3,
    dr4: DR4,
    dr5: DR5,
    dr6: DR6,
    dr7: DR7,
    dr8: DR8,
    dr9: DR9,
    dr10: DR10,
    rtccr: RTCCR,
    cr: CR,
    csr: CSR,
    _reserved13: [u8; 0x08],
    dr11: DR11,
    dr12: DR12,
    dr13: DR13,
    dr14: DR14,
    dr15: DR15,
    dr16: DR16,
    dr17: DR17,
    dr18: DR18,
    dr19: DR19,
    dr20: DR20,
    dr21: DR21,
    dr22: DR22,
    dr23: DR23,
    dr24: DR24,
    dr25: DR25,
    dr26: DR26,
    dr27: DR27,
    dr28: DR28,
    dr29: DR29,
    dr30: DR30,
    dr31: DR31,
    dr32: DR32,
    dr33: DR33,
    dr34: DR34,
    dr35: DR35,
    dr36: DR36,
    dr37: DR37,
    dr38: DR38,
    dr39: DR39,
    dr40: DR40,
    dr41: DR41,
    dr42: DR42,
}
impl RegisterBlock {
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr1(&self) -> &DR1 {
        &self.dr1
    }
    #[doc = "0x04 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr2(&self) -> &DR2 {
        &self.dr2
    }
    #[doc = "0x08 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr3(&self) -> &DR3 {
        &self.dr3
    }
    #[doc = "0x0c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr4(&self) -> &DR4 {
        &self.dr4
    }
    #[doc = "0x10 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr5(&self) -> &DR5 {
        &self.dr5
    }
    #[doc = "0x14 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr6(&self) -> &DR6 {
        &self.dr6
    }
    #[doc = "0x18 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr7(&self) -> &DR7 {
        &self.dr7
    }
    #[doc = "0x1c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr8(&self) -> &DR8 {
        &self.dr8
    }
    #[doc = "0x20 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr9(&self) -> &DR9 {
        &self.dr9
    }
    #[doc = "0x24 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr10(&self) -> &DR10 {
        &self.dr10
    }
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    #[inline(always)]
    pub const fn rtccr(&self) -> &RTCCR {
        &self.rtccr
    }
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x30 - BKP_CSR control/status register (BKP_CSR)"]
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr11(&self) -> &DR11 {
        &self.dr11
    }
    #[doc = "0x40 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr12(&self) -> &DR12 {
        &self.dr12
    }
    #[doc = "0x44 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr13(&self) -> &DR13 {
        &self.dr13
    }
    #[doc = "0x48 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr14(&self) -> &DR14 {
        &self.dr14
    }
    #[doc = "0x4c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr15(&self) -> &DR15 {
        &self.dr15
    }
    #[doc = "0x50 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr16(&self) -> &DR16 {
        &self.dr16
    }
    #[doc = "0x54 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr17(&self) -> &DR17 {
        &self.dr17
    }
    #[doc = "0x58 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr18(&self) -> &DR18 {
        &self.dr18
    }
    #[doc = "0x5c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr19(&self) -> &DR19 {
        &self.dr19
    }
    #[doc = "0x60 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr20(&self) -> &DR20 {
        &self.dr20
    }
    #[doc = "0x64 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr21(&self) -> &DR21 {
        &self.dr21
    }
    #[doc = "0x68 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr22(&self) -> &DR22 {
        &self.dr22
    }
    #[doc = "0x6c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr23(&self) -> &DR23 {
        &self.dr23
    }
    #[doc = "0x70 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr24(&self) -> &DR24 {
        &self.dr24
    }
    #[doc = "0x74 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr25(&self) -> &DR25 {
        &self.dr25
    }
    #[doc = "0x78 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr26(&self) -> &DR26 {
        &self.dr26
    }
    #[doc = "0x7c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr27(&self) -> &DR27 {
        &self.dr27
    }
    #[doc = "0x80 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr28(&self) -> &DR28 {
        &self.dr28
    }
    #[doc = "0x84 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr29(&self) -> &DR29 {
        &self.dr29
    }
    #[doc = "0x88 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr30(&self) -> &DR30 {
        &self.dr30
    }
    #[doc = "0x8c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr31(&self) -> &DR31 {
        &self.dr31
    }
    #[doc = "0x90 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr32(&self) -> &DR32 {
        &self.dr32
    }
    #[doc = "0x94 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr33(&self) -> &DR33 {
        &self.dr33
    }
    #[doc = "0x98 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr34(&self) -> &DR34 {
        &self.dr34
    }
    #[doc = "0x9c - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr35(&self) -> &DR35 {
        &self.dr35
    }
    #[doc = "0xa0 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr36(&self) -> &DR36 {
        &self.dr36
    }
    #[doc = "0xa4 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr37(&self) -> &DR37 {
        &self.dr37
    }
    #[doc = "0xa8 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr38(&self) -> &DR38 {
        &self.dr38
    }
    #[doc = "0xac - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr39(&self) -> &DR39 {
        &self.dr39
    }
    #[doc = "0xb0 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr40(&self) -> &DR40 {
        &self.dr40
    }
    #[doc = "0xb4 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr41(&self) -> &DR41 {
        &self.dr41
    }
    #[doc = "0xb8 - Backup data register (BKP_DR)"]
    #[inline(always)]
    pub const fn dr42(&self) -> &DR42 {
        &self.dr42
    }
}
#[doc = "DR1 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`]
module"]
pub type DR1 = crate::Reg<dr1::DR1_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr1;
#[doc = "DR2 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`]
module"]
pub type DR2 = crate::Reg<dr2::DR2_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr2;
#[doc = "DR3 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`]
module"]
pub type DR3 = crate::Reg<dr3::DR3_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr3;
#[doc = "DR4 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`]
module"]
pub type DR4 = crate::Reg<dr4::DR4_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr4;
#[doc = "DR5 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`]
module"]
pub type DR5 = crate::Reg<dr5::DR5_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr5;
#[doc = "DR6 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`]
module"]
pub type DR6 = crate::Reg<dr6::DR6_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr6;
#[doc = "DR7 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`]
module"]
pub type DR7 = crate::Reg<dr7::DR7_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr7;
#[doc = "DR8 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`]
module"]
pub type DR8 = crate::Reg<dr8::DR8_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr8;
#[doc = "DR9 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr9`]
module"]
pub type DR9 = crate::Reg<dr9::DR9_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr9;
#[doc = "DR10 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr10`]
module"]
pub type DR10 = crate::Reg<dr10::DR10_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr10;
#[doc = "DR11 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr11`]
module"]
pub type DR11 = crate::Reg<dr11::DR11_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr11;
#[doc = "DR12 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr12`]
module"]
pub type DR12 = crate::Reg<dr12::DR12_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr12;
#[doc = "DR13 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr13`]
module"]
pub type DR13 = crate::Reg<dr13::DR13_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr13;
#[doc = "DR14 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr14`]
module"]
pub type DR14 = crate::Reg<dr14::DR14_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr14;
#[doc = "DR15 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr15`]
module"]
pub type DR15 = crate::Reg<dr15::DR15_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr15;
#[doc = "DR16 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr16`]
module"]
pub type DR16 = crate::Reg<dr16::DR16_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr16;
#[doc = "DR17 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr17`]
module"]
pub type DR17 = crate::Reg<dr17::DR17_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr17;
#[doc = "DR18 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr18`]
module"]
pub type DR18 = crate::Reg<dr18::DR18_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr18;
#[doc = "DR19 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr19`]
module"]
pub type DR19 = crate::Reg<dr19::DR19_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr19;
#[doc = "DR20 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr20`]
module"]
pub type DR20 = crate::Reg<dr20::DR20_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr20;
#[doc = "DR21 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr21`]
module"]
pub type DR21 = crate::Reg<dr21::DR21_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr21;
#[doc = "DR22 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr22`]
module"]
pub type DR22 = crate::Reg<dr22::DR22_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr22;
#[doc = "DR23 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr23`]
module"]
pub type DR23 = crate::Reg<dr23::DR23_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr23;
#[doc = "DR24 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr24`]
module"]
pub type DR24 = crate::Reg<dr24::DR24_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr24;
#[doc = "DR25 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr25`]
module"]
pub type DR25 = crate::Reg<dr25::DR25_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr25;
#[doc = "DR26 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr26`]
module"]
pub type DR26 = crate::Reg<dr26::DR26_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr26;
#[doc = "DR27 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr27`]
module"]
pub type DR27 = crate::Reg<dr27::DR27_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr27;
#[doc = "DR28 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr28`]
module"]
pub type DR28 = crate::Reg<dr28::DR28_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr28;
#[doc = "DR29 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr29`]
module"]
pub type DR29 = crate::Reg<dr29::DR29_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr29;
#[doc = "DR30 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr30`]
module"]
pub type DR30 = crate::Reg<dr30::DR30_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr30;
#[doc = "DR31 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr31`]
module"]
pub type DR31 = crate::Reg<dr31::DR31_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr31;
#[doc = "DR32 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr32`]
module"]
pub type DR32 = crate::Reg<dr32::DR32_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr32;
#[doc = "DR33 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr33`]
module"]
pub type DR33 = crate::Reg<dr33::DR33_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr33;
#[doc = "DR34 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr34`]
module"]
pub type DR34 = crate::Reg<dr34::DR34_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr34;
#[doc = "DR35 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr35`]
module"]
pub type DR35 = crate::Reg<dr35::DR35_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr35;
#[doc = "DR36 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr36`]
module"]
pub type DR36 = crate::Reg<dr36::DR36_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr36;
#[doc = "DR37 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr37`]
module"]
pub type DR37 = crate::Reg<dr37::DR37_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr37;
#[doc = "DR38 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr38`]
module"]
pub type DR38 = crate::Reg<dr38::DR38_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr38;
#[doc = "DR39 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr39`]
module"]
pub type DR39 = crate::Reg<dr39::DR39_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr39;
#[doc = "DR40 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr40`]
module"]
pub type DR40 = crate::Reg<dr40::DR40_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr40;
#[doc = "DR41 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr41`]
module"]
pub type DR41 = crate::Reg<dr41::DR41_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr41;
#[doc = "DR42 (rw) register accessor: Backup data register (BKP_DR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr42`]
module"]
pub type DR42 = crate::Reg<dr42::DR42_SPEC>;
#[doc = "Backup data register (BKP_DR)"]
pub mod dr42;
#[doc = "RTCCR (rw) register accessor: RTC clock calibration register (BKP_RTCCR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccr`]
module"]
pub type RTCCR = crate::Reg<rtccr::RTCCR_SPEC>;
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "CR (rw) register accessor: Backup control register (BKP_CR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "CSR (rw) register accessor: BKP_CSR control/status register (BKP_CSR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "BKP_CSR control/status register (BKP_CSR)"]
pub mod csr;
