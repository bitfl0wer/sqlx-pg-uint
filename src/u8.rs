use macros::UIntWrapper;
use num_bigint::BigUint;

use crate::UIntType;

impl UIntType for PgU8 {
    type Uint = u8;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
pub struct PgU8 {
    inner: BigUint,
}
