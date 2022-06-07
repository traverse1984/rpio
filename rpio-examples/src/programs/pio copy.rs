use crate::io::*;

use rp2040_hal::{
    gpio::{bank0, FunctionPio0, Pin, PullDownDisabled},
    pac::PIO0,
    pio::{PIOExt, UninitStateMachine, PIO, SM0},
};

type PioPin = Pin<bank0::Gpio17, PullDownDisabled>;

pub fn pio<S, D, K>(
    io: Io<S, D, K>,
    pin: PioPin,
    mut pio: PIO<PIO0>,
    sm0: UninitStateMachine<(PIO0, SM0)>,
) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen, keypad);

    let _pin: Pin<_, FunctionPio0> = pin.into_mode();
    let mut asm = pio::Assembler::<32>::new();
    let mut wrap_target = asm.label();
    let mut wrap_source = asm.label();

    asm.set(pio::SetDestination::PINDIRS, 1);
    asm.bind(&mut wrap_target);

    asm.set_with_delay(pio::SetDestination::PINS, 0, 31);
    asm.set_with_delay(pio::SetDestination::PINS, 1, 31);

    asm.bind(&mut wrap_source);

    let program = asm.assemble_with_wrap(wrap_source, wrap_target);

    let installed = pio.install(&program).unwrap();
    let div = 0f32; // as slow as possible (0 is interpreted as 65536)
    let (sm, _, _) = rp2040_hal::pio::PIOBuilder::from_program(installed)
        .set_pins(17, 1)
        .clock_divisor(div)
        .build(sm0);
    sm.start();

    let mut n: usize = 0;
    loop {
        print!("Count {}", n);
        n += 1;
        delay.delay_ms(1000);
    }
}
