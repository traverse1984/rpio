use core::fmt::LowerHex;

use super::buffer::*;
use super::error::Error;
use super::op::{Code, Type};
use super::size::Size;
use super::status::Status;
use rpio_spi::{Error as SpiError, SpiDevice, Transfer};

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct Device<SPI: SpiDevice, B: FlashBuffer> {
    spi: SPI,
    pub buf: B,
    size: Size,
    block_protect: u8,
}

impl<SPI: SpiDevice, B: FlashBuffer> Device<SPI, B> {
    pub fn new(spi: SPI, size: Size, buf: B) -> Self {
        Self {
            spi,
            size,
            buf,
            block_protect: 0xF,
        }
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

    pub fn write_status_enable(&mut self) -> Result {
        self.write_enable()?;
        self.buf.set_op(Code::WriteStatusEnable);
        self.send(Type::Op, 0)
    }

    pub fn write_block_protect_bits(&mut self, bp_bits: u8) -> Result {
        let bp_bits = (bp_bits & 0xF) << 2;

        self.write_status_enable()?;
        self.buf.set_op(Code::WriteStatus);
        *self.buf.get_mut(0) = bp_bits;
        self.send(Type::Op, 1)
    }

    pub fn write_byte(&mut self, addr: u32, byte: u8) -> Result {
        self.wait_ready()?;
        self.write_enable()?;

        self.buf.set_op_addr(Code::WriteByte, addr);
        *self.buf.get_mut(0) = byte;

        self.send(Type::OpAddr, 1)
    }

    pub fn read(&mut self, addr: u32, len: usize) -> Result<&[u8]> {
        self.buf.set_op_addr(Code::Read, addr);
        self.send(Type::OpAddr, len)?;
        Ok(&self.buf.data()[..len])
    }

    pub fn read_to_sector_end(&mut self, addr: u32) -> Result<&[u8]> {
        self.read(addr, 4096 - (addr & 0xFFF) as usize)
    }

    pub fn wait_ready(&mut self) -> Result {
        while self.read_status()?.is_busy() {}
        Ok(())
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
