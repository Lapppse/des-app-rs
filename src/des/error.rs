use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid round, expected (1 <= round <= 16), got {0}")]
    InvalidRound(u8),
    #[error("couldn't convert string {0} to main key (is it hex with length 16?)")]
    StringParseError(String),
    #[error("expected bitvec to be 64 bits long, but provided bitvec was of length {0}")]
    InvalidKeyLength(usize),
}
