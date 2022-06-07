use cortex_m::{delay::Delay as CortexDelay, peripheral::SYST};
use embedded_hal::blocking::delay::*;

static mut DELAY: Option<CortexDelay> = None;

pub struct Delay;

impl Delay {
    pub fn init(syst: SYST, ahb_frequency: u32) -> Result<Self, ()> {
        unsafe {
            if let None = &DELAY {
                DELAY.replace(CortexDelay::new(syst, ahb_frequency));
                Ok(Self)
            } else {
                Err(())
            }
        }
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        unsafe { &mut DELAY }.as_mut().unwrap().delay_ms(ms)
    }
}

impl DelayUs<u32> for Delay {
    fn delay_us(&mut self, us: u32) {
        unsafe { &mut DELAY }.as_mut().unwrap().delay_us(us)
    }
}
