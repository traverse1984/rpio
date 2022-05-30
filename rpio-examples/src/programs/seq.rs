use crate::io::*;

pub fn keyseq<S, D, K>(io: Io<S, D, K>) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen: ScaledBuf::new(), keypad);
    offset!(4, 4);

    loop {
        println!("Waiting");
        let mut seq = [0; 8];

        for key in keypad.read_seq(&mut seq) {
            match key {
                Some(0xC) => break,
                Some(key) => {
                    print!("Key {:x}", key);
                }
                _ => (),
            }
        }

        clear!();
        draw!(txt "Result\n");
        hex!(&seq);
        update!();

        delay.delay_ms(2000);
        clear!(now);
    }
}
