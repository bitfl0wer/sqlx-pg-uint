use super::*;
use sqlx_pg_uint_macros::UIntWrapper;

use crate::UIntType;

impl UIntType for PgU32 {
    type Uint = u32;
}

impl private::Sealed for PgU32 {}
#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper, sqlx::FromRow)]
/// PostgreSQL-compatible unsigned 32-bit integer
pub struct PgU32 {
    inner: BigDecimal,
}

impl From<PgU32> for u32 {
    fn from(value: PgU32) -> Self {
        value.inner.to_string().parse().unwrap()
    }
}

impl From<u32> for PgU32 {
    fn from(value: u32) -> Self {
        Self {
            inner: BigDecimal::from(value),
        }
    }
}

#[cfg(test)]
mod pg_u32_tests {
    use bigdecimal::num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_to_u32() {
        let pg_u32 = PgU32::from(121u32);
        assert_eq!(pg_u32.to_uint(), 121u32);
        let pg_u32 = PgU32::from(0);
        assert_eq!(pg_u32.to_uint(), 0u32);
        let pg_u32 = PgU32::from(u32::MAX);
        assert_eq!(pg_u32.to_uint(), u32::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u32 = PgU32::from(12u32);
        let pg_u322 = PgU32::from(12u32);
        assert_eq!((pg_u32 + pg_u322).to_uint(), 24u32);

        let pg_u32 = PgU32::from(0u32);
        let pg_u322 = PgU32::from(0u32);
        assert_eq!((pg_u32 + pg_u322).to_uint(), 0u32);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let pg_u32 = PgU32::from(u32::MAX);
        let pg_u322 = PgU32::from(1u32);
        let _ = pg_u32 + pg_u322;
    }

    #[test]
    #[should_panic]
    fn test_add_underflow() {
        let pg_u32 = PgU32::from(0u32);
        let pg_u322 = PgU32::from(1u32);
        let _ = pg_u32 - pg_u322;
    }

    #[test]
    fn try_from_bigdecimal() {
        let pg_u32 = PgU32::try_from(BigDecimal::from(126u32)).unwrap();
        assert_eq!(pg_u32.to_uint(), 126u32);

        let pg_u32 = PgU32::try_from(BigDecimal::from(0)).unwrap();
        assert_eq!(pg_u32.to_uint(), 0u32);

        let pg_u32 = PgU32::try_from(BigDecimal::from(u32::MAX)).unwrap();
        assert_eq!(pg_u32.to_uint(), u32::MAX);

        let pg_u32 = PgU32::try_from(BigDecimal::from(-1));
        assert!(pg_u32.is_err());
        let err = pg_u32.unwrap_err();
        assert_eq!(err, Error::InvalidValue(BigDecimal::from(-1)));

        let fractional = BigDecimal::from(3) / BigDecimal::from(2);
        let pg_u32 = PgU32::try_from(fractional.clone());
        assert_eq!(pg_u32.unwrap_err(), Error::Fractional(fractional));

        let big_decimal = BigDecimal::from(BigInt::from(2).pow(128));
        let pg_u32 = PgU32::try_from(big_decimal.clone());
        assert_eq!(pg_u32.unwrap_err(), Error::InvalidValue(big_decimal));
    }

    #[test]
    fn test_option_conversion() {
        let somepguint = Some(PgU32::from(123u32));
        let someuint = somepguint.to_option_uint();
        assert_eq!(someuint, Some(123u32));

        let pguint = PgU32::from(123);
        let someuint = pguint.to_option_uint();
        assert_eq!(someuint, Some(123u32));

        let pguint: Option<PgU32> = None;
        let someuint = pguint.to_option_uint();
        assert_eq!(someuint, None::<u32>);
    }
}
