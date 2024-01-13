
#![no_std]
#![no_main]

#[allow(unused_imports)]
use stm32f103_hal::{
    halt as _, pac, prelude::*
};
use stm32f103_hal::rcc;
// use stm32f103_hal::rcc;

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    rcc.cfgr
        .source(rcc::HSE::new(8.MHz()))
        .pahb(rcc::PAHB::Div2)
        .freeze();
    
    // rcc.cfgr.freeze(&mut flash.acr);
    // 时钟配置，采用构造者模式，枚举出配置情况
    // rcc.cfgr.config().freeze(&mut flash.acr);
    // let cfgr = rcc.cfgr;
    // rcc.cfgr.use_hse(8.MHz());

    loop {}
}