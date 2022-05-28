/// Read from GPIO pin(s).
///
/// # Panics
///
/// Calls `unwrap` on read results, and as such could panic if the read is
/// fallible. Integer arithmetic could theoretically panic, but not in any
/// reasonable use case.
///
/// # Examples
///
/// ```ignore
/// use rpio;
///
/// // Read one pin:
/// let val: bool = rpio::read!(pin);
///
/// // Read multiple pins:
/// let (a, b): (bool, bool) = rpio::read!(pin_a, pin_b);
///
/// // True if any pin matches:
/// let any_high: bool = rpio::read!(any true; pin_a, pin_b);
/// let any_low: bool = rpio::read!(any 0; pin_a, pin_b);
///
/// // True if all pins match:
/// let all_high: bool = rpio::read!(all 1; pin_a, pin_b);
/// let all_low: bool = rpio::read!(all false; pin_a, pin_b);
///
/// // Count matching pins (as u8):
/// let num_high: u8 = rpio::read!(count 1; pin_a, pin_b);
/// let num_low: u8 = rpio::read!(count 0; pin_a, pin_b);
///
/// // Interpret pins as an int:
/// let num: u8 = rpio::read!(u8; a, b, c, d); // 0b0000abcd
/// ```
#[macro_export]
macro_rules! read {
    ($pin: expr) => {
        $pin.is_high().ok().unwrap()
    };

    ($($pin: expr),+) => {
        (
            $(
                $pin.is_high().ok().unwrap()
            ),+
        )
    };

    (any $val: literal; $($pin: expr),+) => {
        $(
            $pin.is_high().ok().unwrap() == ($val as u8 != 0)
        ) || +
    };

    (all $val: literal; $($pin: expr),+) => {
        $(
            $pin.is_high().ok().unwrap() == ($val as u8 != 0)
        ) && +
    };

    (count $val: literal; $($pin: expr),+) => {
        $(
            (($pin.is_high().ok().unwrap() == ($val as u8 != 0)) as u8) +
        )+ 0
    };

    ($type: ty; $($pin: expr),+) => {{
        let num: $type = 0;
        $(
            let num = (num << 1) | ($pin.is_high().ok().unwrap() as $type);
        )+
        num
    }}
}

/// Write to GPIO pins.
///
/// # Panics
///
/// Calls `unwrap` on write results, and as such could panic if the write is
/// fallible. Integer arithmetic could theoretically panic, but not in
/// any reasonable use case.
///
/// # Examples
///
/// ```ignore
/// use rpio;
///
/// // Write one pin:
/// rpio::write!(pin => true);
///
/// // Write multiple pins:
/// rpio::write!(pin_a, pin_b => true);
///
/// // Interpret pins as some bits of an int:
/// rpio::write!(a, b, c => 3 bit => 0b101); // a=1, b=0, c=1
/// rpio::write!(a, b, c => 5 bit => 0b11100); // a=1, b=1, c=1
///
/// // Write multiple sets of pins:
/// rpio::write! {
///     pin_a => true;
///     pin_b, pin_c => 0;
///     pin_d => 1;
///     pin_e => false;
///     pf, pg, ph => 3 bit => 7;
/// };
/// ```
#[macro_export]
macro_rules! write {
    () => {};

    (
        $($pin: expr),+ => 1
        $(; $($tail: tt)*)?
    ) => {
        $($pin.set_high().ok().unwrap();)+
        $($crate::write!($($tail)*))?
    };

    (
        $($pin: expr),+ => true
        $(; $($tail: tt)*)?
    ) => {
        $($pin.set_high().ok().unwrap();)+
        $($crate::write!($($tail)*))?
    };

    (
        $($pin: expr),+ => 0
        $(; $($tail: tt)*)?
    ) => {
        $($pin.set_low().ok().unwrap();)+
        $($crate::write!($($tail)*))?
    };

    (
        $($pin: expr),+ => false
        $(; $($tail: tt)*)?
    ) => {
        $($pin.set_low().ok().unwrap();)+
        $($crate::write!($($tail)*))?
    };

    (
        $($pin: expr),+ => $bit: literal bit => $val: expr
        $(; $($tail: tt)*)?
    ) => {
        {
            let mask = (1 << ($bit - 1));
            let num = $val;

            $(
                if num & mask > 0 {
                    $pin.set_high().ok().unwrap();
                } else {
                    $pin.set_low().ok().unwrap();
                }

                let num = num << 1;
            )+
        }

        $($crate::write!($($tail)*))?
    };

    (
        $($pin: expr),+ => $val: expr
        $(; $($tail: tt)*)?
    ) => {
        {
            if $val as u8 > 0 {
                $($pin.set_high().ok().unwrap());+
            } else {
                $($pin.set_low().ok().unwrap());+
            }
        }

        $($crate::write!($($tail)*))?
    };
}
