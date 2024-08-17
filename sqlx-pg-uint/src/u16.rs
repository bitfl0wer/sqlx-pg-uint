use sqlx_pg_uint_macros::UIntWrapper;
use bigdecimal::num_bigint::BigUint;

use crate::UIntType;

impl UIntType for PgU16 {
    type Uint = u16;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
pub struct PgU16 {
    inner: BigUint,
}
