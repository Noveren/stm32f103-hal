
use crate::pac;
use fugit::HertzU32 as Hertz;

pub trait RccExt {
    fn constrain(self) -> Rcc;
}

impl RccExt for pac::RCC {
    fn constrain(self) -> Rcc {
        Rcc {
            cfgr: CFGR(())
        }
    }
}

pub struct Rcc {
    pub cfgr: CFGR,
}

pub struct CFGR(());

pub trait Source {
    fn config(&self);
    fn freq(&self) -> Hertz;
}

pub struct HSE(Hertz);

impl HSE {
    pub const fn new(freq: Hertz) -> Self {
        Self(freq)
    }
}

impl Source for HSE {
    fn freq(&self) -> Hertz {
        self.0
    }

    fn config(&self) {
        // TODO
    }
}

pub struct HSI(Hertz);

impl HSI {
    pub const fn new(freq: Hertz) -> Self {
        Self(freq)
    }
}

impl Source for HSI {
    fn freq(&self) -> Hertz {
        self.0
    }

    fn config(&self) {
        // TODO
    }
}

// pub struct PLL {
//     src: (u32, bool)
// }

// impl PLL {
//     pub const fn new() -> Self {
//         Self {

//         }
//     }
// }

// impl Source for PLL {
//     fn config(self) -> Self {
//         // TODO
//         self
//     }
// }

#[repr(u8)]
pub enum PAPB {
    Div1  = 0b001,
    Div2  = 0b100,
    Div4  = 0b101,
    Div8  = 0b110,
    Div16 = 0b111,
}

#[repr(u8)]
pub enum PAHB {
    Div1  = 0b0001,
    Div2  = 0b1000,
    Div4  = 0b1001,
    Div8  = 0b1010,
    Div16 = 0b1011,
    Div64 = 0b1100,
    Div128 = 0b1101,
    Div256 = 0b1110,
    Div512 = 0b1111,
}

pub struct Config<T: Source> {
    src: T,
    papb2: PAPB,
    papb1: PAPB,
    pahb: PAHB,
}

impl CFGR {
    pub const fn source<T: Source>(self, src: T) -> Config<T> {
        Config {
            src, papb1: PAPB::Div1, papb2: PAPB::Div1, pahb: PAHB::Div1
        }
    }
}

impl<T: Source> Config<T> {
    #[inline(always)]
    pub fn papb2(mut self, p: PAPB) -> Self {
        self.papb2 = p;
        self
    }

    #[inline(always)]
    pub fn papb1(mut self, p: PAPB) -> Self {
        self.papb1 = p;
        self
    }

    #[inline(always)]
    pub fn pahb(mut self, p: PAHB) -> Self {
        self.pahb = p;
        self
    }

    #[inline(always)]
    pub fn freeze(self) -> Clocks<T> {
        self.src.config();
        Clocks {
            src: self.src,
        }
    }
}

pub struct Clocks<T: Source> {
    src: T,
}