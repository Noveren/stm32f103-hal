#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    fs_hcfg: FS_HCFG,
    hfir: HFIR,
    fs_hfnum: FS_HFNUM,
    _reserved3: [u8; 0x04],
    fs_hptxsts: FS_HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    fs_hprt: FS_HPRT,
    _reserved7: [u8; 0xbc],
    fs_hcchar0: FS_HCCHAR0,
    _reserved8: [u8; 0x04],
    fs_hcint0: FS_HCINT0,
    fs_hcintmsk0: FS_HCINTMSK0,
    fs_hctsiz0: FS_HCTSIZ0,
    _reserved11: [u8; 0x0c],
    fs_hcchar1: FS_HCCHAR1,
    _reserved12: [u8; 0x04],
    fs_hcint1: FS_HCINT1,
    fs_hcintmsk1: FS_HCINTMSK1,
    fs_hctsiz1: FS_HCTSIZ1,
    _reserved15: [u8; 0x0c],
    fs_hcchar2: FS_HCCHAR2,
    _reserved16: [u8; 0x04],
    fs_hcint2: FS_HCINT2,
    fs_hcintmsk2: FS_HCINTMSK2,
    fs_hctsiz2: FS_HCTSIZ2,
    _reserved19: [u8; 0x0c],
    fs_hcchar3: FS_HCCHAR3,
    _reserved20: [u8; 0x04],
    fs_hcint3: FS_HCINT3,
    fs_hcintmsk3: FS_HCINTMSK3,
    fs_hctsiz3: FS_HCTSIZ3,
    _reserved23: [u8; 0x0c],
    fs_hcchar4: FS_HCCHAR4,
    _reserved24: [u8; 0x04],
    fs_hcint4: FS_HCINT4,
    fs_hcintmsk4: FS_HCINTMSK4,
    fs_hctsiz4: FS_HCTSIZ4,
    _reserved27: [u8; 0x0c],
    fs_hcchar5: FS_HCCHAR5,
    _reserved28: [u8; 0x04],
    fs_hcint5: FS_HCINT5,
    fs_hcintmsk5: FS_HCINTMSK5,
    fs_hctsiz5: FS_HCTSIZ5,
    _reserved31: [u8; 0x0c],
    fs_hcchar6: FS_HCCHAR6,
    _reserved32: [u8; 0x04],
    fs_hcint6: FS_HCINT6,
    fs_hcintmsk6: FS_HCINTMSK6,
    fs_hctsiz6: FS_HCTSIZ6,
    _reserved35: [u8; 0x0c],
    fs_hcchar7: FS_HCCHAR7,
    _reserved36: [u8; 0x04],
    fs_hcint7: FS_HCINT7,
    fs_hcintmsk7: FS_HCINTMSK7,
    fs_hctsiz7: FS_HCTSIZ7,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    #[inline(always)]
    pub const fn fs_hcfg(&self) -> &FS_HCFG {
        &self.fs_hcfg
    }
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    #[inline(always)]
    pub const fn fs_hfnum(&self) -> &FS_HFNUM {
        &self.fs_hfnum
    }
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    #[inline(always)]
    pub const fn fs_hptxsts(&self) -> &FS_HPTXSTS {
        &self.fs_hptxsts
    }
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    #[inline(always)]
    pub const fn fs_hprt(&self) -> &FS_HPRT {
        &self.fs_hprt
    }
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    #[inline(always)]
    pub const fn fs_hcchar0(&self) -> &FS_HCCHAR0 {
        &self.fs_hcchar0
    }
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    #[inline(always)]
    pub const fn fs_hcint0(&self) -> &FS_HCINT0 {
        &self.fs_hcint0
    }
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    #[inline(always)]
    pub const fn fs_hcintmsk0(&self) -> &FS_HCINTMSK0 {
        &self.fs_hcintmsk0
    }
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz0(&self) -> &FS_HCTSIZ0 {
        &self.fs_hctsiz0
    }
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    #[inline(always)]
    pub const fn fs_hcchar1(&self) -> &FS_HCCHAR1 {
        &self.fs_hcchar1
    }
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    #[inline(always)]
    pub const fn fs_hcint1(&self) -> &FS_HCINT1 {
        &self.fs_hcint1
    }
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    #[inline(always)]
    pub const fn fs_hcintmsk1(&self) -> &FS_HCINTMSK1 {
        &self.fs_hcintmsk1
    }
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz1(&self) -> &FS_HCTSIZ1 {
        &self.fs_hctsiz1
    }
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    #[inline(always)]
    pub const fn fs_hcchar2(&self) -> &FS_HCCHAR2 {
        &self.fs_hcchar2
    }
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    #[inline(always)]
    pub const fn fs_hcint2(&self) -> &FS_HCINT2 {
        &self.fs_hcint2
    }
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    #[inline(always)]
    pub const fn fs_hcintmsk2(&self) -> &FS_HCINTMSK2 {
        &self.fs_hcintmsk2
    }
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz2(&self) -> &FS_HCTSIZ2 {
        &self.fs_hctsiz2
    }
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    #[inline(always)]
    pub const fn fs_hcchar3(&self) -> &FS_HCCHAR3 {
        &self.fs_hcchar3
    }
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    #[inline(always)]
    pub const fn fs_hcint3(&self) -> &FS_HCINT3 {
        &self.fs_hcint3
    }
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    #[inline(always)]
    pub const fn fs_hcintmsk3(&self) -> &FS_HCINTMSK3 {
        &self.fs_hcintmsk3
    }
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz3(&self) -> &FS_HCTSIZ3 {
        &self.fs_hctsiz3
    }
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    #[inline(always)]
    pub const fn fs_hcchar4(&self) -> &FS_HCCHAR4 {
        &self.fs_hcchar4
    }
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    #[inline(always)]
    pub const fn fs_hcint4(&self) -> &FS_HCINT4 {
        &self.fs_hcint4
    }
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    #[inline(always)]
    pub const fn fs_hcintmsk4(&self) -> &FS_HCINTMSK4 {
        &self.fs_hcintmsk4
    }
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz4(&self) -> &FS_HCTSIZ4 {
        &self.fs_hctsiz4
    }
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    #[inline(always)]
    pub const fn fs_hcchar5(&self) -> &FS_HCCHAR5 {
        &self.fs_hcchar5
    }
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    #[inline(always)]
    pub const fn fs_hcint5(&self) -> &FS_HCINT5 {
        &self.fs_hcint5
    }
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    #[inline(always)]
    pub const fn fs_hcintmsk5(&self) -> &FS_HCINTMSK5 {
        &self.fs_hcintmsk5
    }
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz5(&self) -> &FS_HCTSIZ5 {
        &self.fs_hctsiz5
    }
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    #[inline(always)]
    pub const fn fs_hcchar6(&self) -> &FS_HCCHAR6 {
        &self.fs_hcchar6
    }
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    #[inline(always)]
    pub const fn fs_hcint6(&self) -> &FS_HCINT6 {
        &self.fs_hcint6
    }
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    #[inline(always)]
    pub const fn fs_hcintmsk6(&self) -> &FS_HCINTMSK6 {
        &self.fs_hcintmsk6
    }
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz6(&self) -> &FS_HCTSIZ6 {
        &self.fs_hctsiz6
    }
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    #[inline(always)]
    pub const fn fs_hcchar7(&self) -> &FS_HCCHAR7 {
        &self.fs_hcchar7
    }
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    #[inline(always)]
    pub const fn fs_hcint7(&self) -> &FS_HCINT7 {
        &self.fs_hcint7
    }
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    #[inline(always)]
    pub const fn fs_hcintmsk7(&self) -> &FS_HCINTMSK7 {
        &self.fs_hcintmsk7
    }
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    #[inline(always)]
    pub const fn fs_hctsiz7(&self) -> &FS_HCTSIZ7 {
        &self.fs_hctsiz7
    }
}
#[doc = "FS_HCFG (rw) register accessor: OTG_FS host configuration register (OTG_FS_HCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcfg`]
module"]
pub type FS_HCFG = crate::Reg<fs_hcfg::FS_HCFG_SPEC>;
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod fs_hcfg;
#[doc = "HFIR (rw) register accessor: OTG_FS Host frame interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`]
module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_FS Host frame interval register"]
pub mod hfir;
#[doc = "FS_HFNUM (r) register accessor: OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hfnum::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hfnum`]
module"]
pub type FS_HFNUM = crate::Reg<fs_hfnum::FS_HFNUM_SPEC>;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod fs_hfnum;
#[doc = "FS_HPTXSTS (rw) register accessor: OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hptxsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hptxsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hptxsts`]
module"]
pub type FS_HPTXSTS = crate::Reg<fs_hptxsts::FS_HPTXSTS_SPEC>;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod fs_hptxsts;
#[doc = "HAINT (r) register accessor: OTG_FS Host all channels interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haint::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`]
module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: OTG_FS host all channels interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`haintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`]
module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "FS_HPRT (rw) register accessor: OTG_FS host port control and status register (OTG_FS_HPRT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hprt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hprt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hprt`]
module"]
pub type FS_HPRT = crate::Reg<fs_hprt::FS_HPRT_SPEC>;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod fs_hprt;
#[doc = "FS_HCCHAR0 (rw) register accessor: OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar0`]
module"]
pub type FS_HCCHAR0 = crate::Reg<fs_hcchar0::FS_HCCHAR0_SPEC>;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod fs_hcchar0;
#[doc = "FS_HCCHAR1 (rw) register accessor: OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar1`]
module"]
pub type FS_HCCHAR1 = crate::Reg<fs_hcchar1::FS_HCCHAR1_SPEC>;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod fs_hcchar1;
#[doc = "FS_HCCHAR2 (rw) register accessor: OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar2`]
module"]
pub type FS_HCCHAR2 = crate::Reg<fs_hcchar2::FS_HCCHAR2_SPEC>;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod fs_hcchar2;
#[doc = "FS_HCCHAR3 (rw) register accessor: OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar3`]
module"]
pub type FS_HCCHAR3 = crate::Reg<fs_hcchar3::FS_HCCHAR3_SPEC>;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod fs_hcchar3;
#[doc = "FS_HCCHAR4 (rw) register accessor: OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar4`]
module"]
pub type FS_HCCHAR4 = crate::Reg<fs_hcchar4::FS_HCCHAR4_SPEC>;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod fs_hcchar4;
#[doc = "FS_HCCHAR5 (rw) register accessor: OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar5`]
module"]
pub type FS_HCCHAR5 = crate::Reg<fs_hcchar5::FS_HCCHAR5_SPEC>;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod fs_hcchar5;
#[doc = "FS_HCCHAR6 (rw) register accessor: OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar6`]
module"]
pub type FS_HCCHAR6 = crate::Reg<fs_hcchar6::FS_HCCHAR6_SPEC>;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod fs_hcchar6;
#[doc = "FS_HCCHAR7 (rw) register accessor: OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcchar7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcchar7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcchar7`]
module"]
pub type FS_HCCHAR7 = crate::Reg<fs_hcchar7::FS_HCCHAR7_SPEC>;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod fs_hcchar7;
#[doc = "FS_HCINT0 (rw) register accessor: OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint0`]
module"]
pub type FS_HCINT0 = crate::Reg<fs_hcint0::FS_HCINT0_SPEC>;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod fs_hcint0;
#[doc = "FS_HCINT1 (rw) register accessor: OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint1`]
module"]
pub type FS_HCINT1 = crate::Reg<fs_hcint1::FS_HCINT1_SPEC>;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod fs_hcint1;
#[doc = "FS_HCINT2 (rw) register accessor: OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint2`]
module"]
pub type FS_HCINT2 = crate::Reg<fs_hcint2::FS_HCINT2_SPEC>;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod fs_hcint2;
#[doc = "FS_HCINT3 (rw) register accessor: OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint3`]
module"]
pub type FS_HCINT3 = crate::Reg<fs_hcint3::FS_HCINT3_SPEC>;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod fs_hcint3;
#[doc = "FS_HCINT4 (rw) register accessor: OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint4`]
module"]
pub type FS_HCINT4 = crate::Reg<fs_hcint4::FS_HCINT4_SPEC>;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod fs_hcint4;
#[doc = "FS_HCINT5 (rw) register accessor: OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint5`]
module"]
pub type FS_HCINT5 = crate::Reg<fs_hcint5::FS_HCINT5_SPEC>;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod fs_hcint5;
#[doc = "FS_HCINT6 (rw) register accessor: OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint6`]
module"]
pub type FS_HCINT6 = crate::Reg<fs_hcint6::FS_HCINT6_SPEC>;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod fs_hcint6;
#[doc = "FS_HCINT7 (rw) register accessor: OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcint7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcint7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcint7`]
module"]
pub type FS_HCINT7 = crate::Reg<fs_hcint7::FS_HCINT7_SPEC>;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod fs_hcint7;
#[doc = "FS_HCINTMSK0 (rw) register accessor: OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk0`]
module"]
pub type FS_HCINTMSK0 = crate::Reg<fs_hcintmsk0::FS_HCINTMSK0_SPEC>;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod fs_hcintmsk0;
#[doc = "FS_HCINTMSK1 (rw) register accessor: OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk1`]
module"]
pub type FS_HCINTMSK1 = crate::Reg<fs_hcintmsk1::FS_HCINTMSK1_SPEC>;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod fs_hcintmsk1;
#[doc = "FS_HCINTMSK2 (rw) register accessor: OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk2`]
module"]
pub type FS_HCINTMSK2 = crate::Reg<fs_hcintmsk2::FS_HCINTMSK2_SPEC>;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod fs_hcintmsk2;
#[doc = "FS_HCINTMSK3 (rw) register accessor: OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk3`]
module"]
pub type FS_HCINTMSK3 = crate::Reg<fs_hcintmsk3::FS_HCINTMSK3_SPEC>;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod fs_hcintmsk3;
#[doc = "FS_HCINTMSK4 (rw) register accessor: OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk4`]
module"]
pub type FS_HCINTMSK4 = crate::Reg<fs_hcintmsk4::FS_HCINTMSK4_SPEC>;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod fs_hcintmsk4;
#[doc = "FS_HCINTMSK5 (rw) register accessor: OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk5`]
module"]
pub type FS_HCINTMSK5 = crate::Reg<fs_hcintmsk5::FS_HCINTMSK5_SPEC>;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod fs_hcintmsk5;
#[doc = "FS_HCINTMSK6 (rw) register accessor: OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk6`]
module"]
pub type FS_HCINTMSK6 = crate::Reg<fs_hcintmsk6::FS_HCINTMSK6_SPEC>;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod fs_hcintmsk6;
#[doc = "FS_HCINTMSK7 (rw) register accessor: OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hcintmsk7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hcintmsk7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hcintmsk7`]
module"]
pub type FS_HCINTMSK7 = crate::Reg<fs_hcintmsk7::FS_HCINTMSK7_SPEC>;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod fs_hcintmsk7;
#[doc = "FS_HCTSIZ0 (rw) register accessor: OTG_FS host channel-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz0`]
module"]
pub type FS_HCTSIZ0 = crate::Reg<fs_hctsiz0::FS_HCTSIZ0_SPEC>;
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod fs_hctsiz0;
#[doc = "FS_HCTSIZ1 (rw) register accessor: OTG_FS host channel-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz1`]
module"]
pub type FS_HCTSIZ1 = crate::Reg<fs_hctsiz1::FS_HCTSIZ1_SPEC>;
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod fs_hctsiz1;
#[doc = "FS_HCTSIZ2 (rw) register accessor: OTG_FS host channel-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz2`]
module"]
pub type FS_HCTSIZ2 = crate::Reg<fs_hctsiz2::FS_HCTSIZ2_SPEC>;
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod fs_hctsiz2;
#[doc = "FS_HCTSIZ3 (rw) register accessor: OTG_FS host channel-3 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz3`]
module"]
pub type FS_HCTSIZ3 = crate::Reg<fs_hctsiz3::FS_HCTSIZ3_SPEC>;
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod fs_hctsiz3;
#[doc = "FS_HCTSIZ4 (rw) register accessor: OTG_FS host channel-x transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz4`]
module"]
pub type FS_HCTSIZ4 = crate::Reg<fs_hctsiz4::FS_HCTSIZ4_SPEC>;
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod fs_hctsiz4;
#[doc = "FS_HCTSIZ5 (rw) register accessor: OTG_FS host channel-5 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz5`]
module"]
pub type FS_HCTSIZ5 = crate::Reg<fs_hctsiz5::FS_HCTSIZ5_SPEC>;
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod fs_hctsiz5;
#[doc = "FS_HCTSIZ6 (rw) register accessor: OTG_FS host channel-6 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz6`]
module"]
pub type FS_HCTSIZ6 = crate::Reg<fs_hctsiz6::FS_HCTSIZ6_SPEC>;
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod fs_hctsiz6;
#[doc = "FS_HCTSIZ7 (rw) register accessor: OTG_FS host channel-7 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hctsiz7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hctsiz7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hctsiz7`]
module"]
pub type FS_HCTSIZ7 = crate::Reg<fs_hctsiz7::FS_HCTSIZ7_SPEC>;
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod fs_hctsiz7;
