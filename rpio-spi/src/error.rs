/// Indicates an SPI error. The [`Transport`](super::traits::Transport)
/// in use determines which errors are possible.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    Transfer,
    ChipSelect,
    ChipDeselect,
    ClockSpeed,
    NotImplemented,
}

/// Result where the Err is an SPI [`Error`].
pub type Result<T = ()> = core::result::Result<T, Error>;

#[cfg(feature = "std")]
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Transfer => "SPI transfer error",
                Error::ChipSelect => "Select SPI chip error",
                Error::ChipDeselect => "Deselect SPI chip error",
                Error::ClockSpeed => "Set SPI clock speed error",
                Error::NotImplemented => "That feature is not implemented",
            }
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
