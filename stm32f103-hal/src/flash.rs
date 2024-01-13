
use crate::pac;

pub trait FlashExt {
    fn constrain(self) -> Parts;
}

impl FlashExt for pac::FLASH {
    fn constrain(self) -> Parts {
        Parts {
            acr: ACR(()),
        }
    }
}

pub struct Parts {
    pub acr: ACR,
}

pub struct ACR(());