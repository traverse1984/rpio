use super::{auto, cs};
use crate::{OutputPin, Spi, Transfer};

impl Spi {
    /// Construct a transport from any [`Transfer<u8>`](Transfer).
    pub fn from_hal<SPI: Transfer<u8>>(spi: SPI) -> HalBuilder<SPI> {
        HalBuilder { spi }
    }
}

pub struct HalBuilder<SPI: Transfer<u8>> {
    spi: SPI,
}

impl<SPI: Transfer<u8>> HalBuilder<SPI> {
    /// Use the provided chip select pin.
    pub fn with_cs<CS: OutputPin>(self, cs: CS) -> HalChipSelectBuilder<SPI, CS> {
        HalChipSelectBuilder { spi: self.spi, cs }
    }

    /// Initialize the transport.
    ///
    /// Chip select must be handled by the provided SPI device.
    pub fn init(self) -> auto::Spi<SPI> {
        auto::Spi::new(self.spi)
    }
}

pub struct HalChipSelectBuilder<SPI: Transfer<u8>, CS: OutputPin> {
    spi: SPI,
    cs: CS,
}

impl<SPI: Transfer<u8>, CS: OutputPin> HalChipSelectBuilder<SPI, CS> {
    /// Initialize the transport.
    pub fn init(self) -> cs::Spi<SPI, CS> {
        cs::Spi::new(self.spi, self.cs)
    }
}
