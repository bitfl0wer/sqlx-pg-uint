use sqlx_pg_uint_macros::UIntWrapper;
use num_bigint::BigUint;

use crate::UIntType;

impl UIntType for PgU128 {
    type Uint = u128;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
pub struct PgU128 {
    inner: BigUint,
}

#[cfg(test)]
mod pg_u128_tests {
    use super::*;

    #[test]
    fn test_to_u128() {
        let pg_u128 = PgU128 {
            inner: BigUint::from(12678671u128),
        };
        assert_eq!(pg_u128.to_uint(), 12678671u128);
        let pg_u128 = PgU128 {
            inner: BigUint::from(0u128),
        };
        assert_eq!(pg_u128.to_uint(), 0u128);
        let pg_u128 = PgU128 {
            inner: BigUint::from(u128::MAX),
        };
        assert_eq!(pg_u128.to_uint(), u128::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u128 = PgU128 {
            inner: BigUint::from(12678671u128),
        };
        let pg_u1282 = PgU128 {
            inner: BigUint::from(12678671u128),
        };
        assert_eq!((pg_u128 + pg_u1282).to_uint(), 25357342u128);
        let pg_u128 = PgU128 {
            inner: BigUint::from(0u128),
        };
        let pg_u1282 = PgU128 {
            inner: BigUint::from(0u128),
        };
        assert_eq!((pg_u128 + pg_u1282).to_uint(), 0u128);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let pg_u128 = PgU128 {
            inner: BigUint::from(u128::MAX),
        };
        let pg_u1282 = PgU128 {
            inner: BigUint::from(1u128),
        };
        let _ = pg_u128 + pg_u1282;
    }
}
