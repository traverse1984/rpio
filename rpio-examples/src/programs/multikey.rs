use crate::io::*;

pub fn multikey<S, D, K>(io: Io<S, D, K>) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen: ScaledBuf::new(), keypad);

    loop {
        clear!();

        match keypad.read_multi() {
            Some(keys) => {
                cur!(5, 13);
                hex!(&keys.as_array().map(|key| key.unwrap_or(0xFF)));
            }
            _ => {
                cur!(12, 13);
                draw!(fmt "Waiting");
            }
        };

        update!();

        delay.delay_ms(100);
    }
}
