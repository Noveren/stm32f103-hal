#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    fs_gotgctl: FS_GOTGCTL,
    fs_gotgint: FS_GOTGINT,
    fs_gahbcfg: FS_GAHBCFG,
    fs_gusbcfg: FS_GUSBCFG,
    fs_grstctl: FS_GRSTCTL,
    fs_gintsts: FS_GINTSTS,
    fs_gintmsk: FS_GINTMSK,
    _reserved_7_fs_grxstsr: [u8; 0x04],
    _reserved8: [u8; 0x04],
    fs_grxfsiz: FS_GRXFSIZ,
    _reserved_9_fs_gnptxfsiz: [u8; 0x04],
    fs_gnptxsts: FS_GNPTXSTS,
    _reserved11: [u8; 0x08],
    fs_gccfg: FS_GCCFG,
    fs_cid: FS_CID,
    _reserved13: [u8; 0xc0],
    fs_hptxfsiz: FS_HPTXFSIZ,
    fs_dieptxf1: FS_DIEPTXF1,
    fs_dieptxf2: FS_DIEPTXF2,
    fs_dieptxf3: FS_DIEPTXF3,
}
impl RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    #[inline(always)]
    pub const fn fs_gotgctl(&self) -> &FS_GOTGCTL {
        &self.fs_gotgctl
    }
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    #[inline(always)]
    pub const fn fs_gotgint(&self) -> &FS_GOTGINT {
        &self.fs_gotgint
    }
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    #[inline(always)]
    pub const fn fs_gahbcfg(&self) -> &FS_GAHBCFG {
        &self.fs_gahbcfg
    }
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    #[inline(always)]
    pub const fn fs_gusbcfg(&self) -> &FS_GUSBCFG {
        &self.fs_gusbcfg
    }
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    #[inline(always)]
    pub const fn fs_grstctl(&self) -> &FS_GRSTCTL {
        &self.fs_grstctl
    }
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    #[inline(always)]
    pub const fn fs_gintsts(&self) -> &FS_GINTSTS {
        &self.fs_gintsts
    }
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    #[inline(always)]
    pub const fn fs_gintmsk(&self) -> &FS_GINTMSK {
        &self.fs_gintmsk
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub const fn fs_grxstsr_host(&self) -> &FS_GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub const fn fs_grxstsr_device(&self) -> &FS_GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    #[inline(always)]
    pub const fn fs_grxfsiz(&self) -> &FS_GRXFSIZ {
        &self.fs_grxfsiz
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub const fn fs_gnptxfsiz_host(&self) -> &FS_GNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub const fn fs_gnptxfsiz_device(&self) -> &FS_GNPTXFSIZ_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    #[inline(always)]
    pub const fn fs_gnptxsts(&self) -> &FS_GNPTXSTS {
        &self.fs_gnptxsts
    }
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    #[inline(always)]
    pub const fn fs_gccfg(&self) -> &FS_GCCFG {
        &self.fs_gccfg
    }
    #[doc = "0x3c - core ID register"]
    #[inline(always)]
    pub const fn fs_cid(&self) -> &FS_CID {
        &self.fs_cid
    }
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    #[inline(always)]
    pub const fn fs_hptxfsiz(&self) -> &FS_HPTXFSIZ {
        &self.fs_hptxfsiz
    }
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    #[inline(always)]
    pub const fn fs_dieptxf1(&self) -> &FS_DIEPTXF1 {
        &self.fs_dieptxf1
    }
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    #[inline(always)]
    pub const fn fs_dieptxf2(&self) -> &FS_DIEPTXF2 {
        &self.fs_dieptxf2
    }
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    #[inline(always)]
    pub const fn fs_dieptxf3(&self) -> &FS_DIEPTXF3 {
        &self.fs_dieptxf3
    }
}
#[doc = "FS_GOTGCTL (rw) register accessor: OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gotgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gotgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gotgctl`]
module"]
pub type FS_GOTGCTL = crate::Reg<fs_gotgctl::FS_GOTGCTL_SPEC>;
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod fs_gotgctl;
#[doc = "FS_GOTGINT (rw) register accessor: OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gotgint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gotgint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gotgint`]
module"]
pub type FS_GOTGINT = crate::Reg<fs_gotgint::FS_GOTGINT_SPEC>;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod fs_gotgint;
#[doc = "FS_GAHBCFG (rw) register accessor: OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gahbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gahbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gahbcfg`]
module"]
pub type FS_GAHBCFG = crate::Reg<fs_gahbcfg::FS_GAHBCFG_SPEC>;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod fs_gahbcfg;
#[doc = "FS_GUSBCFG (rw) register accessor: OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gusbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gusbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gusbcfg`]
module"]
pub type FS_GUSBCFG = crate::Reg<fs_gusbcfg::FS_GUSBCFG_SPEC>;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod fs_gusbcfg;
#[doc = "FS_GRSTCTL (rw) register accessor: OTG_FS reset register (OTG_FS_GRSTCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_grstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_grstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_grstctl`]
module"]
pub type FS_GRSTCTL = crate::Reg<fs_grstctl::FS_GRSTCTL_SPEC>;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod fs_grstctl;
#[doc = "FS_GINTSTS (rw) register accessor: OTG_FS core interrupt register (OTG_FS_GINTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gintsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gintsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gintsts`]
module"]
pub type FS_GINTSTS = crate::Reg<fs_gintsts::FS_GINTSTS_SPEC>;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod fs_gintsts;
#[doc = "FS_GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gintmsk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gintmsk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gintmsk`]
module"]
pub type FS_GINTMSK = crate::Reg<fs_gintmsk::FS_GINTMSK_SPEC>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod fs_gintmsk;
#[doc = "FS_GRXSTSR_Device (r) register accessor: OTG_FS Receive status debug read(Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_grxstsr_device::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_grxstsr_device`]
module"]
pub type FS_GRXSTSR_DEVICE = crate::Reg<fs_grxstsr_device::FS_GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod fs_grxstsr_device;
#[doc = "FS_GRXSTSR_Host (r) register accessor: OTG_FS Receive status debug read(Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_grxstsr_host::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_grxstsr_host`]
module"]
pub type FS_GRXSTSR_HOST = crate::Reg<fs_grxstsr_host::FS_GRXSTSR_HOST_SPEC>;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod fs_grxstsr_host;
#[doc = "FS_GRXFSIZ (rw) register accessor: OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_grxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_grxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_grxfsiz`]
module"]
pub type FS_GRXFSIZ = crate::Reg<fs_grxfsiz::FS_GRXFSIZ_SPEC>;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod fs_grxfsiz;
#[doc = "FS_GNPTXFSIZ_Device (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gnptxfsiz_device::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gnptxfsiz_device::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gnptxfsiz_device`]
module"]
pub type FS_GNPTXFSIZ_DEVICE = crate::Reg<fs_gnptxfsiz_device::FS_GNPTXFSIZ_DEVICE_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod fs_gnptxfsiz_device;
#[doc = "FS_GNPTXFSIZ_Host (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gnptxfsiz_host::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gnptxfsiz_host::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gnptxfsiz_host`]
module"]
pub type FS_GNPTXFSIZ_HOST = crate::Reg<fs_gnptxfsiz_host::FS_GNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod fs_gnptxfsiz_host;
#[doc = "FS_GNPTXSTS (r) register accessor: OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gnptxsts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gnptxsts`]
module"]
pub type FS_GNPTXSTS = crate::Reg<fs_gnptxsts::FS_GNPTXSTS_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod fs_gnptxsts;
#[doc = "FS_GCCFG (rw) register accessor: OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_gccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_gccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_gccfg`]
module"]
pub type FS_GCCFG = crate::Reg<fs_gccfg::FS_GCCFG_SPEC>;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod fs_gccfg;
#[doc = "FS_CID (rw) register accessor: core ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_cid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_cid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_cid`]
module"]
pub type FS_CID = crate::Reg<fs_cid::FS_CID_SPEC>;
#[doc = "core ID register"]
pub mod fs_cid;
#[doc = "FS_HPTXFSIZ (rw) register accessor: OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_hptxfsiz::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_hptxfsiz::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_hptxfsiz`]
module"]
pub type FS_HPTXFSIZ = crate::Reg<fs_hptxfsiz::FS_HPTXFSIZ_SPEC>;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod fs_hptxfsiz;
#[doc = "FS_DIEPTXF1 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_dieptxf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_dieptxf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_dieptxf1`]
module"]
pub type FS_DIEPTXF1 = crate::Reg<fs_dieptxf1::FS_DIEPTXF1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod fs_dieptxf1;
#[doc = "FS_DIEPTXF2 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_dieptxf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_dieptxf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_dieptxf2`]
module"]
pub type FS_DIEPTXF2 = crate::Reg<fs_dieptxf2::FS_DIEPTXF2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod fs_dieptxf2;
#[doc = "FS_DIEPTXF3 (rw) register accessor: OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_dieptxf3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_dieptxf3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_dieptxf3`]
module"]
pub type FS_DIEPTXF3 = crate::Reg<fs_dieptxf3::FS_DIEPTXF3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod fs_dieptxf3;
