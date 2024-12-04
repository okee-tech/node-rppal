#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod gpio;
pub mod i2c;
pub mod pwm;
pub mod spi;
pub mod uart;

use rppal;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
