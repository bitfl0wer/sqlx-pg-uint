pub mod u128;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;

pub(crate) use bigdecimal::BigDecimal;
use thiserror::Error;
pub use u128::*;
pub use u16::*;
pub use u32::*;
pub use u64::*;
pub use u8::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Error)]
pub enum Error {
    #[error("Value is either too large, to small or not an integer")]
    InvalidValue(BigDecimal),
    #[error("Invalid value for target type")]
    Fractional(BigDecimal),
}

pub trait UIntType {
    type Uint;
}
