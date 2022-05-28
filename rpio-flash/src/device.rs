use super::buffer::*;
use super::error::Error;
use super::opcode::Opcode;
use rpio_spi::{Error as SpiError, SpiDevice, Transfer};

pub type Result<T = ()> = core::result::Result<T, Error>;

fn spi_to_flash_error(err: SpiError) -> Error {
    match err {
        SpiError::Transfer => Error::SPITransfer,
        SpiError::ChipSelect => Error::SPIChipSelect,
        SpiError::ChipDeselect => Error::SPIChipDeselect,
        SpiError::ClockSpeed => Error::SPISetClockSpeed,
        SpiError::NotImplemented => panic!("SpiDevice feature not implemented."),
    }
}

#[derive(Debug)]
pub struct Device<SPI: SpiDevice, B: FlashBuffer> {
    spi: SPI,
    buf: B,
}

impl<SPI: SpiDevice, B: FlashBuffer> Device<SPI, B> {
    pub fn new(spi: SPI, buf: B) -> Self {
        Self { spi, buf }
    }

    fn transfer_op(&mut self, data_len: usize) -> Result<&[u8]> {
        let buf = self.buf.op(data_len);
        self.spi.transfer(buf).map_err(spi_to_flash_error)
    }

    fn transfer_op_addr(&mut self, data_len: usize) -> Result<&[u8]> {
        let buf = self.buf.op_addr(data_len);
        self.spi.transfer(buf).map_err(spi_to_flash_error)
    }

    fn transfer_highspeed_read(&mut self, data_len: usize) -> Result<&[u8]> {
        let buf = self.buf.highspeed_read(data_len);
        self.spi.transfer(buf).map_err(spi_to_flash_error)
    }

    pub fn read_status(&mut self) -> Result<u8> {
        self.buf.set_op(Opcode::ReadStatus);
        self.transfer_op(1).map(|buf| buf[0])
    }
}

// Read = 0x03,
// ReadStatus = 0x05,
// ReadHighspeed = 0x0B,
// ReadId = 0xAB,
// ReadJedecId = 0x9F,
// WriteByte = 0x02,
// WriteAutoIncrement = 0xAD,
// WriteStatus = 0x01,
// EraseSector = 0x20,
// EraseBlock32 = 0x52,
// EraseBlock64 = 0xD8,
// EraseChip = 0xC7,
// WriteEnable = 0x06,
// WriteStatusEnable = 0x50,
// WriteDisable = 0x04,
// BusyStatusOutputEnable = 0x70,
// BusyStatusOutputDisable = 0x80,
