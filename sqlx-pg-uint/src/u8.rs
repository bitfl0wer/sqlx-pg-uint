use super::*;
use sqlx_pg_uint_macros::UIntWrapper;

use crate::UIntType;

impl UIntType for PgU8 {
    type Uint = u8;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper)]
/// PostgreSQL-compatible unsigned 8-bit integer
pub struct PgU8 {
    inner: BigDecimal,
}

#[cfg(test)]
mod pg_u8_tests {
    use bigdecimal::num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_to_u8() {
        let pg_u8 = PgU8::from(121u8);
        assert_eq!(pg_u8.to_uint(), 121u8);
        let pg_u8 = PgU8::from(0);
        assert_eq!(pg_u8.to_uint(), 0u8);
        let pg_u8 = PgU8::from(u8::MAX);
        assert_eq!(pg_u8.to_uint(), u8::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u8 = PgU8::from(12u8);
        let pg_u82 = PgU8::from(12u8);
        assert_eq!((pg_u8 + pg_u82).to_uint(), 24u8);

        let pg_u8 = PgU8::from(0u8);
        let pg_u82 = PgU8::from(0u8);
        assert_eq!((pg_u8 + pg_u82).to_uint(), 0u8);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let pg_u8 = PgU8::from(u8::MAX);
        let pg_u82 = PgU8::from(1u8);
        let _ = pg_u8 + pg_u82;
    }

    #[test]
    #[should_panic]
    fn test_add_underflow() {
        let pg_u8 = PgU8::from(0u8);
        let pg_u82 = PgU8::from(1u8);
        let _ = pg_u8 - pg_u82;
    }

    #[test]
    fn try_from_bigdecimal() {
        let pg_u8 = PgU8::try_from(BigDecimal::from(126u8)).unwrap();
        assert_eq!(pg_u8.to_uint(), 126u8);

        let pg_u8 = PgU8::try_from(BigDecimal::from(0)).unwrap();
        assert_eq!(pg_u8.to_uint(), 0u8);

        let pg_u8 = PgU8::try_from(BigDecimal::from(u8::MAX)).unwrap();
        assert_eq!(pg_u8.to_uint(), u8::MAX);

        let pg_u8 = PgU8::try_from(BigDecimal::from(-1));
        assert!(pg_u8.is_err());
        let err = pg_u8.unwrap_err();
        assert_eq!(err, Error::InvalidValue(BigDecimal::from(-1)));

        let fractional = BigDecimal::from(3) / BigDecimal::from(2);
        let pg_u8 = PgU8::try_from(fractional.clone());
        assert_eq!(pg_u8.unwrap_err(), Error::Fractional(fractional));

        let big_decimal = BigDecimal::from(BigInt::from(2).pow(128));
        let pg_u8 = PgU8::try_from(big_decimal.clone());
        assert_eq!(pg_u8.unwrap_err(), Error::InvalidValue(big_decimal));
    }
}
