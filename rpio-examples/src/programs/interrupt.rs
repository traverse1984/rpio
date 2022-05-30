use crate::io::*;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use embedded_hal::digital::v2::ToggleableOutputPin;
use rp2040_hal::{
    gpio::{bank0, Interrupt, Pin, PullUpInput, PushPullOutput},
    pac::{self, interrupt},
};

type LedPin = Pin<bank0::Gpio25, PushPullOutput>;
type ButtonPin = Pin<bank0::Gpio15, PullUpInput>;
type Controls = (LedPin, ButtonPin);

static GLOBAL_CONTROLS: Mutex<RefCell<Option<Controls>>> = Mutex::new(RefCell::new(None));

pub fn interrupt<S, D, K>(io: Io<S, D, K>, led: LedPin, button: ButtonPin) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen: ScaledBuf::new(), _keypad);

    button.set_interrupt_enabled(Interrupt::EdgeLow, true);

    cortex_m::interrupt::free(|cs| {
        GLOBAL_CONTROLS.borrow(cs).replace(Some((led, button)));
    });

    unsafe {
        rp2040_hal::pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }

    let mut count = 0u32;
    loop {
        print!("{}", count);
        count += 1;

        delay.delay_us(1);
    }
}

#[interrupt]
fn IO_IRQ_BANK0() {
    static mut CONTROLS: Option<Controls> = None;

    if CONTROLS.is_none() {
        cortex_m::interrupt::free(|cs| {
            *CONTROLS = GLOBAL_CONTROLS.borrow(cs).take();
        })
    }

    if let Some((led, button)) = CONTROLS {
        led.toggle().ok();
        button.clear_interrupt(Interrupt::EdgeLow);
    }
}
