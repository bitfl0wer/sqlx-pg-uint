use super::*;
use sqlx_pg_uint_macros::UIntWrapper;

use crate::UIntType;

impl UIntType for PgU8 {
    type Uint = u8;
}

impl private::Sealed for PgU8 {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper, sqlx::FromRow)]
/// PostgreSQL-compatible unsigned 8-bit integer
pub struct PgU8 {
    inner: BigDecimal,
}

impl From<PgU8> for u8 {
    fn from(value: PgU8) -> Self {
        value.inner.to_string().parse().unwrap()
    }
}

impl From<u8> for PgU8 {
    fn from(value: u8) -> Self {
        Self {
            inner: BigDecimal::from(value),
        }
    }
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

    #[test]
    fn test_option_conversion() {
        let somepguint = Some(PgU8::from(123u8));
        let someuint = somepguint.to_option_uint();
        assert_eq!(someuint, Some(123u8));

        let pguint = PgU8::from(123);
        let someuint = pguint.to_option_uint();
        assert_eq!(someuint, Some(123u8));

        let pguint: Option<PgU8> = None;
        let someuint = pguint.to_option_uint();
        assert_eq!(someuint, None::<u8>);
    }
}
