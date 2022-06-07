use super::print::print;
use super::types::{Draw, FrameBuffer};
use core::fmt;

pub struct FrameBuf {
    buf: [u128; 64],
    wrap_x: bool,
    wrap_y: bool,
}

impl FrameBuf {
    pub fn new() -> Self {
        Self {
            buf: [0; 64],
            wrap_x: true,
            wrap_y: true,
        }
    }
}

impl fmt::Write for FrameBuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(self, 2, 2, s);
        Ok(())
    }
}

impl FrameBuffer for FrameBuf {
    type Scan = u128;
    type ScanOut = u128;

    fn width(&self) -> usize {
        128
    }

    fn height(&self) -> usize {
        64
    }

    fn clear(&mut self) {
        self.buf = [0; 64];
    }

    fn read(&self) -> &[u128] {
        &self.buf
    }

    fn write_scan(&mut self, y: usize, scan: Self::Scan) -> bool {
        if y < self.height() || self.wrap_y {
            self.buf[y % self.height()] = scan;
            true
        } else {
            false
        }
    }

    fn write(&mut self, y: usize, scans: &[Self::Scan]) {
        for (offset, &scan) in scans.iter().enumerate() {
            self.buf[(y + offset) % self.height()] = scan;
        }
    }

    fn wrap_range_left(&mut self, y0: usize, y1: usize, shl: u32) {
        for y in y0..y1 {
            self.buf[y] = self.buf[y % self.height()].rotate_left(shl);
        }
    }

    fn wrap_range_right(&mut self, y0: usize, y1: usize, shr: u32) {
        for y in y0..y1 {
            self.buf[y] = self.buf[y % self.height()].rotate_left(shr);
        }
    }

    fn wrap_left(&mut self, shl: u32) {
        self.wrap_range_left(0, self.height(), shl);
    }

    fn wrap_right(&mut self, shr: u32) {
        self.wrap_range_right(0, self.height(), shr);
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
                        (scan as $into).rotate_right(wrap) ^ self.buf[(y + offset) % self.height()]
                    } else {
                        (scan as $into) >> wrap
                    };

                    if !self.write_scan(y + offset, scan) {
                        break;
                    }
                }
            }
        }
    };
}

impl_draw!(FrameBuf<u128:u128>: 0);
impl_draw!(FrameBuf<u64 :u128>: 64);
impl_draw!(FrameBuf<u32 :u128>: 32);
impl_draw!(FrameBuf<u16 :u128>: 16);
impl_draw!(FrameBuf<u8  :u128>: 8);
