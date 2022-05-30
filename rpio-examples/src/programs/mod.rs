mod adc;
mod flash;
mod multikey;
mod pwm;
mod seq;

pub use adc::adc;
pub use flash::flash;
pub use multikey::multikey;
pub use pwm::pwm;
pub use seq::keyseq;

#[derive(Clone, Copy)]
pub enum Program {
    FlashBrowser,
    MultiKey,
    KeySeq,
    Adc,
    Pwm,
}

impl Program {
    pub fn name(&self) -> &'static str {
        use Program::*;

        match self {
            FlashBrowser => "Flash Browser",
            MultiKey => "Multikey Example",
            Adc => "ADC Example",
            Pwm => "PWM Example",
            KeySeq => "Key Sequence",
        }
    }
}
