use super::super::{Error, Result};
use crate::{ChipSelect, ClockSpeed, OutputPin, SpiDevice, Transfer};
use embedded_time::rate::{Extensions, Hertz};
use rp2040_hal::{
    gpio::{Pin, PinId, PushPullOutput},
    spi::{Enabled, Spi as Rp2040Spi, SpiDevice as Rp2040SpiDevice},
};

pub struct Spi<D: Rp2040SpiDevice, P: PinId> {
    spi: Rp2040Spi<Enabled, D, 8>,
    peripheral_freq: Hertz<u32>,
    cs: Pin<P, PushPullOutput>,
}

impl<D: Rp2040SpiDevice, P: PinId> Spi<D, P> {
    pub fn new(
        spi: Rp2040Spi<Enabled, D, 8>,
        peripheral_freq: Hertz<u32>,
        cs: Pin<P, PushPullOutput>,
    ) -> Self {
        let mut transport = Self {
            spi,
            peripheral_freq,
            cs,
        };

        transport.deselect().ok();
        transport
    }
}

impl<D: Rp2040SpiDevice, P: PinId> SpiDevice for Spi<D, P> {
    impl_cs_common!();

    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.spi.transfer(words).or(Err(Error::Transfer))
    }

    fn is_clock_speed(&self) -> bool {
        true
    }

    fn set_clock_speed(&mut self, speed: u32) -> Result {
        self.spi.set_baudrate(self.peripheral_freq, speed.Hz());
        Ok(())
    }
}

impl<D: Rp2040SpiDevice, P: PinId> Transfer<u8> for Spi<D, P> {
    impl_cs_transfer_common!();
}

impl<D: Rp2040SpiDevice, P: PinId> ChipSelect for Spi<D, P> {}
impl<D: Rp2040SpiDevice, P: PinId> ClockSpeed for Spi<D, P> {}
