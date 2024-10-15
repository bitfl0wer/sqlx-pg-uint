use super::*;
use sqlx_pg_uint_macros::UIntWrapper;

use crate::UIntType;

impl UIntType for PgU16 {
    type Uint = u16;
}

impl private::Sealed for PgU16 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper, sqlx::FromRow)]
/// PostgreSQL-compatible unsigned 16-bit integer
pub struct PgU16 {
    inner: BigDecimal,
}

impl From<PgU16> for u16 {
    fn from(value: PgU16) -> Self {
        value.inner.to_string().parse().unwrap()
    }
}

impl From<u16> for PgU16 {
    fn from(value: u16) -> Self {
        Self {
            inner: BigDecimal::from(value),
        }
    }
}

#[cfg(test)]
mod pg_u16_tests {
    use bigdecimal::num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_to_u16() {
        let pg_u16 = PgU16::from(121u16);
        assert_eq!(pg_u16.to_uint(), 121u16);
        let pg_u16 = PgU16::from(0);
        assert_eq!(pg_u16.to_uint(), 0u16);
        let pg_u16 = PgU16::from(u16::MAX);
        assert_eq!(pg_u16.to_uint(), u16::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u16 = PgU16::from(12u16);
        let pg_u162 = PgU16::from(12u16);
        assert_eq!((pg_u16 + pg_u162).to_uint(), 24u16);

        let pg_u16 = PgU16::from(0u16);
        let pg_u162 = PgU16::from(0u16);
        assert_eq!((pg_u16 + pg_u162).to_uint(), 0u16);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let pg_u16 = PgU16::from(u16::MAX);
        let pg_u162 = PgU16::from(1u16);
        let _ = pg_u16 + pg_u162;
    }

    #[test]
    #[should_panic]
    fn test_add_underflow() {
        let pg_u16 = PgU16::from(0u16);
        let pg_u162 = PgU16::from(1u16);
        let _ = pg_u16 - pg_u162;
    }

    #[test]
    fn try_from_bigdecimal() {
        let pg_u16 = PgU16::try_from(BigDecimal::from(126u16)).unwrap();
        assert_eq!(pg_u16.to_uint(), 126u16);

        let pg_u16 = PgU16::try_from(BigDecimal::from(0)).unwrap();
        assert_eq!(pg_u16.to_uint(), 0u16);

        let pg_u16 = PgU16::try_from(BigDecimal::from(u16::MAX)).unwrap();
        assert_eq!(pg_u16.to_uint(), u16::MAX);

        let pg_u16 = PgU16::try_from(BigDecimal::from(-1));
        assert!(pg_u16.is_err());
        let err = pg_u16.unwrap_err();
        assert_eq!(err, Error::InvalidValue(BigDecimal::from(-1)));

        let fractional = BigDecimal::from(3) / BigDecimal::from(2);
        let pg_u16 = PgU16::try_from(fractional.clone());
        assert_eq!(pg_u16.unwrap_err(), Error::Fractional(fractional));

        let big_decimal = BigDecimal::from(BigInt::from(2).pow(128));
        let pg_u16 = PgU16::try_from(big_decimal.clone());
        assert_eq!(pg_u16.unwrap_err(), Error::InvalidValue(big_decimal));
    }
}
