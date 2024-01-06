pub mod error;
pub mod main_key;
pub mod shift;

pub use error::Error;
pub use main_key::MainKey;
pub use shift::ShiftSchemes;

pub type Result<T> = std::result::Result<T, Error>;
