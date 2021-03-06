#[repr(usize)]
pub enum Type {
    Op = 4,
    OpAddr = 1,
    ReadHighspeed = 0,
}

impl Type {
    pub fn to_offset(self) -> usize {
        self as usize
    }
}

pub enum Code {
    Read = 0x03,
    ReadStatus = 0x05,
    ReadHighspeed = 0x0B,
    ReadId = 0xAB,
    ReadJedecId = 0x9F,
    WriteByte = 0x02,
    WriteAutoIncrement = 0xAD,
    WriteStatus = 0x01,
    EraseSector = 0x20,
    EraseBlock32 = 0x52,
    EraseBlock64 = 0xD8,
    EraseChip = 0xC7,
    WriteEnable = 0x06,
    WriteStatusEnable = 0x50,
    WriteDisable = 0x04,
    BusyStatusOutputEnable = 0x70,
    BusyStatusOutputDisable = 0x80,
}

impl Code {
    pub fn to_instruction(self) -> u8 {
        self as u8
    }
}
