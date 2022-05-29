use super::Error;
use core::convert::TryFrom;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Size(u32);

impl Size {
    // pub const MBIT: u32 = 131072;
    pub const MB: u32 = 1048576;

    pub fn is_addr(&self, addr: u32) -> bool {
        addr < self.0
    }

    pub fn size(&self) -> u32 {
        self.0
    }

    pub fn last_addr(&self) -> u32 {
        self.0 - 1
    }

    pub fn from_mb(mb: u32) -> Result<Self, Error> {
        let size = mb.checked_mul(Self::MB).ok_or(Error::ChipSize)?;
        Self::try_from(size)
    }

    pub fn protected_offset(&self, bp_bits: u8) -> Option<u32> {
        if bp_bits > 0 {
            Some((0xFFFF0000 << (bp_bits - 1)) & self.last_addr())
        } else {
            None
        }
    }

    pub fn is_protected(&self, addr: u32, block_protect: u8) -> bool {
        self.protected_offset(block_protect)
            .map_or(false, |offset| addr >= offset & self.last_addr())
    }
}

impl Into<u32> for Size {
    fn into(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Size {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        // let block = if value < Self::MB {
        //     Self::MBIT
        // } else {
        //     Self::MB
        // };

        if value > 0 && value <= 64 * Self::MB && value % Self::MB == 0 {
            Ok(Size(value))
        } else {
            Err(Error::ChipSize)
        }
    }
}

mod tests {
    use super::{Error, Size};

    #[test]
    fn size() {
        assert_eq!(Size::from_mb(1), Ok(Size(Size::MB)));

        // assert_eq!(Size::from_mbit(1), Ok(Size(Size::MBIT)));
        // assert_eq!(Size::from_mbit(8), Ok(Size(Size::MB)));
        // assert_eq!(Size::from_mbit(16), Ok(Size(Size::MB * 2)));

        //assert_eq!(Size::try_from(131072), Ok(Size(Size::MBIT)));
        assert_eq!(Size::try_from(1048576), Ok(Size(Size::MB)));

        assert_eq!(Size::from_mb(0), Err(Error::ChipSize));
        assert_eq!(Size::from_mb(65), Err(Error::ChipSize));
        // assert_eq!(Size::from_mbit(0), Err(Error::ChipSize));
        // assert_eq!(Size::from_mbit(9), Err(Error::ChipSize));
        assert_eq!(Size::try_from(10), Err(Error::ChipSize));

        let mb1 = Size::from_mb(1).unwrap();

        assert_eq!(mb1.protected_offset(0b000), None);
        assert_eq!(mb1.protected_offset(0b001), Some(0xF0000));
        assert_eq!(mb1.protected_offset(0b010), Some(0xE0000));
        assert_eq!(mb1.protected_offset(0b011), Some(0xC0000));
        assert_eq!(mb1.protected_offset(0b100), Some(0x80000));
        assert_eq!(mb1.protected_offset(0b101), Some(0x00000));
        assert_eq!(mb1.protected_offset(0b110), Some(0x00000));

        assert_eq!(mb1.is_protected(mb1.last_addr(), 0b000), false);
        assert_eq!(mb1.is_protected(mb1.last_addr(), 0b001), true);
        assert_eq!(mb1.is_protected(0, 0b001), false);
        assert_eq!(mb1.is_protected(0, 0b111), true);

        let mb16 = Size::from_mb(8).unwrap();

        assert_eq!(mb16.protected_offset(0b0000), None);
        assert_eq!(mb16.protected_offset(0b0001), Some(0x7F0000));
        assert_eq!(mb16.protected_offset(0b0010), Some(0x7E0000));
        assert_eq!(mb16.protected_offset(0b0011), Some(0x7C0000));
        assert_eq!(mb16.protected_offset(0b0100), Some(0x780000));
        assert_eq!(mb16.protected_offset(0b0101), Some(0x700000));
        assert_eq!(mb16.protected_offset(0b0110), Some(0x600000));
        assert_eq!(mb16.protected_offset(0b0111), Some(0x400000));
        assert_eq!(mb16.protected_offset(0b1000), Some(0x000000));

        assert_eq!(mb16.is_protected(mb16.last_addr(), 0b0000), false);
        assert_eq!(mb16.is_protected(mb16.last_addr(), 0b0001), true);
        assert_eq!(mb16.is_protected(0, 0b1000), true);
    }
}
