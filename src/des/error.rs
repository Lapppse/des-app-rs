use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("invalid round, expected (1 <= round <= 16), got {0}")]
    InvalidRound(u8),
    #[error("couldn't convert string {0} to bitvec (is it hex?)")]
    StringParseError(String),
    #[error(
        "expected iterable to be at least/exactly {expected} bits long, but provided iterable was of length {got}"
    )]
    InvalidIterableLength { expected: usize, got: usize },
}
