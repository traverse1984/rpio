use crate::io::*;

pub fn adc<S, D, K, AdcChannel>(io: Io<S, D, K>) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen, keypad);
}
