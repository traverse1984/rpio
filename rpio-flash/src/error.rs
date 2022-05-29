#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ChipSize,
    SPIChipSelect,
    SPIChipDeselect,
    SPITransfer,
    SPISetClockSpeed,
    FlashSizeNotSupported,
    AddressOutOfRange,
}
