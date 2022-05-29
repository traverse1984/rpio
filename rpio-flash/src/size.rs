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

    pub fn protected_offset(&self, block_protect: u8) -> Option<u32> {
        let bp_bits = block_protect >> 2 & 0xF;

        if bp_bits > 0 {
            Some((0xFFFF0000 << (bp_bits - 1)) & self.last_addr())
        } else {
            None
        }
    }

    pub fn is_protected(&self, block_protect: u8, addr: u32) -> bool {
        self.protected_offset(block_protect)
            .map_or(false, |offset| addr < offset & self.0)
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
    }
}
