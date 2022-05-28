use crate::{Error, Result, Transfer};

/// Indicates that the implementation of [`Transfer<u8>`](Transfer) for this
/// struct:
///
/// - Selects the chip at the start of transfer.
/// - Deselects the chip at the end of successful transfer.
/// - Uses the [`Error`] type.
pub trait SpiDevice: Transfer<u8, Error = Error> {
    /// Whether chip selection can be controlled
    fn is_chip_select(&self) -> bool {
        false
    }

    /// Whether clock speed can be controlled
    fn is_clock_speed(&self) -> bool {
        false
    }

    /// Select the chip.
    ///
    /// This typically drives the pin low, but in some configurations could
    /// drive the pin high.
    fn select(&mut self) -> Result {
        Err(Error::NotImplemented)
    }

    /// Deselect the chip.
    ///
    /// This typically drives the pin high, but in some configurations could
    /// drive the pin low.
    fn deselect(&mut self) -> Result {
        Err(Error::NotImplemented)
    }

    /// Exchange bytes with the chip without selecting or deslecting it.
    fn raw_transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        Err(Error::NotImplemented)
    }

    /// Exchange bytes with the chip without selecting it. Deselect only if an
    /// error occurs during the transfer.
    fn raw_transfer_or_deselect<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
        self.raw_transfer(words)
            .map_err(|err| self.deselect().map_or(Error::ChipDeselect, |_| err))
    }

    /// Set the SPI clock speed.
    fn set_clock_speed(&mut self, speed: u32) -> Result {
        Err(Error::NotImplemented)
    }
}

/// Indicates that chip selection is controlled by a user-defined output pin.
pub trait ChipSelect: SpiDevice {}

/// Indicates that the SPI clock speed can be set during operation.
pub trait ClockSpeed: SpiDevice {}
