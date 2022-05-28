macro_rules! impl_cs_common {
    () => {
        fn is_chip_select(&self) -> bool {
            true
        }

        fn select(&mut self) -> Result {
            self.cs.set_low().or(Err(Error::ChipSelect))
            // match self.polarity {
            //     Polarity::IdleHigh => self.cs.set_low(),
            //     Polarity::IdleLow => self.cs.set_high(),
            // }
        }

        fn deselect(&mut self) -> Result {
            self.cs.set_high().or(Err(Error::ChipDeselect))
            // match self.polarity {
            //     Polarity::IdleHigh => self.cs.set_high(),
            //     Polarity::IdleLow => self.cs.set_low(),
            // }
        }
    };
}

macro_rules! impl_auto_transfer_common {
    () => {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
            self.spi.transfer(words).or(Err(Error::Transfer))
        }
    };
}

macro_rules! impl_cs_transfer_common {
    () => {
        type Error = Error;

        fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8]> {
            self.select()
                .and_then(|_| self.raw_transfer_or_deselect(words))
                .and_then(|res| self.deselect().and(Ok(res)))
        }
    };
}
