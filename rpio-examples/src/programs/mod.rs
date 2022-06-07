mod adc;
mod flash;
//mod interrupt;
mod draw;
mod multikey;
mod pio;
mod pwm;
mod read_irq;
mod seq;

pub use adc::adc;
pub use flash::flash;
//pub use interrupt::interrupt;
pub use self::pio::pio;
pub use draw::draw;
pub use multikey::multikey;
pub use pwm::pwm;
pub use read_irq::read_irq;
pub use seq::keyseq;

#[derive(Clone, Copy)]
pub enum Program {
    FlashBrowser,
    MultiKey,
    KeySeq,
    Adc,
    Pwm,
    Interrupt,
    ReadIrq,
    Draw,
    Pio,
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
            Interrupt => "Interrupt",
            ReadIrq => "Read IRQ",
            Draw => "Draw",
            Pio => "Pio Example",
        }
    }
}
