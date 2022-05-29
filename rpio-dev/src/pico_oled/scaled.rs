use super::types::{Draw, FrameBuffer};

pub struct ScaledBuf {
    buf: [u128; 64],
    wrap_x: bool,
    wrap_y: bool,
}

impl ScaledBuf {
    pub fn new() -> Self {
        Self {
            buf: [0; 64],
            wrap_x: true,
            wrap_y: true,
        }
    }

    fn write_blit(&mut self, y: usize, scan: <Self as FrameBuffer>::Scan) -> bool {
        if y < self.height() || self.wrap_y {
            let y = (y % 32) * 2;
            let scan = wide_u64(scan) ^ self.buf[y];

            self.buf[y] = scan;
            self.buf[y + 1] = scan;
            true
        } else {
            false
        }
    }
}

impl FrameBuffer for ScaledBuf {
    type Scan = u64;
    type ScanOut = u128;

    fn width(&self) -> usize {
        64
    }

    fn height(&self) -> usize {
        32
    }

    fn clear(&mut self) {
        self.buf = [0; 64];
    }

    fn read(&self) -> &[u128] {
        &self.buf
    }

    fn write_scan(&mut self, y: usize, scan: Self::Scan) -> bool {
        if y < self.height() || self.wrap_y {
            let y = (y % 32) * 2;
            let scan = wide_u64(scan);

            self.buf[y] = scan;
            self.buf[y + 1] = scan;
            true
        } else {
            false
        }
    }

    fn write(&mut self, y: usize, scans: &[Self::Scan]) {
        for (offset, &scan) in scans.iter().enumerate() {
            if !self.write_scan(y + offset, scan) {
                break;
            }
        }
    }

    fn wrap_range_left(&mut self, y0: usize, y1: usize, shl: u32) {
        for y in y0..y1 {
            self.buf[y] = self.buf[y % 32].rotate_left(shl);
            self.buf[y + 1] = self.buf[y % 32].rotate_left(shl);
        }
    }

    fn wrap_range_right(&mut self, y0: usize, y1: usize, shr: u32) {
        for y in y0..y1 {
            self.buf[y] = self.buf[y % 32].rotate_left(shr);
            self.buf[y + 1] = self.buf[y % 32].rotate_left(shr);
        }
    }

    fn wrap_left(&mut self, shl: u32) {
        self.wrap_range_left(0, 32, shl);
    }

    fn wrap_right(&mut self, shr: u32) {
        self.wrap_range_right(0, 32, shr);
    }
}

macro_rules! impl_draw {
    ($for: ident <$type: ty : $into: ty> : $shr: expr) => {
        impl Draw<$type> for $for {
            fn draw(&mut self, x: usize, y: usize, scans: &[$type]) {
                let wrap = $shr + (x as u32);
                for (offset, &scan) in scans.iter().enumerate() {
                    let scan = if self.wrap_x {
                        (scan as $into).rotate_right(wrap)
                    } else {
                        (scan as $into) >> wrap
                    };

                    if !self.write_scan(y + offset, scan) {
                        break;
                    }
                }
            }

            fn blit(&mut self, x: usize, y: usize, scans: &[$type]) {
                let wrap = $shr + (x as u32);
                for (offset, &scan) in scans.iter().enumerate() {
                    let scan = if self.wrap_x {
                        (scan as $into).rotate_right(wrap)
                    } else {
                        (scan as $into) >> wrap
                    };

                    if !self.write_blit(y + offset, scan) {
                        break;
                    }
                }
            }
        }
    };
}

impl_draw!(ScaledBuf<u64:u64>: 64);
impl_draw!(ScaledBuf<u32:u64>: 32);
impl_draw!(ScaledBuf<u16:u64>: 16);
impl_draw!(ScaledBuf<u8 :u64>: 8);

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

#[inline]
fn wide_u32(val: u32) -> u64 {
    let (msb, lsb) = (((val & 0xFFFF0000) >> 16), val & 0xFFFF);
    ((wide_u16(msb as u16) as u64) << 32) | (wide_u16(lsb as u16) as u64)
}

#[inline]
fn wide_u64(val: u64) -> u128 {
    let (msb, lsb) = (((val & 0xFFFFFFFF00000000) >> 32), val & 0xFFFFFFFF);
    ((wide_u32(msb as u32) as u128) << 64) | (wide_u32(lsb as u32) as u128)
}
