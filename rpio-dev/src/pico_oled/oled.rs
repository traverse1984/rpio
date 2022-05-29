use super::types::Display;
use rpio_spi::{OutputPin, SpiDevice};

pub struct PicoOled<SPI: SpiDevice, DCMD: OutputPin> {
    buf: [u128; 64],
    spi: SPI,
    dcmd: DCMD,
}

impl<SPI: SpiDevice, DCMD: OutputPin> PicoOled<SPI, DCMD> {
    pub fn new(spi: SPI, dcmd: DCMD) -> Self {
        Self {
            buf: [0; 64],
            spi,
            dcmd,
        }
    }

    pub fn cmd(&mut self, cmd: u8) -> &mut Self {
        self.dcmd.set_low().ok();
        self.spi.transfer(&mut [cmd]).ok();
        self
    }

    pub fn data(&mut self, data: u8) -> &mut Self {
        self.dcmd.set_high().ok();
        self.spi.transfer(&mut [data]).ok();
        self
    }

    pub fn init(&mut self) {
        // Based off the 'example code' for the device.
        // Need to figure out what all of this does.
        self.cmd(0xAE); //  #turn off OLED display
        self.cmd(0x00); //    #set lower column address
        self.cmd(0x10); //    #set higher column address
        self.cmd(0xB0); //    #set page address
        self.cmd(0xdc); //     #et display start line
        self.cmd(0x00); //
        self.cmd(0x81); //     #contract control
        self.cmd(0x6f); //     #128
        self.cmd(0x21); //    # Set Memory addressing mode (0x20/0x21);  #
        self.cmd(0xa0); //     #set segment remap
        self.cmd(0xc0); //     #Com scan direction
        self.cmd(0xa4); //   #Disable Entire Display On (0xA4/0xA5);
        self.cmd(0xa6); //     #normal / reverse
        self.cmd(0xa8); //     #multiplex ratio
        self.cmd(0x3f); //     #duty = 1/64
        self.cmd(0xd3); //     #set display offset
        self.cmd(0x60); //
        self.cmd(0xd5); //     #set osc division
        self.cmd(0x41); //
        self.cmd(0xd9); //     #set pre-charge period
        self.cmd(0x22); //
        self.cmd(0xdb); //     #set vcomh
        self.cmd(0x35); //
        self.cmd(0xad); //     #set charge pump enable
        self.cmd(0x8a); //     #Set DC-DC enable (a=0:disable; a=1:enable)
        self.cmd(0xAF); //

        self.clear();
    }

    pub fn clear(&mut self) {
        for y in 0u8..64 {
            self.draw_scan(y, 0);
        }
    }

    fn draw_scan(&mut self, y: u8, data: u128) {
        self.cmd(0xb0);
        self.cmd(0x00 + (y & 0x0f));
        self.cmd(0x10 + (y >> 4));

        for byte in data.to_be_bytes() {
            self.data(byte.reverse_bits());
        }
    }
}

impl<SPI: SpiDevice, DCMD: OutputPin> Display for PicoOled<SPI, DCMD> {
    fn update(&mut self, fb: &[u128]) {
        for (offset, &scan) in fb.iter().enumerate() {
            let y = 63 - (offset % 64);
            if self.buf[y] != scan {
                self.buf[y] = scan;
                self.draw_scan(y as u8, scan);
            }
        }
    }
}
