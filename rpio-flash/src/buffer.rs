use super::opcode::Opcode;

pub trait FlashBuffer {
    /// Get the size of the storage element of the buffer
    fn len(&self) -> usize;

    /// Return the number of bytes of data copied
    fn write(&mut self, data: &[u8]) -> usize;

    /// Get a single byte, unchecked
    fn get(&self, index: usize) -> &u8;

    /// Get a mutable byte reference
    fn get_mut(&mut self, index: usize) -> &mut u8;

    /// Get raw ref to data
    fn data(&self) -> &[u8];

    /// Get mutable ref to data
    fn data_mut(&mut self) -> &mut [u8];

    /// Set an operation-only flag and return a slice
    fn set_op(&mut self, op: Opcode);
    fn op(&mut self, data_len: usize) -> &mut [u8];

    /// Set an op+addr instruction and return a slice
    fn set_op_addr(&mut self, op: Opcode, addr: u32);
    fn op_addr(&mut self, data_len: usize) -> &mut [u8];

    /// Set the highspeed instruction and return a slice
    fn set_highspeed_read(&mut self, addr: u32);
    fn highspeed_read(&mut self, data_len: usize) -> &mut [u8];
}

// Can probably be generic
#[derive(Debug)]
pub struct Buffer<const LEN: usize> {
    buf: [u8; LEN],
}

impl<const LEN: usize> Buffer<LEN> {
    pub fn new() -> Self {
        if LEN < 6 {
            panic!("Buffer can't be smaller than 6 bytes.");
        }
        Self { buf: [0; LEN] }
    }

    pub fn new4k() -> Buffer<5001> {
        Buffer::<5001>::new()
    }
}

impl<const LEN: usize> FlashBuffer for Buffer<LEN> {
    fn len(&self) -> usize {
        LEN - 5
    }

    fn set_op(&mut self, op: Opcode) {
        self.buf[4] = op.to_instruction();
    }

    fn op(&mut self, data_len: usize) -> &mut [u8] {
        &mut self.buf[4..data_len + 5]
    }

    fn set_op_addr(&mut self, op: Opcode, addr: u32) {
        let addr_space = &mut self.buf[1..5];
        addr_space.copy_from_slice(&addr.to_be_bytes());
        self.buf[1] = op.to_instruction();
    }

    fn op_addr(&mut self, data_len: usize) -> &mut [u8] {
        &mut self.buf[1..data_len + 5]
    }

    fn set_highspeed_read(&mut self, addr: u32) {
        let addr_space = &mut self.buf[..4];
        addr_space.copy_from_slice(&addr.to_be_bytes());
        self.buf[0] = Opcode::ReadHighspeed.to_instruction();
    }

    fn highspeed_read(&mut self, data_len: usize) -> &mut [u8] {
        &mut self.buf[..data_len + 5]
    }

    fn get(&self, index: usize) -> &u8 {
        &self.buf[index + 5]
    }

    fn get_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.buf[index + 5]
    }

    fn data(&self) -> &[u8] {
        &self.buf[5..]
    }

    fn data_mut(&mut self) -> &mut [u8] {
        &mut self.buf[5..]
    }

    fn write(&mut self, data: &[u8]) -> usize {
        let copy = self.len().min(data.len());
        (&mut self.buf[5..copy + 5]).copy_from_slice(&data[..copy]);
        copy
    }
}

mod tests {
    use super::{Buffer, FlashBuffer, Opcode};

    #[test]
    fn data() {
        let mut buf = Buffer::<9>::new();

        assert_eq!(buf.len(), 4);

        assert_eq!(buf.write(&[1, 2, 3, 4]), 4);
        assert_eq!(buf.data(), &[1, 2, 3, 4]);

        buf.set_op(Opcode::WriteByte);
        assert_eq!(buf.op(4), [Opcode::WriteByte.to_instruction(), 1, 2, 3, 4]);

        buf.set_op_addr(Opcode::Read, 0x00123456);
        assert_eq!(
            buf.op_addr(4),
            &[Opcode::Read.to_instruction(), 0x12, 0x34, 0x56, 1, 2, 3, 4]
        );

        buf.set_highspeed_read(0x00ABCDEF);
        let hs_read = Opcode::ReadHighspeed.to_instruction();
        assert_eq!(
            buf.highspeed_read(4),
            &[hs_read, 0xAB, 0xCD, 0xEF, 0x56, 1, 2, 3, 4]
        );

        let countdown: [u8; 8] = [8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(buf.write(&countdown), 4);
        assert_eq!(buf.data(), &[8, 7, 6, 5]);

        *buf.get_mut(0) = 9;
        assert_eq!(buf.get(0), &9);
        assert_eq!(buf.data()[0], 9);
    }
}
