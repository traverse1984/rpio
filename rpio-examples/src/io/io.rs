pub use rpio::{
    dev::{keypad::*, pico_oled::*, screen, use_screen},
    flash::*,
    spi::SpiDevice,
    OutputPin,
};

use cortex_m::delay::Delay;

pub struct Io<S: SpiDevice, D: OutputPin, K: Keypad> {
    delay: Delay,
    oled: PicoOled<S, D>,
    keypad: K,
}

impl<S: SpiDevice, D: OutputPin, K: Keypad> Io<S, D, K> {
    pub fn new(delay: Delay, oled: PicoOled<S, D>, keypad: K) -> Self {
        Self {
            delay,
            oled,
            keypad,
        }
    }

    pub fn take(self) -> (Delay, PicoOled<S, D>, K) {
        (self.delay, self.oled, self.keypad)
    }
}

#[macro_export]
macro_rules! setup {
   ($io: expr => $delay: ident, $screen: ident $(: $buf: expr)?, $keypad: ident) => {
      let (mut $delay, mut oled, mut $keypad) = $io.take();
      let mut $screen = screen!(oled $(, $buf)?);
      use_screen!($screen);
   };
}
