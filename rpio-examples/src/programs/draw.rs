use crate::io::*;

fn enlarge(scan: u16) -> u64 {
    let scan = wide_u16(scan) as u64;
    let scan = (scan << 16) | 0xFF000000000000FF;
    scan
}

fn to_u16(buf: &[u8; 5], len: usize) -> u16 {
    if len > 0 {
        let mut local = [0; 5];
        local.copy_from_slice(buf);
        local = local.map(|byte| byte + 48);
        core::str::from_utf8(&local[0..len])
            .unwrap()
            .parse::<u16>()
            .unwrap_or(65535)
    } else {
        0
    }
}

pub fn draw<S, D, K>(io: Io<S, D, K>) -> !
where
    S: SpiDevice,
    D: OutputPin,
    K: Keypad,
{
    setup!(io => delay, screen: ScaledBuf::new(), keypad);
    offset!(0, 0);

    // let mut squares = [0b1010101010101010u16; 16];
    // for i in 0..16 {
    //     if i % 4 > 1 {
    //         squares[i] = !squares[i];
    //     }
    // }

    // let squares = squares.map(|sq| enlarge(sq));

    // draw!(0, 0, &squares);
    // draw!(0, 16, &squares);
    // update!();

    let mut img = [0xFF000000000000FFu64; 32];
    let mut img_ptr = 0;
    let mut seq = [0; 5];
    let mut idx = 0;

    let mut hexin = false;
    let mut hexval: u16 = 0;

    let mut del = false;

    draw!(0, 0, &img);
    update!();

    loop {
        let mut display_input = false;
        let mut display_image = false;

        match keypad.read_keyup() {
            Some(0xF) => {
                img.fill(0xFF000000000000FF);
                img_ptr = 0;
                display_image = true;
            }

            Some(0xB) if img_ptr > 0 && idx == 0 && !hexin => {
                if del {
                    img[img_ptr - 1] = 0xFF000000000000FFu64;
                    img[img_ptr - 2] = 0xFF000000000000FFu64;
                    del = false;
                    img_ptr -= 2;
                    display_image = true;
                } else {
                    del = true;
                    continue;
                }
            }

            _ if img_ptr == 32 => (),

            Some(_) if del => {
                del = false;
                continue;
            }

            Some(0xA) if hexin => {
                clear!();
                offset!(10, 12);
                draw!(txt "HEX");
                update!();

                for i in (0u8..4).rev() {
                    let key = loop {
                        match keypad.read_keyup() {
                            Some(k) => break k,
                            None => (),
                        }
                    };

                    hexval = hexval | ((key as u16) << (i * 4));
                    print!("{:04x}", hexval);
                }

                offset!(0, 0);
                img[img_ptr] = enlarge(hexval);
                img[img_ptr + 1] = enlarge(hexval);
                img_ptr += 2;
                display_image = true;
                hexval = 0;
            }

            Some(_) if hexin => {
                hexin = false;
                continue;
            }

            Some(0xA) if idx == 0 => {
                hexin = true;
                delay.delay_ms(150);
                continue;
            }

            Some(0xC) => {
                idx = 0;
                seq.fill(0);
                display_image = true;
            }
            Some(0xE) => {
                let scan = enlarge(to_u16(&seq, idx));
                img[img_ptr] = scan;
                img[img_ptr + 1] = scan;
                img_ptr += 2;
                idx = 0;
                seq.fill(0);
                display_image = true;
            }
            Some(0xD) if idx > 0 => {
                seq[idx - 1] = 0;
                idx -= 1;
                display_input = true;
            }
            Some(key @ 0..=9) if idx < 5 => {
                seq[idx] = key;
                idx += 1;
                display_input = true;
            }
            _ => (),
        }

        if display_image {
            clear!();
            screen.buf().draw(0, 0, &img);
            update!();
        } else if display_input {
            clear!();
            cur!(10, 12);
            draw!(fmt "{:05}", to_u16(&seq, idx));
            update!();
        }
    }
}

// Copied from pico_oled - Should be const/static array.
#[inline]
fn wide_nibble(val: u8) -> u8 {
    match val & 0x0F {
        0x00 => 0,
        0x01 => 3,
        0x02 => 12,
        0x03 => 15,
        0x04 => 48,
        0x05 => 51,
        0x06 => 60,
        0x07 => 63,
        0x08 => 192,
        0x09 => 195,
        0x0A => 204,
        0x0B => 207,
        0x0C => 240,
        0x0D => 243,
        0x0E => 252,
        0x0F => 255,
        _ => unreachable!(),
    }
}

#[inline]
fn wide_u8(val: u8) -> u16 {
    u16::from_be_bytes([wide_nibble((val & 0xF0) >> 4), wide_nibble(val)])
}

#[inline]
fn wide_u16(val: u16) -> u32 {
    let (msb, lsb) = ((val & 0xFF00) >> 8, val & 0x00FF);
    ((wide_u8(msb as u8) as u32) << 16) | (wide_u8(lsb as u8) as u32)
}
