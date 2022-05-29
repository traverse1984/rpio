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

    pub fn protected_offset(&self) -> Option<u32> {
        let bp_bits = self.0 >> 2 & 0xF;

        if bp_bits > 0 {
            Some(0xFFFF0000 << (bp_bits - 1))
        } else {
            None
        }
    }
}

mod tests {
    use super::Status;

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

        assert_eq!(Status(0b00000000).protected_offset(), None);
        assert_eq!(Status(0b00000100).protected_offset(), Some(0xFFFF0000));
        assert_eq!(Status(0b00001000).protected_offset(), Some(0xFFFE0000));
        assert_eq!(Status(0b00001100).protected_offset(), Some(0xFFFC0000));
        assert_eq!(Status(0b00010000).protected_offset(), Some(0xFFF80000));
        assert_eq!(Status(0b00010100).protected_offset(), Some(0xFFF00000));
        assert_eq!(Status(0b00011000).protected_offset(), Some(0xFFE00000));
        assert_eq!(Status(0b00011100).protected_offset(), Some(0xFFC00000));
        assert_eq!(Status(0b00100000).protected_offset(), Some(0xFF800000));
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
