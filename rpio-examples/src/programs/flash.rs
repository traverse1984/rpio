use crate::io::*;

pub fn flash<S, D, K, FlashSpi, FlashBuf>(
    io: Io<S, D, K>,
    mut flash: Device<FlashSpi, FlashBuf>,
) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
    FlashSpi: SpiDevice,
    FlashBuf: FlashBuffer,
{
    setup!(io => delay, screen, keypad);
    offset!(38, 28);

    // flash.write_block_protect_bits(0).ok().unwrap();

    // for i in 0..4096 {
    //     flash.write_enable().ok().unwrap();
    //     flash.write_byte(0x1000 | i, (i % 256) as u8);
    // }

    let mut addr = 0x1000;
    //let status = flash.read_status().ok().unwrap();
    let data = flash.read(0x1000, 4096).unwrap();

    let mut started = false;
    let mut i: usize = 0;
    let mut kd = 0;
    let mut ka = false;

    loop {
        let kp = keypad.key_is_pressed();

        let mut display = false;

        if kp || !started {
            if ka {
                match keypad.read() {
                    Some(key @ 0..=9) => {
                        while let Some(_) = keypad.read() {}
                        let nx = kd * 10 + key;

                        print!("SEARCH {}", nx);
                        delay.delay_ms(150);

                        kd = 0;
                        ka = false;

                        if nx < 64 {
                            i = nx as usize;
                        } else {
                            print!("INVALID");
                            delay.delay_ms(500);
                            continue;
                        }
                    }
                    Some(0xC) => {
                        ka = false;
                        kd = 0;
                        continue;
                    }
                    _ => (),
                };

                if ka {
                    continue;
                }
            }

            display = true;

            if !started {
                started = true;
            } else if kp {
                match keypad.read() {
                    Some(key @ 0..=9) => {
                        ka = true;
                        kd = key;

                        print!("SEARCH {}", key);

                        // print(&mut fb, 10, 28, "SEARCH");
                        // print(&mut fb, 54, 28, core::str::from_utf8(&[key + 48]).unwrap());
                        // oled.update(fb.read());

                        while let Some(_) = keypad.read() {}
                        continue;
                    }
                    Some(0xF) => i = (i + 1) % 64,
                    Some(0xE) => i = i.saturating_sub(1),
                    Some(0xD) => i = 0,
                    Some(0xB) => {
                        // let page = [(i / 10) as u8 + 48, (i % 10) as u8 + 48];
                        // let n = core::str::from_utf8(&page).unwrap();

                        print!("PAGE {}", i);
                        // print(&mut fb, 10, 28, "  PAGE");
                        // print(&mut fb, 54, 28, n);
                        // oled.update(fb.read());

                        keypad.read_keyup();
                    }
                    Some(0xA) => {
                        print!("ADDR {:x}", addr);

                        keypad.read_keyup();
                    }
                    _ => (),
                };
            }

            if !started {
                started = true;
            }
        }

        if display {
            clear!();
            let chunk = 64 * i;
            for j in 0..8 {
                let offset = chunk + j * 8;
                let chunk = &data[offset..offset + 8];

                cur!(6, 2 + (j * 7));
                hex!(chunk);
            }

            let bar = u128::MAX << (128 - ((i + 1) * 2));

            draw!(0, 61, &[bar, bar, bar]);
            update!();

            delay.delay_ms(150);
        }
    }
}
