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

/// Helper trait to define the underlying integer type for a given `PgUint` type. Used in the
/// `sqlx-pg-uint-macros` crate to generate the necessary code for the `UIntWrapper` derive.
///
/// Not intended to be implemented by users, nor is it required to be used directly.
pub trait UIntType {
    type Uint;
}
