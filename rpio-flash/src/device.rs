use core::fmt::LowerHex;

use super::buffer::*;
use super::error::Error;
use super::op::{Code, Type};
use super::status::Status;
use rpio_spi::{Error as SpiError, SpiDevice, Transfer};

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Device<SPI: SpiDevice, B: FlashBuffer> {
    spi: SPI,
    pub buf: B,
}

impl<SPI: SpiDevice, B: FlashBuffer> Device<SPI, B> {
    pub fn new(spi: SPI, buf: B) -> Self {
        Self { spi, buf }
    }

    pub fn send(&mut self, op: Type, data_len: usize) -> Result {
        let buf = self.buf.op(op, data_len);

        self.spi.transfer(buf).map_err(|err| match err {
            SpiError::Transfer => Error::SPITransfer,
            SpiError::ChipSelect => Error::SPIChipSelect,
            SpiError::ChipDeselect => Error::SPIChipDeselect,
            SpiError::ClockSpeed => Error::SPISetClockSpeed,
            SpiError::NotImplemented => unreachable!(),
        })?;

        Ok(())
    }

    pub fn read_status(&mut self) -> Result<Status> {
        self.buf.set_op(Code::ReadStatus);
        self.send(Type::Op, 1)?;
        Ok(Status::from(*self.buf.get(0)))
    }

    pub fn write_enable(&mut self) -> Result {
        self.buf.set_op(Code::WriteEnable);
        self.send(Type::Op, 0)
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
