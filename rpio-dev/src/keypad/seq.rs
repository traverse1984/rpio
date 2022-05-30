use super::Keypad;

pub struct KeySeqIter<'a, 'b, K: Keypad + 'a> {
    keypad: &'a mut K,
    buf: &'b mut [u8],
    idx: usize,
}

impl<'a, 'b, K: Keypad + 'a> KeySeqIter<'a, 'b, K> {
    pub fn new(keypad: &'a mut K, buf: &'b mut [u8]) -> Self {
        Self {
            keypad,
            buf,
            idx: 0,
        }
    }
}

impl<'a, 'b, K: Keypad + 'a> Iterator for KeySeqIter<'a, 'b, K> {
    type Item = Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.get_mut(self.idx).map(|k| {
            self.keypad.read_keyup().map(|key| {
                *k = key;
                self.idx += 1;
                key
            })
        })
    }
}
