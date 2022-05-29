use super::sprites::SPRITE;

const UNKNOWN: [u8; 5] = [0xF8; 5];
const SPACE: [u8; 5] = [0; 5];
const NL: [u8; 5] = [0, 0, 0, 0, 1];

pub fn char(char: u8) -> &'static [u8; 5] {
    let char = char as usize;

    match char {
        10 => &NL,
        32 => &SPACE,
        48..=57 => &SPRITE[char - 48],
        65..=90 => &SPRITE[char - 55],
        97..=122 => &SPRITE[char - 87],
        63 => &SPRITE[36],
        _ => &UNKNOWN,
    }
}

pub fn hex(nib: u8) -> &'static [u8; 5] {
    &SPRITE[(nib as usize) & 0x0F]
}

pub fn string(str: &str) -> StringIter<'_> {
    StringIter::new(str)
}

pub fn hexstr(hex: &[u8]) -> HexIter<'_> {
    HexIter::new(hex)
}

pub struct HexIter<'a> {
    hex: &'a [u8],
    idx: usize,
    msb: bool,
}

impl<'a> HexIter<'a> {
    pub fn new(hex: &'a [u8]) -> Self {
        Self {
            hex,
            idx: 0,
            msb: true,
        }
    }
}

impl<'a> Iterator for HexIter<'a> {
    type Item = &'static [u8; 5];

    fn next(&mut self) -> Option<Self::Item> {
        self.hex.get(self.idx).map(|byte| {
            if self.msb {
                self.msb = false;
                hex((*byte & 0xF0) >> 4)
            } else {
                self.idx += 1;
                self.msb = true;
                hex(byte & 0x0F)
            }
        })
    }
}

pub struct StringIter<'a> {
    str: &'a [u8],
    idx: usize,
}

impl<'a> StringIter<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            str: str.as_bytes(),
            idx: 0,
        }
    }
}

impl<'a> Iterator for StringIter<'a> {
    type Item = &'static [u8; 5];

    fn next(&mut self) -> Option<Self::Item> {
        self.str.get(self.idx).map(|&ch| {
            self.idx += 1;
            char(ch)
        })
    }
}
