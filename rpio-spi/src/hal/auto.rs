use crate::{Error, Result, SpiDevice, Transfer};

pub struct Spi<SPI: Transfer<u8>> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> Spi<SPI> {
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }
}

impl<SPI: Transfer<u8>> Transfer<u8> for Spi<SPI> {
    impl_auto_transfer_common!();
}

impl<SPI: Transfer<u8>> SpiDevice for Spi<SPI> {}
