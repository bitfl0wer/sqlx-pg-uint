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

```
use sqlx_pg_uint::PgU64;

let a_u64_number = 2937854645u64;
let pg_u_64 = PgU64::from(a_u64_number);
println!("PgU64: {}", pg_u_64);
let back_to_u64: u64 = pg_u_64.to_uint();
println!("Back to u64: {}", back_to_u64);
println!(
    "Maths work the same way as you'd expect: {}",
    PgU64::from(67) + PgU64::from(2) * PgU64::from(3) / PgU64::from(3)
);
println!(
    "Interact with the underlying BigDecimal type directly: {}",
    pg_u_64.as_big_decimal()
);
println!("PgUint types can be converted to and from BigDecimals, and are storable in an sqlx::Postgres database.");
println!("If you load a PgUint from a database successfully, you can be sure that it's a valid fixed-size unsigned integer.");
```
*/

mod u128;
mod u16;
mod u32;
mod u64;
mod u8;

use std::fmt::Display;
use std::str::FromStr;

pub(crate) use bigdecimal::BigDecimal;
use thiserror::Error;
pub use u128::*;
pub use u16::*;
pub use u32::*;
pub use u64::*;
pub use u8::*;

#[derive(Debug, PartialEq, Clone, Error)]
/// Error type for conversions between `BigDecimal` and `PgUint` types.
pub enum Error {
    #[error("Value is either too large, to small or not an integer")]
    /// Error when the value is either too large, too small or not an integer.
    InvalidValue(BigDecimal),
    #[error("Invalid value for target type")]
    /// Provided value is a floating point number, which is not supported by the target type.
    Fractional(BigDecimal),
    #[cfg(feature = "serde")]
    #[error(transparent)]
    /// Error when deserializing a `BigDecimal` from a `serde` deserializer.
    Serde(#[from] serde::de::value::Error),
    #[error(transparent)]
    /// Error when parsing a `BigDecimal` from a string.
    ParseInt(#[from] std::num::ParseIntError),
}

mod private {
    pub trait Sealed {}
}

impl private::Sealed for u8 {}
impl private::Sealed for u16 {}
impl private::Sealed for u32 {}
impl private::Sealed for u64 {}
impl private::Sealed for u128 {}

/// Helper trait to define the underlying integer type for a given `PgUint` type. Used in the
/// `sqlx-pg-uint-macros` crate to generate the necessary code for the `UIntWrapper` derive.
///
/// Not intended to be implemented by users, nor is it required to be used directly.
pub trait UIntType: private::Sealed + Display {
    /// The underlying integer type for the `PgUint` type.
    type Uint: private::Sealed + FromStr;
}

/// Allows for converting an `Option<PgUInt>` to an `Option<[underlying integer type]>`
pub trait OptionPgUint<T: UIntType> {
    /// Convert any `Option<PgUint>` to an `Option<[underlying integer type]>`
    fn to_option_uint(&self) -> Option<T::Uint>;
}
