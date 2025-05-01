use super::*;
use sqlx_pg_uint_macros::UIntWrapper;

use crate::UIntType;

impl private::Sealed for PgU128 {}

impl UIntType for PgU128 {
    type Uint = u128;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, UIntWrapper, sqlx::FromRow)]
/// PostgreSQL-compatible unsigned 128-bit integer
pub struct PgU128 {
    inner: BigDecimal,
}

impl From<PgU128> for u128 {
    fn from(value: PgU128) -> Self {
        value.inner.to_string().parse().unwrap()
    }
}

impl From<u128> for PgU128 {
    fn from(value: u128) -> Self {
        Self {
            inner: BigDecimal::from(value),
        }
    }
}

#[cfg(test)]
mod pg_u128_tests {
    use bigdecimal::num_bigint::BigInt;

    use super::*;

    #[test]
    fn test_to_u128() {
        let pg_u128 = PgU128::from(12678671u128);
        assert_eq!(pg_u128.to_uint(), 12678671u128);
        let pg_u128 = PgU128::from(0);
        assert_eq!(pg_u128.to_uint(), 0u128);
        let pg_u128 = PgU128::from(u128::MAX);
        assert_eq!(pg_u128.to_uint(), u128::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u128 = PgU128::from(12678671u128);
        let pg_u1282 = PgU128::from(12678671u128);
        assert_eq!((pg_u128 + pg_u1282).to_uint(), 25357342u128);

        let pg_u128 = PgU128::from(0u128);
        let pg_u1282 = PgU128::from(0u128);
        assert_eq!((pg_u128 + pg_u1282).to_uint(), 0u128);
    }

    #[test]
    #[should_panic]
    fn test_add_overflow() {
        let pg_u128 = PgU128::from(u128::MAX);
        let pg_u1282 = PgU128::from(1u128);
        let _ = pg_u128 + pg_u1282;
    }

    #[test]
    #[should_panic]
    fn test_add_underflow() {
        let pg_u128 = PgU128::from(0u128);
        let pg_u1282 = PgU128::from(1u128);
        let _ = pg_u128 - pg_u1282;
    }

    #[test]
    fn try_from_bigdecimal() {
        let pg_u128 = PgU128::try_from(BigDecimal::from(12678671u128)).unwrap();
        assert_eq!(pg_u128.to_uint(), 12678671u128);

        let pg_u128 = PgU128::try_from(BigDecimal::from(0)).unwrap();
        assert_eq!(pg_u128.to_uint(), 0u128);

        let pg_u128 = PgU128::try_from(BigDecimal::from(u128::MAX)).unwrap();
        assert_eq!(pg_u128.to_uint(), u128::MAX);

        let pg_u128 = PgU128::try_from(BigDecimal::from(-1));
        assert!(pg_u128.is_err());
        let err = pg_u128.unwrap_err();
        assert_eq!(err, Error::InvalidValue(BigDecimal::from(-1)));

        let fractional = BigDecimal::from(3) / BigDecimal::from(2);
        let pg_u128 = PgU128::try_from(fractional.clone());
        assert_eq!(pg_u128.unwrap_err(), Error::Fractional(fractional));

        let big_decimal = BigDecimal::from(BigInt::from(2).pow(128));
        let pg_u128 = PgU128::try_from(big_decimal.clone());
        assert_eq!(pg_u128.unwrap_err(), Error::InvalidValue(big_decimal));
    }

    #[test]
    fn test_option_conversion() {
        let somepguint = Some(PgU128::from(123u128));
        let someuint = somepguint.to_option_uint();
        assert_eq!(someuint, Some(123u128));

        let pguint = PgU128::from(123);
        let someuint = pguint.to_option_uint();
        assert_eq!(someuint, Some(123u128));

        let pguint: Option<PgU128> = None;
        let someuint = pguint.to_option_uint();
        assert_eq!(someuint, None::<u128>);
    }
}
