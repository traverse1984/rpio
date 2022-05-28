#![warn(clippy::all)]
#![no_std]

pub use embedded_hal::digital::v2::{InputPin, OutputPin};

mod io;

#[cfg(feature = "rp2040")]
mod pinout;
