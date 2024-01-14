pub mod block;
pub mod error;
pub mod main_key;
pub mod shift;

pub use block::Block;
pub use error::Error;
pub use main_key::MainKey;
pub use shift::{ShiftDirection, ShiftSchemes};
// FIXME: pub use std::str::FromStr;?

pub type Result<T> = std::result::Result<T, Error>;
