use crate::io::*;

pub fn multikey<S, D, K>(io: Io<S, D, K>) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen: ScaledBuf::new(), keypad);

    loop {}

    // loop {
    //     clear!();

    //     match keypad.read_delayed(&mut delay) {
    //         Some(key) => {
    //             cur!(5, 13);
    //             draw!(fmt "{:0x}", key);
    //         }
    //         _ => {
    //             cur!(12, 13);
    //             if keypad.key_is_pressed() {
    //                 draw!(fmt "Keydown");
    //             } else {
    //                 draw!(fmt "Waiting");
    //             }
    //         }
    //     };

    //     update!();

    //     delay.delay_ms(100);
    // }
}
