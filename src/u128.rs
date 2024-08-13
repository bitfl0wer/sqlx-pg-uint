use std::ops::{Add, Mul, Sub};

use num_bigint::BigUint;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct PgU128 {
    inner: BigUint,
}

impl From<u128> for PgU128 {
    fn from(value: u128) -> Self {
        Self {
            inner: BigUint::from(value),
        }
    }
}

impl PgU128 {
    pub fn to_u128(&self) -> u128 {
        let bigint_bytes_be = self.inner.to_bytes_be();
        let mut bytes = [0u8; 16];
        for (i, byte) in bigint_bytes_be.iter().enumerate() {
            bytes[16 - bigint_bytes_be.len() + i] = *byte;
        }
        u128::from_be_bytes(bytes)
    }

    pub fn new(num: u128) -> Self {
        Self {
            inner: BigUint::from(num),
        }
    }
}

impl Add for PgU128 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.to_u128() + rhs.to_u128())
    }
}

impl Mul for PgU128 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.to_u128() * rhs.to_u128())
    }
}

impl Sub for PgU128 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.to_u128() - rhs.to_u128())
    }
}

impl std::fmt::Display for PgU128 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_u128())
    }
}

impl TryFrom<BigUint> for PgU128 {
    type Error = crate::Error;

    fn try_from(value: BigUint) -> Result<Self, Self::Error> {
        if value.bits() > 128 {
            return Err(crate::Error::Overflow);
        }
        Ok(Self { inner: value })
    }
}

#[cfg(test)]
mod pg_u128_tests {
    use super::*;

    #[test]
    fn test_to_u128() {
        let pg_u128 = PgU128 {
            inner: BigUint::from(12678671u128),
        };
        assert_eq!(pg_u128.to_u128(), 12678671u128);
        let pg_u128 = PgU128 {
            inner: BigUint::from(0u128),
        };
        assert_eq!(pg_u128.to_u128(), 0u128);
        let pg_u128 = PgU128 {
            inner: BigUint::from(u128::MAX),
        };
        assert_eq!(pg_u128.to_u128(), u128::MAX);
    }

    #[test]
    fn test_add() {
        let pg_u128 = PgU128 {
            inner: BigUint::from(12678671u128),
        };
        let pg_u1282 = PgU128 {
            inner: BigUint::from(12678671u128),
        };
        assert_eq!((pg_u128 + pg_u1282).to_u128(), 25357342u128);
        let pg_u128 = PgU128 {
            inner: BigUint::from(0u128),
        };
        let pg_u1282 = PgU128 {
            inner: BigUint::from(0u128),
        };
        assert_eq!((pg_u128 + pg_u1282).to_u128(), 0u128);
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
