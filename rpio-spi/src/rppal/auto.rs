use super::super::{Error, Result};
use crate::{ClockSpeed, SpiDevice, Transfer};
use _rppal::spi::Spi as RppalSpi;

pub struct Spi {
    spi: RppalSpi,
}

impl Spi {
    pub fn new(spi: RppalSpi) -> Self {
        Self { spi }
    }
}

impl Transfer<u8> for Spi {
    type Error = Error;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        <RppalSpi as Transfer<u8>>::transfer(&mut self.spi, words).or(Err(Error::Transfer))
    }
}

impl SpiDevice for Spi {
    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}

impl ClockSpeed for Spi {}
