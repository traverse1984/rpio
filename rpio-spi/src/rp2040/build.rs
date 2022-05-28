use super::cs;
use crate::Spi as EmbeddedSpi;
use embedded_hal::spi::Mode;
use embedded_time::rate::Hertz;
use rp2040_hal::{
    gpio::{Pin, PinId, PinMode, PushPullOutput, ValidPinMode},
    pac::RESETS,
    spi::{Enabled, Spi, SpiDevice},
};

impl EmbeddedSpi {
    pub fn new_rp2040<
        D: SpiDevice,
        CS: PinId,
        M: PinMode + ValidPinMode<CS>,
        P: Into<Hertz>,
        B: Into<Hertz> + Copy,
    >(
        device: D,
        chip_select: Pin<CS, M>,
        resets: &mut RESETS,
        peri_frequency: P,
        baudrate: B,
        mode: &Mode,
    ) -> Rp2040ChipSelectBuilder<D, CS> {
        let spi: Spi<_, _, 8> = Spi::new(device).init(resets, peri_frequency, baudrate, mode);

        Rp2040Builder {
            spi,
            baudrate: baudrate.into(),
        }
        .with_cs(chip_select.into_push_pull_output())
    }

    /// Construct a transport from an [`rp2040::spi::Spi`](Spi).
    pub fn from_rp2040<D: SpiDevice>(
        spi: Spi<Enabled, D, 8>,
        baudrate: impl Into<Hertz>,
    ) -> Rp2040Builder<D> {
        Rp2040Builder {
            spi,
            baudrate: baudrate.into(),
        }
    }
}

pub struct Rp2040Builder<D: SpiDevice> {
    spi: Spi<Enabled, D, 8>,
    baudrate: Hertz,
}

impl<D: SpiDevice> Rp2040Builder<D> {
    /// Use the provided [`rp2040::gpio::Pin`](Pin) for chip select. It must
    /// be configured as a [`PushPullOutput`].
    pub fn with_cs<CS: PinId, M: PinMode + ValidPinMode<CS>>(
        self,
        pin: Pin<CS, M>,
    ) -> Rp2040ChipSelectBuilder<D, CS> {
        Rp2040ChipSelectBuilder {
            spi: self.spi,
            baudrate: self.baudrate,
            cs: pin.into_push_pull_output(),
        }
    }
}

pub struct Rp2040ChipSelectBuilder<D: SpiDevice, P: PinId> {
    spi: Spi<Enabled, D, 8>,
    baudrate: Hertz,
    cs: Pin<P, PushPullOutput>,
    //active_high: bool,
}

impl<D: SpiDevice, P: PinId> Rp2040ChipSelectBuilder<D, P> {
    /// Use the provided polarity. Defaults to [IdleHigh](Polarity::IdleHigh).
    // pub fn with_active_high(mut self) -> Self {
    //     self.active_high = true;
    //     self
    // }

    /// Initialize the transport.
    pub fn init(self) -> cs::Spi<D, P> {
        cs::Spi::new(self.spi, self.baudrate, self.cs)
    }
}
