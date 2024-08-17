use macros::UIntWrapper;
use num_bigint::BigUint;

use crate::UIntType;

impl UIntType for PgU32 {
    type Uint = u32;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
pub struct PgU32 {
    inner: BigUint,
}
