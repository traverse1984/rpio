#![no_std]
#![warn(clippy::all)]

pub use embedded_hal::{blocking::spi::Transfer, digital::v2::OutputPin};

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod common;

mod error;
mod traits;

#[derive(Debug, Default)]
pub struct Spi;

#[cfg(feature = "hal")]
mod hal;

#[cfg(feature = "rppal")]
mod rppal;

#[cfg(feature = "rppal")]
pub use _rppal::spi::{Bus, Mode, SlaveSelect};

#[cfg(feature = "rp2040")]
mod rp2040;

#[cfg(feature = "rp2040")]
pub use embedded_hal::spi::{Mode, MODE_0, MODE_1, MODE_2, MODE_3};

pub use {
    error::{Error, Result},
    traits::{ChipSelect, ClockSpeed, SpiDevice},
};
