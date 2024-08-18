#![warn(
    missing_docs,
    missing_debug_implementations,
    rust_2018_idioms,
    missing_copy_implementations
)]
#![deny(unsafe_code)]

/*!
# sqlx-pg-uint

`SQLx` extension to support working with Rust unsigned integers in PostgreSQL.

---

This crate provides types with `sqlx::{Encode, Decode, Type}` implemented for them, which allow you
to work with fixed-size unsigned integers in PostgreSQL.
*/

mod u128;
mod u16;
mod u32;
mod u64;
mod u8;

pub(crate) use bigdecimal::BigDecimal;
use thiserror::Error;
pub use u128::*;
pub use u16::*;
pub use u32::*;
pub use u64::*;
pub use u8::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Error)]
/// Error type for conversions between `BigDecimal` and `PgUint` types.
pub enum Error {
    #[error("Value is either too large, to small or not an integer")]
    /// Error when the value is either too large, too small or not an integer.
    InvalidValue(BigDecimal),
    #[error("Invalid value for target type")]
    /// Provided value is a floating point number, which is not supported by the target type.
    Fractional(BigDecimal),
}

mod private {
    pub trait Sealed {}
}

/// Helper trait to define the underlying integer type for a given `PgUint` type. Used in the
/// `sqlx-pg-uint-macros` crate to generate the necessary code for the `UIntWrapper` derive.
///
/// Not intended to be implemented by users, nor is it required to be used directly.
pub trait UIntType: private::Sealed {
    /// The underlying integer type for the `PgUint` type.
    type Uint;
}
