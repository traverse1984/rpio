use super::super::{Error, Result};
use crate::{ChipSelect, OutputPin, SpiDevice, Transfer};

pub struct Spi<SPI: Transfer<u8>, CS: OutputPin> {
    spi: SPI,
    cs: CS,
}

impl<SPI: Transfer<u8>, CS: OutputPin> Spi<SPI, CS> {
    pub fn new(spi: SPI, cs: CS) -> Self {
        let mut transport = Self { spi, cs };

        transport.deselect().ok();
        transport
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> Transfer<u8> for Spi<SPI, CS> {
    impl_cs_transfer_common!();
}

impl<SPI: Transfer<u8>, CS: OutputPin> SpiDevice for Spi<SPI, CS> {
    impl_cs_common!();

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }
}

impl<SPI: Transfer<u8>, CS: OutputPin> ChipSelect for Spi<SPI, CS> {}
