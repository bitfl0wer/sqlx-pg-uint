use macros::UIntWrapper;
use num_bigint::BigUint;

use crate::UIntType;

impl UIntType for PgU64 {
    type Uint = u64;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
pub struct PgU64 {
    inner: BigUint,
}
