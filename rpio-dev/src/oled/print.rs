use super::{ascii, Draw};

pub fn print<F: Draw<u8>>(fb: &mut F, mut x: usize, mut y: usize, data: &str) {
    let ox = x;

    let xlimit = fb.width() - x; // not safe.
    let ylimit = fb.height() - y;

    for (i, sprite) in ascii::string(data).enumerate() {
        if x >= xlimit || ascii::char(b'\n') == sprite {
            y += 7;
            x = ox;

            if ascii::char(b'\n') == sprite {
                continue;
            }

            if y >= ylimit {
                break;
            }
        }

        fb.blit(x, y, sprite);
        x += 6;
    }
}

pub fn hexdump<F: Draw<u8>>(fb: &mut F, mut x: usize, mut y: usize, data: &[u8]) {
    let ox = x;

    let xlimit = fb.width() - x; // not safe.
    let ylimit = fb.height() - y;

    for (i, sprite) in ascii::hexstr(data).enumerate() {
        if x >= xlimit {
            y += 7;
            x = ox;

            if y >= ylimit {
                break;
            }
        }

        fb.blit(x, y, sprite);
        x += if i % 2 == 0 { 6 } else { 9 };
    }
}
