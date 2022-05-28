use super::super::{Error, Result};
use crate::{ChipSelect, ClockSpeed, SpiDevice, Transfer};
use _rppal::{gpio::OutputPin as RppalPin, spi::Spi as RppalSpi};

pub struct Spi {
    spi: RppalSpi,
    cs: RppalPin,
}

impl Spi {
    pub fn new(spi: RppalSpi, cs: RppalPin) -> Self {
        let mut transport = Self { spi, cs };

        transport.deselect().ok();
        transport
    }
}

impl SpiDevice for Spi {
    fn is_chip_select(&self) -> bool {
        true
    }

    fn select(&mut self) -> Result {
        self.cs.set_low();
        Ok(())
    }

    fn deselect(&mut self) -> Result {
        self.cs.set_high();
        Ok(())
    }

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        <RppalSpi as Transfer<u8>>::transfer(&mut self.spi, words).or(Err(Error::Transfer))
    }

    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_clock_speed(speed).or(Err(Error::ClockSpeed))
    }
}

impl Transfer<u8> for Spi {
    impl_cs_transfer_common!();
}

impl ChipSelect for Spi {}
impl ClockSpeed for Spi {}
