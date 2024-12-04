#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod uart;
