use super::size::Size;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Status(u8);

impl Status {
    pub fn is_busy(&self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn is_write_enabled(&self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn is_auto_increment_mode(&self) -> bool {
        self.0 & 0x40 != 0
    }

    pub fn is_block_protect_locked(&self) -> bool {
        self.0 & 0x80 != 0
    }

    pub fn block_protect_bits(&self) -> u8 {
        self.0 >> 2 & 0xF
    }
}

mod tests {
    use super::{Size, Status};

    #[test]
    fn status() {
        assert!(!Status(0).is_busy());
        assert!(!Status(0).is_write_enabled());
        assert!(!Status(0).is_auto_increment_mode());
        assert!(!Status(0).is_block_protect_locked());

        assert!(Status(0b00000001).is_busy());
        assert!(Status(0b00000010).is_write_enabled());
        assert!(Status(0b01000000).is_auto_increment_mode());
        assert!(Status(0b10000000).is_block_protect_locked());

        assert_eq!(Status(0b00000100).block_protect_bits(), 0x1);
        assert_eq!(Status(0b00001000).block_protect_bits(), 0x2);
        assert_eq!(Status(0b00010000).block_protect_bits(), 0x4);
        assert_eq!(Status(0b00100000).block_protect_bits(), 0x8);
        assert_eq!(Status(0b00111100).block_protect_bits(), 0xF);
    }
}

impl Into<u8> for Status {
    fn into(self) -> u8 {
        self.0
    }
}

impl From<u8> for Status {
    fn from(status: u8) -> Self {
        Self(status)
    }
}
