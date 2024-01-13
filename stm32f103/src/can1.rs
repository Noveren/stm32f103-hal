#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    can_mcr: CAN_MCR,
    can_msr: CAN_MSR,
    can_tsr: CAN_TSR,
    can_rf0r: CAN_RF0R,
    can_rf1r: CAN_RF1R,
    can_ier: CAN_IER,
    can_esr: CAN_ESR,
    can_btr: CAN_BTR,
    _reserved8: [u8; 0x0160],
    can_ti0r: CAN_TI0R,
    can_tdt0r: CAN_TDT0R,
    can_tdl0r: CAN_TDL0R,
    can_tdh0r: CAN_TDH0R,
    can_ti1r: CAN_TI1R,
    can_tdt1r: CAN_TDT1R,
    can_tdl1r: CAN_TDL1R,
    can_tdh1r: CAN_TDH1R,
    can_ti2r: CAN_TI2R,
    can_tdt2r: CAN_TDT2R,
    can_tdl2r: CAN_TDL2R,
    can_tdh2r: CAN_TDH2R,
    can_ri0r: CAN_RI0R,
    can_rdt0r: CAN_RDT0R,
    can_rdl0r: CAN_RDL0R,
    can_rdh0r: CAN_RDH0R,
    can_ri1r: CAN_RI1R,
    can_rdt1r: CAN_RDT1R,
    can_rdl1r: CAN_RDL1R,
    can_rdh1r: CAN_RDH1R,
    _reserved28: [u8; 0x30],
    can_fmr: CAN_FMR,
    can_fm1r: CAN_FM1R,
    _reserved30: [u8; 0x04],
    can_fs1r: CAN_FS1R,
    _reserved31: [u8; 0x04],
    can_ffa1r: CAN_FFA1R,
    _reserved32: [u8; 0x04],
    can_fa1r: CAN_FA1R,
    _reserved33: [u8; 0x20],
    f0r1: F0R1,
    f0r2: F0R2,
    f1r1: F1R1,
    f1r2: F1R2,
    f2r1: F2R1,
    f2r2: F2R2,
    f3r1: F3R1,
    f3r2: F3R2,
    f4r1: F4R1,
    f4r2: F4R2,
    f5r1: F5R1,
    f5r2: F5R2,
    f6r1: F6R1,
    f6r2: F6R2,
    f7r1: F7R1,
    f7r2: F7R2,
    f8r1: F8R1,
    f8r2: F8R2,
    f9r1: F9R1,
    f9r2: F9R2,
    f10r1: F10R1,
    f10r2: F10R2,
    f11r1: F11R1,
    f11r2: F11R2,
    f12r1: F12R1,
    f12r2: F12R2,
    f13r1: F13R1,
    f13r2: F13R2,
}
impl RegisterBlock {
    #[doc = "0x00 - CAN_MCR"]
    #[inline(always)]
    pub const fn can_mcr(&self) -> &CAN_MCR {
        &self.can_mcr
    }
    #[doc = "0x04 - CAN_MSR"]
    #[inline(always)]
    pub const fn can_msr(&self) -> &CAN_MSR {
        &self.can_msr
    }
    #[doc = "0x08 - CAN_TSR"]
    #[inline(always)]
    pub const fn can_tsr(&self) -> &CAN_TSR {
        &self.can_tsr
    }
    #[doc = "0x0c - CAN_RF0R"]
    #[inline(always)]
    pub const fn can_rf0r(&self) -> &CAN_RF0R {
        &self.can_rf0r
    }
    #[doc = "0x10 - CAN_RF1R"]
    #[inline(always)]
    pub const fn can_rf1r(&self) -> &CAN_RF1R {
        &self.can_rf1r
    }
    #[doc = "0x14 - CAN_IER"]
    #[inline(always)]
    pub const fn can_ier(&self) -> &CAN_IER {
        &self.can_ier
    }
    #[doc = "0x18 - CAN_ESR"]
    #[inline(always)]
    pub const fn can_esr(&self) -> &CAN_ESR {
        &self.can_esr
    }
    #[doc = "0x1c - CAN_BTR"]
    #[inline(always)]
    pub const fn can_btr(&self) -> &CAN_BTR {
        &self.can_btr
    }
    #[doc = "0x180 - CAN_TI0R"]
    #[inline(always)]
    pub const fn can_ti0r(&self) -> &CAN_TI0R {
        &self.can_ti0r
    }
    #[doc = "0x184 - CAN_TDT0R"]
    #[inline(always)]
    pub const fn can_tdt0r(&self) -> &CAN_TDT0R {
        &self.can_tdt0r
    }
    #[doc = "0x188 - CAN_TDL0R"]
    #[inline(always)]
    pub const fn can_tdl0r(&self) -> &CAN_TDL0R {
        &self.can_tdl0r
    }
    #[doc = "0x18c - CAN_TDH0R"]
    #[inline(always)]
    pub const fn can_tdh0r(&self) -> &CAN_TDH0R {
        &self.can_tdh0r
    }
    #[doc = "0x190 - CAN_TI1R"]
    #[inline(always)]
    pub const fn can_ti1r(&self) -> &CAN_TI1R {
        &self.can_ti1r
    }
    #[doc = "0x194 - CAN_TDT1R"]
    #[inline(always)]
    pub const fn can_tdt1r(&self) -> &CAN_TDT1R {
        &self.can_tdt1r
    }
    #[doc = "0x198 - CAN_TDL1R"]
    #[inline(always)]
    pub const fn can_tdl1r(&self) -> &CAN_TDL1R {
        &self.can_tdl1r
    }
    #[doc = "0x19c - CAN_TDH1R"]
    #[inline(always)]
    pub const fn can_tdh1r(&self) -> &CAN_TDH1R {
        &self.can_tdh1r
    }
    #[doc = "0x1a0 - CAN_TI2R"]
    #[inline(always)]
    pub const fn can_ti2r(&self) -> &CAN_TI2R {
        &self.can_ti2r
    }
    #[doc = "0x1a4 - CAN_TDT2R"]
    #[inline(always)]
    pub const fn can_tdt2r(&self) -> &CAN_TDT2R {
        &self.can_tdt2r
    }
    #[doc = "0x1a8 - CAN_TDL2R"]
    #[inline(always)]
    pub const fn can_tdl2r(&self) -> &CAN_TDL2R {
        &self.can_tdl2r
    }
    #[doc = "0x1ac - CAN_TDH2R"]
    #[inline(always)]
    pub const fn can_tdh2r(&self) -> &CAN_TDH2R {
        &self.can_tdh2r
    }
    #[doc = "0x1b0 - CAN_RI0R"]
    #[inline(always)]
    pub const fn can_ri0r(&self) -> &CAN_RI0R {
        &self.can_ri0r
    }
    #[doc = "0x1b4 - CAN_RDT0R"]
    #[inline(always)]
    pub const fn can_rdt0r(&self) -> &CAN_RDT0R {
        &self.can_rdt0r
    }
    #[doc = "0x1b8 - CAN_RDL0R"]
    #[inline(always)]
    pub const fn can_rdl0r(&self) -> &CAN_RDL0R {
        &self.can_rdl0r
    }
    #[doc = "0x1bc - CAN_RDH0R"]
    #[inline(always)]
    pub const fn can_rdh0r(&self) -> &CAN_RDH0R {
        &self.can_rdh0r
    }
    #[doc = "0x1c0 - CAN_RI1R"]
    #[inline(always)]
    pub const fn can_ri1r(&self) -> &CAN_RI1R {
        &self.can_ri1r
    }
    #[doc = "0x1c4 - CAN_RDT1R"]
    #[inline(always)]
    pub const fn can_rdt1r(&self) -> &CAN_RDT1R {
        &self.can_rdt1r
    }
    #[doc = "0x1c8 - CAN_RDL1R"]
    #[inline(always)]
    pub const fn can_rdl1r(&self) -> &CAN_RDL1R {
        &self.can_rdl1r
    }
    #[doc = "0x1cc - CAN_RDH1R"]
    #[inline(always)]
    pub const fn can_rdh1r(&self) -> &CAN_RDH1R {
        &self.can_rdh1r
    }
    #[doc = "0x200 - CAN_FMR"]
    #[inline(always)]
    pub const fn can_fmr(&self) -> &CAN_FMR {
        &self.can_fmr
    }
    #[doc = "0x204 - CAN_FM1R"]
    #[inline(always)]
    pub const fn can_fm1r(&self) -> &CAN_FM1R {
        &self.can_fm1r
    }
    #[doc = "0x20c - CAN_FS1R"]
    #[inline(always)]
    pub const fn can_fs1r(&self) -> &CAN_FS1R {
        &self.can_fs1r
    }
    #[doc = "0x214 - CAN_FFA1R"]
    #[inline(always)]
    pub const fn can_ffa1r(&self) -> &CAN_FFA1R {
        &self.can_ffa1r
    }
    #[doc = "0x21c - CAN_FA1R"]
    #[inline(always)]
    pub const fn can_fa1r(&self) -> &CAN_FA1R {
        &self.can_fa1r
    }
    #[doc = "0x240 - Filter bank 0 register 1"]
    #[inline(always)]
    pub const fn f0r1(&self) -> &F0R1 {
        &self.f0r1
    }
    #[doc = "0x244 - Filter bank 0 register 2"]
    #[inline(always)]
    pub const fn f0r2(&self) -> &F0R2 {
        &self.f0r2
    }
    #[doc = "0x248 - Filter bank 1 register 1"]
    #[inline(always)]
    pub const fn f1r1(&self) -> &F1R1 {
        &self.f1r1
    }
    #[doc = "0x24c - Filter bank 1 register 2"]
    #[inline(always)]
    pub const fn f1r2(&self) -> &F1R2 {
        &self.f1r2
    }
    #[doc = "0x250 - Filter bank 2 register 1"]
    #[inline(always)]
    pub const fn f2r1(&self) -> &F2R1 {
        &self.f2r1
    }
    #[doc = "0x254 - Filter bank 2 register 2"]
    #[inline(always)]
    pub const fn f2r2(&self) -> &F2R2 {
        &self.f2r2
    }
    #[doc = "0x258 - Filter bank 3 register 1"]
    #[inline(always)]
    pub const fn f3r1(&self) -> &F3R1 {
        &self.f3r1
    }
    #[doc = "0x25c - Filter bank 3 register 2"]
    #[inline(always)]
    pub const fn f3r2(&self) -> &F3R2 {
        &self.f3r2
    }
    #[doc = "0x260 - Filter bank 4 register 1"]
    #[inline(always)]
    pub const fn f4r1(&self) -> &F4R1 {
        &self.f4r1
    }
    #[doc = "0x264 - Filter bank 4 register 2"]
    #[inline(always)]
    pub const fn f4r2(&self) -> &F4R2 {
        &self.f4r2
    }
    #[doc = "0x268 - Filter bank 5 register 1"]
    #[inline(always)]
    pub const fn f5r1(&self) -> &F5R1 {
        &self.f5r1
    }
    #[doc = "0x26c - Filter bank 5 register 2"]
    #[inline(always)]
    pub const fn f5r2(&self) -> &F5R2 {
        &self.f5r2
    }
    #[doc = "0x270 - Filter bank 6 register 1"]
    #[inline(always)]
    pub const fn f6r1(&self) -> &F6R1 {
        &self.f6r1
    }
    #[doc = "0x274 - Filter bank 6 register 2"]
    #[inline(always)]
    pub const fn f6r2(&self) -> &F6R2 {
        &self.f6r2
    }
    #[doc = "0x278 - Filter bank 7 register 1"]
    #[inline(always)]
    pub const fn f7r1(&self) -> &F7R1 {
        &self.f7r1
    }
    #[doc = "0x27c - Filter bank 7 register 2"]
    #[inline(always)]
    pub const fn f7r2(&self) -> &F7R2 {
        &self.f7r2
    }
    #[doc = "0x280 - Filter bank 8 register 1"]
    #[inline(always)]
    pub const fn f8r1(&self) -> &F8R1 {
        &self.f8r1
    }
    #[doc = "0x284 - Filter bank 8 register 2"]
    #[inline(always)]
    pub const fn f8r2(&self) -> &F8R2 {
        &self.f8r2
    }
    #[doc = "0x288 - Filter bank 9 register 1"]
    #[inline(always)]
    pub const fn f9r1(&self) -> &F9R1 {
        &self.f9r1
    }
    #[doc = "0x28c - Filter bank 9 register 2"]
    #[inline(always)]
    pub const fn f9r2(&self) -> &F9R2 {
        &self.f9r2
    }
    #[doc = "0x290 - Filter bank 10 register 1"]
    #[inline(always)]
    pub const fn f10r1(&self) -> &F10R1 {
        &self.f10r1
    }
    #[doc = "0x294 - Filter bank 10 register 2"]
    #[inline(always)]
    pub const fn f10r2(&self) -> &F10R2 {
        &self.f10r2
    }
    #[doc = "0x298 - Filter bank 11 register 1"]
    #[inline(always)]
    pub const fn f11r1(&self) -> &F11R1 {
        &self.f11r1
    }
    #[doc = "0x29c - Filter bank 11 register 2"]
    #[inline(always)]
    pub const fn f11r2(&self) -> &F11R2 {
        &self.f11r2
    }
    #[doc = "0x2a0 - Filter bank 4 register 1"]
    #[inline(always)]
    pub const fn f12r1(&self) -> &F12R1 {
        &self.f12r1
    }
    #[doc = "0x2a4 - Filter bank 12 register 2"]
    #[inline(always)]
    pub const fn f12r2(&self) -> &F12R2 {
        &self.f12r2
    }
    #[doc = "0x2a8 - Filter bank 13 register 1"]
    #[inline(always)]
    pub const fn f13r1(&self) -> &F13R1 {
        &self.f13r1
    }
    #[doc = "0x2ac - Filter bank 13 register 2"]
    #[inline(always)]
    pub const fn f13r2(&self) -> &F13R2 {
        &self.f13r2
    }
}
#[doc = "CAN_MCR (rw) register accessor: CAN_MCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_mcr`]
module"]
pub type CAN_MCR = crate::Reg<can_mcr::CAN_MCR_SPEC>;
#[doc = "CAN_MCR"]
pub mod can_mcr;
#[doc = "CAN_MSR (rw) register accessor: CAN_MSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_msr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_msr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_msr`]
module"]
pub type CAN_MSR = crate::Reg<can_msr::CAN_MSR_SPEC>;
#[doc = "CAN_MSR"]
pub mod can_msr;
#[doc = "CAN_TSR (rw) register accessor: CAN_TSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tsr`]
module"]
pub type CAN_TSR = crate::Reg<can_tsr::CAN_TSR_SPEC>;
#[doc = "CAN_TSR"]
pub mod can_tsr;
#[doc = "CAN_RF0R (rw) register accessor: CAN_RF0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rf0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_rf0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rf0r`]
module"]
pub type CAN_RF0R = crate::Reg<can_rf0r::CAN_RF0R_SPEC>;
#[doc = "CAN_RF0R"]
pub mod can_rf0r;
#[doc = "CAN_RF1R (rw) register accessor: CAN_RF1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rf1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_rf1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rf1r`]
module"]
pub type CAN_RF1R = crate::Reg<can_rf1r::CAN_RF1R_SPEC>;
#[doc = "CAN_RF1R"]
pub mod can_rf1r;
#[doc = "CAN_IER (rw) register accessor: CAN_IER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ier`]
module"]
pub type CAN_IER = crate::Reg<can_ier::CAN_IER_SPEC>;
#[doc = "CAN_IER"]
pub mod can_ier;
#[doc = "CAN_ESR (rw) register accessor: CAN_ESR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_esr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_esr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_esr`]
module"]
pub type CAN_ESR = crate::Reg<can_esr::CAN_ESR_SPEC>;
#[doc = "CAN_ESR"]
pub mod can_esr;
#[doc = "CAN_BTR (rw) register accessor: CAN_BTR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_btr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_btr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_btr`]
module"]
pub type CAN_BTR = crate::Reg<can_btr::CAN_BTR_SPEC>;
#[doc = "CAN_BTR"]
pub mod can_btr;
#[doc = "CAN_TI0R (rw) register accessor: CAN_TI0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ti0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ti0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ti0r`]
module"]
pub type CAN_TI0R = crate::Reg<can_ti0r::CAN_TI0R_SPEC>;
#[doc = "CAN_TI0R"]
pub mod can_ti0r;
#[doc = "CAN_TDT0R (rw) register accessor: CAN_TDT0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdt0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdt0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdt0r`]
module"]
pub type CAN_TDT0R = crate::Reg<can_tdt0r::CAN_TDT0R_SPEC>;
#[doc = "CAN_TDT0R"]
pub mod can_tdt0r;
#[doc = "CAN_TDL0R (rw) register accessor: CAN_TDL0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdl0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdl0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdl0r`]
module"]
pub type CAN_TDL0R = crate::Reg<can_tdl0r::CAN_TDL0R_SPEC>;
#[doc = "CAN_TDL0R"]
pub mod can_tdl0r;
#[doc = "CAN_TDH0R (rw) register accessor: CAN_TDH0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdh0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdh0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdh0r`]
module"]
pub type CAN_TDH0R = crate::Reg<can_tdh0r::CAN_TDH0R_SPEC>;
#[doc = "CAN_TDH0R"]
pub mod can_tdh0r;
#[doc = "CAN_TI1R (rw) register accessor: CAN_TI1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ti1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ti1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ti1r`]
module"]
pub type CAN_TI1R = crate::Reg<can_ti1r::CAN_TI1R_SPEC>;
#[doc = "CAN_TI1R"]
pub mod can_ti1r;
#[doc = "CAN_TDT1R (rw) register accessor: CAN_TDT1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdt1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdt1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdt1r`]
module"]
pub type CAN_TDT1R = crate::Reg<can_tdt1r::CAN_TDT1R_SPEC>;
#[doc = "CAN_TDT1R"]
pub mod can_tdt1r;
#[doc = "CAN_TDL1R (rw) register accessor: CAN_TDL1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdl1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdl1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdl1r`]
module"]
pub type CAN_TDL1R = crate::Reg<can_tdl1r::CAN_TDL1R_SPEC>;
#[doc = "CAN_TDL1R"]
pub mod can_tdl1r;
#[doc = "CAN_TDH1R (rw) register accessor: CAN_TDH1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdh1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdh1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdh1r`]
module"]
pub type CAN_TDH1R = crate::Reg<can_tdh1r::CAN_TDH1R_SPEC>;
#[doc = "CAN_TDH1R"]
pub mod can_tdh1r;
#[doc = "CAN_TI2R (rw) register accessor: CAN_TI2R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ti2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ti2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ti2r`]
module"]
pub type CAN_TI2R = crate::Reg<can_ti2r::CAN_TI2R_SPEC>;
#[doc = "CAN_TI2R"]
pub mod can_ti2r;
#[doc = "CAN_TDT2R (rw) register accessor: CAN_TDT2R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdt2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdt2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdt2r`]
module"]
pub type CAN_TDT2R = crate::Reg<can_tdt2r::CAN_TDT2R_SPEC>;
#[doc = "CAN_TDT2R"]
pub mod can_tdt2r;
#[doc = "CAN_TDL2R (rw) register accessor: CAN_TDL2R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdl2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdl2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdl2r`]
module"]
pub type CAN_TDL2R = crate::Reg<can_tdl2r::CAN_TDL2R_SPEC>;
#[doc = "CAN_TDL2R"]
pub mod can_tdl2r;
#[doc = "CAN_TDH2R (rw) register accessor: CAN_TDH2R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_tdh2r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_tdh2r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_tdh2r`]
module"]
pub type CAN_TDH2R = crate::Reg<can_tdh2r::CAN_TDH2R_SPEC>;
#[doc = "CAN_TDH2R"]
pub mod can_tdh2r;
#[doc = "CAN_RI0R (r) register accessor: CAN_RI0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ri0r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ri0r`]
module"]
pub type CAN_RI0R = crate::Reg<can_ri0r::CAN_RI0R_SPEC>;
#[doc = "CAN_RI0R"]
pub mod can_ri0r;
#[doc = "CAN_RDT0R (r) register accessor: CAN_RDT0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdt0r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdt0r`]
module"]
pub type CAN_RDT0R = crate::Reg<can_rdt0r::CAN_RDT0R_SPEC>;
#[doc = "CAN_RDT0R"]
pub mod can_rdt0r;
#[doc = "CAN_RDL0R (r) register accessor: CAN_RDL0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdl0r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdl0r`]
module"]
pub type CAN_RDL0R = crate::Reg<can_rdl0r::CAN_RDL0R_SPEC>;
#[doc = "CAN_RDL0R"]
pub mod can_rdl0r;
#[doc = "CAN_RDH0R (r) register accessor: CAN_RDH0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdh0r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdh0r`]
module"]
pub type CAN_RDH0R = crate::Reg<can_rdh0r::CAN_RDH0R_SPEC>;
#[doc = "CAN_RDH0R"]
pub mod can_rdh0r;
#[doc = "CAN_RI1R (r) register accessor: CAN_RI1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ri1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ri1r`]
module"]
pub type CAN_RI1R = crate::Reg<can_ri1r::CAN_RI1R_SPEC>;
#[doc = "CAN_RI1R"]
pub mod can_ri1r;
#[doc = "CAN_RDT1R (r) register accessor: CAN_RDT1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdt1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdt1r`]
module"]
pub type CAN_RDT1R = crate::Reg<can_rdt1r::CAN_RDT1R_SPEC>;
#[doc = "CAN_RDT1R"]
pub mod can_rdt1r;
#[doc = "CAN_RDL1R (r) register accessor: CAN_RDL1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdl1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdl1r`]
module"]
pub type CAN_RDL1R = crate::Reg<can_rdl1r::CAN_RDL1R_SPEC>;
#[doc = "CAN_RDL1R"]
pub mod can_rdl1r;
#[doc = "CAN_RDH1R (r) register accessor: CAN_RDH1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_rdh1r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_rdh1r`]
module"]
pub type CAN_RDH1R = crate::Reg<can_rdh1r::CAN_RDH1R_SPEC>;
#[doc = "CAN_RDH1R"]
pub mod can_rdh1r;
#[doc = "CAN_FMR (rw) register accessor: CAN_FMR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fmr`]
module"]
pub type CAN_FMR = crate::Reg<can_fmr::CAN_FMR_SPEC>;
#[doc = "CAN_FMR"]
pub mod can_fmr;
#[doc = "CAN_FM1R (rw) register accessor: CAN_FM1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fm1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fm1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fm1r`]
module"]
pub type CAN_FM1R = crate::Reg<can_fm1r::CAN_FM1R_SPEC>;
#[doc = "CAN_FM1R"]
pub mod can_fm1r;
#[doc = "CAN_FS1R (rw) register accessor: CAN_FS1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fs1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fs1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fs1r`]
module"]
pub type CAN_FS1R = crate::Reg<can_fs1r::CAN_FS1R_SPEC>;
#[doc = "CAN_FS1R"]
pub mod can_fs1r;
#[doc = "CAN_FFA1R (rw) register accessor: CAN_FFA1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_ffa1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_ffa1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ffa1r`]
module"]
pub type CAN_FFA1R = crate::Reg<can_ffa1r::CAN_FFA1R_SPEC>;
#[doc = "CAN_FFA1R"]
pub mod can_ffa1r;
#[doc = "CAN_FA1R (rw) register accessor: CAN_FA1R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can_fa1r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can_fa1r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_fa1r`]
module"]
pub type CAN_FA1R = crate::Reg<can_fa1r::CAN_FA1R_SPEC>;
#[doc = "CAN_FA1R"]
pub mod can_fa1r;
#[doc = "F0R1 (rw) register accessor: Filter bank 0 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0r1`]
module"]
pub type F0R1 = crate::Reg<f0r1::F0R1_SPEC>;
#[doc = "Filter bank 0 register 1"]
pub mod f0r1;
#[doc = "F0R2 (rw) register accessor: Filter bank 0 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0r2`]
module"]
pub type F0R2 = crate::Reg<f0r2::F0R2_SPEC>;
#[doc = "Filter bank 0 register 2"]
pub mod f0r2;
#[doc = "F1R1 (rw) register accessor: Filter bank 1 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1r1`]
module"]
pub type F1R1 = crate::Reg<f1r1::F1R1_SPEC>;
#[doc = "Filter bank 1 register 1"]
pub mod f1r1;
#[doc = "F1R2 (rw) register accessor: Filter bank 1 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1r2`]
module"]
pub type F1R2 = crate::Reg<f1r2::F1R2_SPEC>;
#[doc = "Filter bank 1 register 2"]
pub mod f1r2;
#[doc = "F2R1 (rw) register accessor: Filter bank 2 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2r1`]
module"]
pub type F2R1 = crate::Reg<f2r1::F2R1_SPEC>;
#[doc = "Filter bank 2 register 1"]
pub mod f2r1;
#[doc = "F2R2 (rw) register accessor: Filter bank 2 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2r2`]
module"]
pub type F2R2 = crate::Reg<f2r2::F2R2_SPEC>;
#[doc = "Filter bank 2 register 2"]
pub mod f2r2;
#[doc = "F3R1 (rw) register accessor: Filter bank 3 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3r1`]
module"]
pub type F3R1 = crate::Reg<f3r1::F3R1_SPEC>;
#[doc = "Filter bank 3 register 1"]
pub mod f3r1;
#[doc = "F3R2 (rw) register accessor: Filter bank 3 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3r2`]
module"]
pub type F3R2 = crate::Reg<f3r2::F3R2_SPEC>;
#[doc = "Filter bank 3 register 2"]
pub mod f3r2;
#[doc = "F4R1 (rw) register accessor: Filter bank 4 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4r1`]
module"]
pub type F4R1 = crate::Reg<f4r1::F4R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f4r1;
#[doc = "F4R2 (rw) register accessor: Filter bank 4 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4r2`]
module"]
pub type F4R2 = crate::Reg<f4r2::F4R2_SPEC>;
#[doc = "Filter bank 4 register 2"]
pub mod f4r2;
#[doc = "F5R1 (rw) register accessor: Filter bank 5 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5r1`]
module"]
pub type F5R1 = crate::Reg<f5r1::F5R1_SPEC>;
#[doc = "Filter bank 5 register 1"]
pub mod f5r1;
#[doc = "F5R2 (rw) register accessor: Filter bank 5 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5r2`]
module"]
pub type F5R2 = crate::Reg<f5r2::F5R2_SPEC>;
#[doc = "Filter bank 5 register 2"]
pub mod f5r2;
#[doc = "F6R1 (rw) register accessor: Filter bank 6 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6r1`]
module"]
pub type F6R1 = crate::Reg<f6r1::F6R1_SPEC>;
#[doc = "Filter bank 6 register 1"]
pub mod f6r1;
#[doc = "F6R2 (rw) register accessor: Filter bank 6 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6r2`]
module"]
pub type F6R2 = crate::Reg<f6r2::F6R2_SPEC>;
#[doc = "Filter bank 6 register 2"]
pub mod f6r2;
#[doc = "F7R1 (rw) register accessor: Filter bank 7 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7r1`]
module"]
pub type F7R1 = crate::Reg<f7r1::F7R1_SPEC>;
#[doc = "Filter bank 7 register 1"]
pub mod f7r1;
#[doc = "F7R2 (rw) register accessor: Filter bank 7 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7r2`]
module"]
pub type F7R2 = crate::Reg<f7r2::F7R2_SPEC>;
#[doc = "Filter bank 7 register 2"]
pub mod f7r2;
#[doc = "F8R1 (rw) register accessor: Filter bank 8 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8r1`]
module"]
pub type F8R1 = crate::Reg<f8r1::F8R1_SPEC>;
#[doc = "Filter bank 8 register 1"]
pub mod f8r1;
#[doc = "F8R2 (rw) register accessor: Filter bank 8 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8r2`]
module"]
pub type F8R2 = crate::Reg<f8r2::F8R2_SPEC>;
#[doc = "Filter bank 8 register 2"]
pub mod f8r2;
#[doc = "F9R1 (rw) register accessor: Filter bank 9 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9r1`]
module"]
pub type F9R1 = crate::Reg<f9r1::F9R1_SPEC>;
#[doc = "Filter bank 9 register 1"]
pub mod f9r1;
#[doc = "F9R2 (rw) register accessor: Filter bank 9 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9r2`]
module"]
pub type F9R2 = crate::Reg<f9r2::F9R2_SPEC>;
#[doc = "Filter bank 9 register 2"]
pub mod f9r2;
#[doc = "F10R1 (rw) register accessor: Filter bank 10 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10r1`]
module"]
pub type F10R1 = crate::Reg<f10r1::F10R1_SPEC>;
#[doc = "Filter bank 10 register 1"]
pub mod f10r1;
#[doc = "F10R2 (rw) register accessor: Filter bank 10 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10r2`]
module"]
pub type F10R2 = crate::Reg<f10r2::F10R2_SPEC>;
#[doc = "Filter bank 10 register 2"]
pub mod f10r2;
#[doc = "F11R1 (rw) register accessor: Filter bank 11 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11r1`]
module"]
pub type F11R1 = crate::Reg<f11r1::F11R1_SPEC>;
#[doc = "Filter bank 11 register 1"]
pub mod f11r1;
#[doc = "F11R2 (rw) register accessor: Filter bank 11 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11r2`]
module"]
pub type F11R2 = crate::Reg<f11r2::F11R2_SPEC>;
#[doc = "Filter bank 11 register 2"]
pub mod f11r2;
#[doc = "F12R1 (rw) register accessor: Filter bank 4 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12r1`]
module"]
pub type F12R1 = crate::Reg<f12r1::F12R1_SPEC>;
#[doc = "Filter bank 4 register 1"]
pub mod f12r1;
#[doc = "F12R2 (rw) register accessor: Filter bank 12 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12r2`]
module"]
pub type F12R2 = crate::Reg<f12r2::F12R2_SPEC>;
#[doc = "Filter bank 12 register 2"]
pub mod f12r2;
#[doc = "F13R1 (rw) register accessor: Filter bank 13 register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13r1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13r1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13r1`]
module"]
pub type F13R1 = crate::Reg<f13r1::F13R1_SPEC>;
#[doc = "Filter bank 13 register 1"]
pub mod f13r1;
#[doc = "F13R2 (rw) register accessor: Filter bank 13 register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13r2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13r2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13r2`]
module"]
pub type F13R2 = crate::Reg<f13r2::F13R2_SPEC>;
#[doc = "Filter bank 13 register 2"]
pub mod f13r2;
