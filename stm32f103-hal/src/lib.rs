
#![no_std]

// use stm32f103 as pac;

pub mod halt;

pub mod prelude;

pub use stm32f103 as pac;

pub mod flash;

pub mod rcc;