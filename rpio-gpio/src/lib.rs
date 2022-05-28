#![warn(clippy::all)]
#![no_std]

mod io;

#[cfg(feature = "rp2040")]
mod pinout;
