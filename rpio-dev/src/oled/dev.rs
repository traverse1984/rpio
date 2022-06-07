use super::types::Display;
use rpio_spi::{OutputPin, SpiDevice};

pub struct Dev<SPI: SpiDevice, DCMD: OutputPin> {
    spi: SPI,
    dcmd: DCMD,
}

impl<SPI: SpiDevice, DCMD: OutputPin> Dev<SPI, DCMD> {
    pub fn new(spi: SPI, dcmd: DCMD) -> Self {
        Self { spi, dcmd }
    }

    pub fn send_cmd(&mut self, cmd: u8) -> &mut Self {}

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

    pub fn set_display_start(&mut self, start: u8) -> &mut Self {
        self.cmd(0xDC).cmd(start)
    }

    pub fn set_addressing_mode(&mut self, mode: Mode) -> &mut Self {
        self.cmd(mode.to_cmd())
    }

    pub fn set_contrast(&mut self, contrast: u8) -> &mut Self {
        self.cmd(0x81).cmd(contrast)
    }

    pub fn set_remap(&mut self, remap: bool) -> &mut Self {
        self.cmd(0xA0 | remap as u8)
    }

    pub fn set_multiplex_ratio(&mut self, ratio: u8) -> &mut Self {
        self.cmd(0xA8).cmd(ratio)
    }

    pub fn display_on(&mut self) -> &mut Self {
        self.cmd(0xAF)
    }

    pub fn display_off(&mut self) -> &mut Self {
        self.cmd(0xAE)
    }

    pub fn force_display_on(&mut self, force: bool) -> &mut Self {
        self.cmd(0xA4 | force as u8)
    }

    /// Set reversed colours
    pub fn set_reversed(&mut self, reversed: bool) -> &mut Self {
        self.cmd(0xA6 + reversed as u8)
    }

    pub fn set_display_offset(&mut self, offset: u8) -> &mut Self {
        self.cmd(0xD3).cmd(offset)
    }

    pub fn set_reversed_scan(&mut self, reversed: bool) -> &mut Self {
        self.cmd(0xC0 & ((reversed as u8) << 3))
    }

    pub fn set_dclk_osc_freq(&mut self, setting: u8) -> &mut Self {
        self.cmd(0xD5).cmd(setting)
    }

    pub fn set_pre_charge_period(&mut self, setting: u8) -> &mut Self {
        self.cmd(0xD9).cmd(setting)
    }

    pub fn set_vcom_deselect_level(&mut self, level: u8) -> &mut Self {
        self.cmd(0xDB).cmd(level)
    }

    pub fn set_dc_converter(&mut self, setting: u8) -> &mut Self {
        self.cmd(0xAD).cmd(0x80 | setting & 0x0F)
    }

    pub fn init(&mut self) {
        // Based off the 'example code' for the device.
        // Need to figure out what all of this does.
        // self.cmd(0xAE); //  #turn off OLED display
        // self.cmd(0x00); //    #set lower column address
        // self.cmd(0x10); //    #set higher column address
        // self.cmd(0xB0); //    #set page address

        self.set_display_start(0);
        self.set_contrast(0x80);

        self.set_addressing_mode(Mode::Vertical);
        self.set_remap(false);
        self.set_multiplex_ratio(0x3f);

        self.set_reversed(false);
        self.set_reversed_scan(true);
        self.set_dclk_osc_freq(0x41);

        self.set_pre_charge_period(0x22);
        self.set_vcom_deselect_level(0x35);

        // I do not understand these, but it doesn't seem to matter
        // self.cmd(0xad); //     #set charge pump enable
        // self.cmd(0x8a); //     #Set DC-DC enable (a=0:disable; a=1:enable)

        self.display_on();
        self.set_display_offset(0x60);
        self.force_display_on(false);
        self.clear();
    }

    pub fn set_page_addr(&mut self, page: u8) -> &mut Self {
        self.cmd(0xB0 | page & 0x0F)
    }

    pub fn set_lower_col_addr(&mut self, col: u8) -> &mut Self {
        self.cmd(col & 0x0F)
    }

    pub fn set_higher_col_addr(&mut self, col: u8) -> &mut Self {
        self.cmd(0x10 | col & 0x7)
    }

    pub fn set_col(&mut self, col: u8) -> &mut Self {
        self.set_lower_col_addr(col).set_higher_col_addr(col >> 4)
    }

    pub fn clear(&mut self) {
        for y in 0u8..64 {
            self.draw_scan(y, 0);
        }
    }

    fn draw_scan(&mut self, y: u8, data: u128) {
        // self.cmd(0xb0 | (y / 4)); Getting somewhere for paged mode
        // self.cmd(0x00);
        self.set_col(y);
        // self.cmd(0x00 + (y & 0x0f));
        // self.cmd(0x10 + (y >> 4));

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
