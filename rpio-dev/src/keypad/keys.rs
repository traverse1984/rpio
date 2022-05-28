/// One or more keys pressed simultaneously.
#[derive(Debug, Clone, Copy)]
pub enum Keys {
    One(u8),
    Two(u8, u8),
    Three(u8, u8, u8),
    Four(u8, u8, u8, u8),
}

impl Keys {
    /// Convert the keys to an array of `Option<u8>`.
    pub fn as_array(&self) -> [Option<u8>; 4] {
        use Keys::*;

        match self {
            &One(k0) => [Some(k0), None, None, None],
            &Two(k0, k1) => [Some(k0), Some(k1), None, None],
            &Three(k0, k1, k2) => [Some(k0), Some(k1), Some(k2), None],
            &Four(k0, k1, k2, k3) => [Some(k0), Some(k1), Some(k2), Some(k3)],
        }
    }

    /// Determines whether a given key is among those pressed.
    pub fn includes(&self, key: u8) -> bool {
        use Keys::*;

        match self {
            &One(k0) => k0 == key,
            &Two(k0, k1) => k0 == key || k1 == key,
            &Three(k0, k1, k2) => k0 == key || k1 == key || k2 == key,
            &Four(k0, k1, k2, k3) => k0 == key || k1 == key || k2 == key || k3 == key,
        }
    }
}
