use crate::io::*;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use embedded_hal::digital::v2::ToggleableOutputPin;
use rp2040_hal::{
    gpio::{bank0, Interrupt, Pin, PullUpInput, PushPullOutput},
    pac::{self, interrupt},
};

use rpio::InputPin;

type ClkPin = Pin<bank0::Gpio17, PullUpInput>;
type DataPin = Pin<bank0::Gpio15, PullUpInput>;
type Controls = (ClkPin, DataPin);
type Bus = (Option<u8>, State);

static GLOBAL_CONTROLS: Mutex<RefCell<Option<Controls>>> = Mutex::new(RefCell::new(None));

pub fn read_irq<S, D, K>(io: Io<S, D, K>, clk: ClkPin, data: DataPin) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen, keypad);

    offset!(4, 4);

    clk.set_interrupt_enabled(Interrupt::EdgeLow, true);

    cortex_m::interrupt::free(|cs| {
        GLOBAL_CONTROLS.borrow(cs).replace(Some((clk, data)));
    });

    unsafe {
        rp2040_hal::pac::NVIC::unmask(pac::Interrupt::IO_IRQ_BANK0);
    }

    let mut dbuf = [0; 8];
    let mut len = 0;

    loop {
        match keypad.read_keyup() {
            Some(0xA) => {
                cortex_m::interrupt::free(|cs| {
                    len = BUF.borrow(cs).borrow_mut().read(&mut dbuf);
                });

                if len > 0 {
                    clear!();
                    print!("Len {}", len);
                    cur!(4, 12);
                    hex!(&dbuf[0..len]);
                    update!();
                } else {
                    print!("Empty");
                }
            }
            Some(0xC) => {
                cortex_m::interrupt::free(|cs| {
                    let mut buf = BUF.borrow(cs).borrow_mut();
                    buf.reset();
                    dbuf.fill(0);
                    // if let State::Overflow = buf.state {
                    //     buf.state = State::ClearOverflow;
                    // }
                });

                print!("Cleared");
            }
            _ => (),
        };
    }
}

enum State {
    Ok,
    Overflow,
    ClearOverflow,
}

struct InputBuffer {
    buf: [u8; 8],
    len: usize,
    curr: u8,
    curr_idx: u8,
    state: State,
}

impl InputBuffer {
    const fn new() -> Self {
        Self {
            buf: [0; 8],
            len: 0,
            curr: 0,
            curr_idx: 0,
            state: State::Ok,
        }
    }

    fn reset(&mut self) {
        self.buf.fill(0);
        self.len = 0;
        self.curr = 0;
        self.curr_idx = 0;
        self.state = State::Ok;
    }

    fn read(&mut self, buf: &mut [u8]) -> usize {
        let len = buf.len().min(self.len);
        if len > 0 {
            (&mut buf[0..len]).copy_from_slice(&self.buf[0..len]);
            self.len = 0;
        }

        len
    }
}

static BUF: Mutex<RefCell<InputBuffer>> = Mutex::new(RefCell::new(InputBuffer::new()));

#[interrupt]
fn IO_IRQ_BANK0() {
    static mut CONTROLS: Option<Controls> = None;

    if CONTROLS.is_none() {
        cortex_m::interrupt::free(|cs| {
            *CONTROLS = GLOBAL_CONTROLS.borrow(cs).take();
        })
    }

    if let Some((clk, data)) = CONTROLS {
        // Do not run this unless state is not Interrupt?
        cortex_m::interrupt::free(|cs| {
            let mut buf = BUF.borrow(cs).borrow_mut();

            if let State::Ok = buf.state {
                buf.curr = (buf.curr << 1) | data.is_low().unwrap() as u8;

                if buf.curr_idx == 7 {
                    if buf.len <= buf.buf.len() {
                        let (len, curr) = (buf.len, buf.curr);
                        buf.buf[len] = curr;

                        buf.curr_idx = 0;
                        buf.curr = 0;
                        buf.len += 1;
                    } else {
                        buf.state = State::Overflow;
                    }
                } else {
                    buf.curr_idx += 1;
                }
            }
        });

        // if !*overflow {
        //     *curr = (*curr << 1) | data.is_high().unwrap() as u8;
        //     if *curr_idx == 6 {
        //         *curr_idx = 0;

        //         match (*buf).get_mut(*idx) {
        //             Some(opt) => {
        //                 opt.replace(*curr);
        //             }
        //             None => {
        //                 *overflow = true;
        //             }
        //         };
        //     }
        // }

        clk.clear_interrupt(Interrupt::EdgeLow);
    }
}
