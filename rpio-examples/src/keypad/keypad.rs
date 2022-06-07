use super::seq::KeySeqIter;
use super::Keys;

pub trait Keypad {
    /// Returns true if any key is pressed, without trying to read which key(s).
    ///
    /// # Examples
    ///
    /// ```ignore
    /// println!("Press any key...");
    ///
    /// loop {
    ///     if keypad.key_is_pressed() {
    ///         println!("Thanks!");
    ///         break;
    ///     }
    /// }
    /// ```
    fn key_is_pressed(&self) -> bool;

    /// Read a single key press from the keypad. The first key identified is
    /// returned as [Some]. If no key is pressed, [None] is returned.
    ///
    /// # Examples
    ///
    /// ```ignore
    ///
    /// match keypad.read() {
    ///     Some(key) => println!("Got key: {}.", key),
    ///     None => println!("No key pressed.");
    /// }
    /// ```
    fn read(&mut self) -> Option<u8>;
    fn read_delayed(&mut self, delay: &mut cortex_m::delay::Delay) -> Option<u8>;

    /// Read multiple key presses from the keypad. Up to four keys can be
    /// identified at once, but it is not possible to detect two keys from
    /// the same row or column. The identified [Keys] are returned as [Some].
    /// If no keys are pressed, [None] is returned.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let val = match keypad.read_multi() {
    ///     Some(Keys::One(key)) => println!("Got key: {}.", key),
    ///     Some(Keys::Two(key, ctrl)) if ctrl == 0xC => {
    ///         println!("Got ctrl key: {}.", key);
    ///     }
    ///     Some(_) => println!("Invalid key combination."),
    ///     None => println!("No key pressed."),
    /// }
    ///
    /// ```
    fn read_multi(&mut self) -> Option<Keys>;

    fn read_keyup(&mut self) -> Option<u8> {
        self.read().map(|key| {
            while self.key_is_pressed() {}
            key
        })
    }

    fn read_seq<'a, 'b>(&'a mut self, buf: &'b mut [u8]) -> KeySeqIter<'a, 'b, Self>
    where
        Self: Sized;
}
