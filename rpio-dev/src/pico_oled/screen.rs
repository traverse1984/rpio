use super::{ascii, Display, Draw, FrameBuffer};
use core::fmt;

pub struct Screen<D: Display, B: Draw<u8>> {
    display: D,
    buf: B,
    ox: usize,
    oy: usize,
    x: usize,
    y: usize,
}

impl<D: Display, B: Draw<u8> + FrameBuffer<ScanOut = u128>> Screen<D, B> {
    pub fn new(display: D, buf: B) -> Self {
        Self {
            display,
            buf,
            ox: 2,
            oy: 2,
            x: 2,
            y: 2,
        }
    }

    pub fn set_offset(&mut self, x: usize, y: usize) -> &mut Self {
        self.ox = x;
        self.oy = y;
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_cur(&mut self, x: usize, y: usize) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn read(&self) -> &[<B as FrameBuffer>::ScanOut] {
        self.buf.read()
    }

    pub fn buf(&mut self) -> &mut B {
        &mut self.buf
    }

    pub fn display(&mut self) -> &mut D {
        &mut self.display
    }

    pub fn update(&mut self) {
        self.display.update(self.buf.read());
    }

    pub fn clear(&mut self) -> &mut Self {
        self.x = self.ox;
        self.y = self.oy;
        self.buf.clear();
        self
    }

    pub fn write(&mut self, s: &str) -> &mut Self {
        for sprite in ascii::string(s) {
            if self.x + 7 >= self.buf.width() || ascii::char(b'\n') == sprite {
                self.y += 7;
                self.x = self.ox;

                if ascii::char(b'\n') == sprite {
                    continue;
                }

                if self.y >= self.buf.height() {
                    break;
                }
            }

            self.buf.blit(self.x, self.y, sprite);
            self.x += 6;
        }

        self
    }

    pub fn hexdump(&mut self, data: &[u8]) -> &mut Self {
        for (i, sprite) in ascii::hexstr(data).enumerate() {
            if self.x >= self.buf.width() {
                self.y += 7;
                self.x = self.ox;

                if self.y >= self.buf.height() {
                    break;
                }
            }

            self.buf.blit(self.x, self.y, sprite);
            self.x += if i % 2 == 0 { 6 } else { 9 };
        }

        self
    }

    pub fn destroy(self) -> (D, B) {
        (self.display, self.buf)
    }
}

impl<D: Display, B: Draw<u8> + FrameBuffer<ScanOut = u128>> fmt::Write for Screen<D, B> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! screen {
    // ($spi: expr, $dcmd: expr, $buf: expr) => {{
    //     let mut oled = rpio::dev::pico_oled::PicoOled::new($spi, $dcmd);
    //     oled.init();
    //     rpio::dev::pico_oled::Screen::new(oled, $buf)
    // }};

    // ($spi: expr, $dcmd: expr) => {{
    //     let buf = rpio::dev::pico_oled::FrameBuf::new();
    //     rpio::dev::screen!($spi, $dcmd, buf)
    // }};
    ($oled: expr, $buf: expr) => {{
        rpio::dev::pico_oled::Screen::new($oled, $buf)
    }};

    ($oled: expr) => {{
        let buf = rpio::dev::pico_oled::FrameBuf::new();
        rpio::dev::pico_oled::Screen::new($oled, buf)
    }};
}

#[macro_export]
macro_rules! use_screen {
    ($screen: expr) => {
        rpio::__use_screen!(($) $screen)
    };
}

#[macro_export]
macro_rules! __use_screen {
    (($d:tt) $screen: expr) => {
        macro_rules! println {
                                            ($d($tt: tt)*) => {
                                                $screen.clear();
                                                $screen.write_fmt(format_args!($d($tt)*)).ok();
                                                $screen.write_raw("\n");
                                                $screen.update();
                                            };
                                        }

        macro_rules! hex {
                                            ($d($tt: tt)*) => {
                                                $screen.hexdump($d($tt)*);
                                            };
                                        }

        macro_rules! print {
                                                ($d($tt: tt)*) => {
                                                    $screen.clear();
                                                    $screen.write_fmt(format_args!($d($tt)*)).ok();
                                                    $screen.update();
                                                };
                                        }

        macro_rules! offset {
                                            ($d($tt: tt)*) => {
                                                $screen.set_offset($d($tt)*);
                                            };
                                        }

        macro_rules! cur {
                                            ($d($tt: tt)*) => {
                                                $screen.set_cur($d($tt)*);
                                            };
                                        }

        macro_rules! draw {
                                            (fmtln $d($tt: tt)*) => {
                                                $screen.write_fmt(format_args!($d($tt)*)).ok();
                                                $screen.write("\n");
                                            };

                                            (fmt $d($tt:tt)*) => {
                                                $screen.write_fmt(format_args!($d($tt)*)).ok();
                                            };

                                            (txt $d($tt: tt)*) => {
                                                $screen.write($d($tt)*)
                                            };

                                            (blit $d($tt: tt)*) => {
                                                $screen.buf().blit($d($tt)*);
                                            };

                                            ($d($tt: tt)*) => {
                                                $screen.buf().draw($d($tt)*);
                                            }
                                        }

        macro_rules! update {
            () => {
                $screen.update();
            };
        }

        macro_rules! clear {
            () => {
                $screen.clear();
            };

            (now) => {
                $screen.clear().update();
            };
        }
    };
}
