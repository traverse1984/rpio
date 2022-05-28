use super::{auto, cs};
use crate::Spi as EmbeddedSpi;
use _rppal::{
    gpio::{Gpio, OutputPin},
    spi::{Bus, Error, Mode, SlaveSelect, Spi},
};

impl EmbeddedSpi {
    pub fn new_rppal(
        bus: Bus,
        slave_select: SlaveSelect,
        clock_speed: u32,
        mode: Mode,
    ) -> RppalBuilder {
        RppalBuilder {
            spi: Spi::new(bus, slave_select, clock_speed, mode),
        }
    }

    /// Construct a transport from an [`rppal::spi::Spi`](Spi).
    pub fn from_rppal(spi: Spi) -> RppalBuilder {
        RppalBuilder { spi: Ok(Spi) }
    }
}

pub struct RppalBuilder {
    spi: Result<Spi, Error>,
}

impl RppalBuilder {
    /// Use the provided [`rppal::gpio::OutputPin`](OutputPin) for chip select.
    pub fn with_cs(self, bcm_pin: u8) -> RppalChipSelectBuilder {
        let cs = Gpio::new()
            .and_then(|gpio| gpio.get(bcm_pin))
            .map(|pin| pin.into_output_high());

        RppalChipSelectBuilder { spi: self.spi, cs }
    }

    /// Initialize the transport.
    pub fn init(self) -> Result<auto::Spi, Error> {
        Ok(auto::Spi::new(self.spi?))
    }
}

pub struct RppalChipSelectBuilder {
    spi: Result<Spi, Error>,
    cs: Result<OutputPin, Error>,
}

impl RppalChipSelectBuilder {
    /// Initialize the transport.
    pub fn init(self) -> Result<cs::Spi, Error> {
        Ok(cs::Spi::new(self.spi?, self.cs?))
    }
}
