use super::{seq::KeySeqIter, Keypad, Keys};
use embedded_hal::digital::v2::{InputPin, OutputPin};
use rpio_gpio as rpio;

/// A keypad implemented using eight GPIO pins.
pub struct GpioKeypad<C1, C2, C3, C4, R1, R2, R3, R4>
where
    C1: OutputPin + InputPin,
    C2: OutputPin + InputPin,
    C3: OutputPin + InputPin,
    C4: OutputPin + InputPin,
    R1: InputPin,
    R2: InputPin,
    R3: InputPin,
    R4: InputPin,
{
    col1: C1,
    col2: C2,
    col3: C3,
    col4: C4,
    row1: R1,
    row2: R2,
    row3: R3,
    row4: R4,
    keymap: [[u8; 4]; 4],
}

impl<C1, C2, C3, C4, R1, R2, R3, R4> GpioKeypad<C1, C2, C3, C4, R1, R2, R3, R4>
where
    C1: OutputPin + InputPin,
    C2: OutputPin + InputPin,
    C3: OutputPin + InputPin,
    C4: OutputPin + InputPin,
    R1: InputPin,
    R2: InputPin,
    R3: InputPin,
    R4: InputPin,
{
    const DEFAULT_KEYMAP: [[u8; 4]; 4] = [
        [0x1, 0x2, 0x3, 0xF],
        [0x4, 0x5, 0x6, 0xE],
        [0x7, 0x8, 0x9, 0xD],
        [0xA, 0x0, 0xB, 0xC],
    ];

    pub fn new(
        col1: C1,
        col2: C2,
        col3: C3,
        col4: C4,
        row1: R1,
        row2: R2,
        row3: R3,
        row4: R4,
    ) -> Self {
        let mut keypad = Self {
            col1,
            col2,
            col3,
            col4,
            row1,
            row2,
            row3,
            row4,
            keymap: Self::DEFAULT_KEYMAP,
        };

        keypad.reset();
        keypad
    }

    pub fn with_keymap(mut self, keymap: [[u8; 4]; 4]) -> Self {
        self.keymap = keymap;
        self
    }

    fn reset(&mut self) {
        rpio::write!(self.col1, self.col2, self.col3, self.col4 => true);
    }

    fn read_key(&self, col: usize) -> Option<u8> {
        let key = rpio::read!(u8; self.row4, self.row3, self.row2, self.row1);

        match key {
            8..=15 => Some(3),
            4..=7 => Some(2),
            2 | 3 => Some(1),
            1 => Some(0),
            _ => None,
        }
        .map(|row| self.keymap[row][col])
    }
}

impl<C1, C2, C3, C4, R1, R2, R3, R4> Keypad for GpioKeypad<C1, C2, C3, C4, R1, R2, R3, R4>
where
    C1: OutputPin + InputPin,
    C2: OutputPin + InputPin,
    C3: OutputPin + InputPin,
    C4: OutputPin + InputPin,
    R1: InputPin,
    R2: InputPin,
    R3: InputPin,
    R4: InputPin,
{
    fn key_is_pressed(&self) -> bool {
        rpio::read!(any true; self.row4, self.row3, self.row2, self.row1)
    }

    #[inline(never)]
    fn read(&mut self) -> Option<u8> {
        if !self.key_is_pressed() {
            return None;
        }

        for pos in 0..4 {
            rpio::write!(self.col4, self.col3, self.col2, self.col1 => 4 bit => 1 << pos);

            if let Some(key) = self.read_key(pos) {
                self.reset();
                return Some(key);
            }
        }

        self.reset();
        None
    }

    fn read_seq<'a, 'b>(&'a mut self, buf: &'b mut [u8]) -> KeySeqIter<'a, 'b, Self> {
        KeySeqIter::new(self, buf)
    }

    fn read_multi(&mut self) -> Option<Keys> {
        if !self.key_is_pressed() {
            return None;
        }

        let mut count = 0;
        let mut buf = [0u8; 4];

        for pos in 0..4 {
            rpio::write!(self.col4, self.col3, self.col2, self.col1 => 4 bit => 1 << pos);

            if let Some(key) = self.read_key(pos) {
                buf[count] = key;
                count += 1;
            }
        }

        self.reset();

        match count {
            0 => None,
            1 => Some(Keys::One(buf[0])),
            2 => Some(Keys::Two(buf[0], buf[1])),
            3 => Some(Keys::Three(buf[0], buf[1], buf[2])),
            4 => Some(Keys::Four(buf[0], buf[1], buf[2], buf[3])),
            _ => unreachable!(),
        }
    }
}
