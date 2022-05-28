#[derive(Debug, Clone, Copy)]
pub enum Error {
    SPIChipSelect,
    SPIChipDeselect,
    SPITransfer,
    SPISetClockSpeed,
    FlashSizeNotSupported,
    AddressOutOfRange,
}
