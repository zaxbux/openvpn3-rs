#![doc = include_str!("../README.md")]

mod error;
pub mod helpers;
mod proxy;

pub use self::error::*;
pub use self::proxy::*;

pub type Result<T> = std::result::Result<T, Error>;
