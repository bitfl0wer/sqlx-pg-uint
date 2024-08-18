use super::*;
use sqlx_pg_uint_macros::UIntWrapper;

use crate::UIntType;

impl UIntType for PgU64 {
    type Uint = u64;
}
impl private::Sealed for PgU64 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
/// PostgreSQL-compatible unsigned 64-bit integer
pub struct PgU64 {
    inner: BigDecimal,
}

#[cfg(test)]
mod pg_u64_tests {
    use bigdecimal::num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_to_u64() {
        let pg_u64 = PgU64::from(121u64);
        assert_eq!(pg_u64.to_uint(), 121u64);
        let pg_u64 = PgU64::from(0);
        assert_eq!(pg_u64.to_uint(), 0u64);
        let pg_u64 = PgU64::from(u64::MAX);
        assert_eq!(pg_u64.to_uint(), u64::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u64 = PgU64::from(12u64);
        let pg_u642 = PgU64::from(12u64);
        assert_eq!((pg_u64 + pg_u642).to_uint(), 24u64);

        let pg_u64 = PgU64::from(0u64);
        let pg_u642 = PgU64::from(0u64);
        assert_eq!((pg_u64 + pg_u642).to_uint(), 0u64);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let pg_u64 = PgU64::from(u64::MAX);
        let pg_u642 = PgU64::from(1u64);
        let _ = pg_u64 + pg_u642;
    }

    #[test]
    #[should_panic]
    fn test_add_underflow() {
        let pg_u64 = PgU64::from(0u64);
        let pg_u642 = PgU64::from(1u64);
        let _ = pg_u64 - pg_u642;
    }

    #[test]
    fn try_from_bigdecimal() {
        let pg_u64 = PgU64::try_from(BigDecimal::from(126u64)).unwrap();
        assert_eq!(pg_u64.to_uint(), 126u64);

        let pg_u64 = PgU64::try_from(BigDecimal::from(0)).unwrap();
        assert_eq!(pg_u64.to_uint(), 0u64);

        let pg_u64 = PgU64::try_from(BigDecimal::from(u64::MAX)).unwrap();
        assert_eq!(pg_u64.to_uint(), u64::MAX);

        let pg_u64 = PgU64::try_from(BigDecimal::from(-1));
        assert!(pg_u64.is_err());
        let err = pg_u64.unwrap_err();
        assert_eq!(err, Error::InvalidValue(BigDecimal::from(-1)));

        let fractional = BigDecimal::from(3) / BigDecimal::from(2);
        let pg_u64 = PgU64::try_from(fractional.clone());
        assert_eq!(pg_u64.unwrap_err(), Error::Fractional(fractional));

        let big_decimal = BigDecimal::from(BigInt::from(2).pow(128));
        let pg_u64 = PgU64::try_from(big_decimal.clone());
        assert_eq!(pg_u64.unwrap_err(), Error::InvalidValue(big_decimal));
    }
}
