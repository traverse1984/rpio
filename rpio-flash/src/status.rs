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
    extern crate std;
    use super::{Size, Status};
    use std::println;

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

        // let mb16 = Size::from_mb(16).unwrap();

        // assert_eq!(Status(0b00000000).protected_offset(mb16), None);
        // assert_eq!(Status(0b00000100).protected_offset(mb16), Some(0xFF0000));
        // assert_eq!(Status(0b00001000).protected_offset(mb16), Some(0xFE0000));
        // assert_eq!(Status(0b00001100).protected_offset(mb16), Some(0xFC0000));
        // assert_eq!(Status(0b00010000).protected_offset(mb16), Some(0xF80000));
        // assert_eq!(Status(0b00010100).protected_offset(mb16), Some(0xF00000));
        // assert_eq!(Status(0b00011000).protected_offset(mb16), Some(0xE00000));
        // assert_eq!(Status(0b00011100).protected_offset(mb16), Some(0xC00000));
        // assert_eq!(Status(0b00100000).protected_offset(mb16), Some(0x800000));

        // let mb1 = Size::from_mb(1).unwrap();

        // assert_eq!(Status(0b00000000).protected_offset(mb1), None);
        // assert_eq!(Status(0b00000100).protected_offset(mb1), Some(0xF0000));
        // assert_eq!(Status(0b00001000).protected_offset(mb1), Some(0xE0000));
        // assert_eq!(Status(0b00001100).protected_offset(mb1), Some(0xC0000));
        // assert_eq!(Status(0b00010000).protected_offset(mb1), Some(0x80000));
        // assert_eq!(Status(0b00010100).protected_offset(mb1), Some(0x00000));
        // assert_eq!(Status(0b00011000).protected_offset(mb1), Some(0x00000));
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
