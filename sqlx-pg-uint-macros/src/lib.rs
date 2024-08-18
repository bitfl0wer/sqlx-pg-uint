use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UIntWrapper)]
/// Derive macro for unsigned integer types.
///
/// Derives all the mathematical operations for the unsigned integer type, as well as `Display`,
/// `From` and `TryFrom` implementations for/to `BigDecimal`, a `to_uint` method to convert the
/// `PgUint` type to the underlying integer type and a `new` method to create a new `PgUint` type
/// from the underlying integer type.
pub fn uint_wrapper_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let gen = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.to_uint())
            }
        }

        impl std::ops::Add for #name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.to_uint() + rhs.to_uint())
            }
        }

        impl std::ops::Mul for #name {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.to_uint() * rhs.to_uint())
            }
        }

        impl std::ops::Sub for #name {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::new(self.to_uint() - rhs.to_uint())
            }
        }

        impl std::ops::Div for #name {
            type Output = Self;

            fn div(self, rhs: Self) -> Self::Output {
                Self::new(self.to_uint() / rhs.to_uint())
            }
        }

        impl std::ops::Rem for #name {
            type Output = Self;

            fn rem(self, rhs: Self) -> Self::Output {
                Self::new(self.to_uint() % rhs.to_uint())
            }
        }

        impl From<<#name as UIntType>::Uint> for #name {
            fn from(value: <#name as UIntType>::Uint) -> Self {
                Self {
                    inner: BigDecimal::from(value),
                }
            }
        }

        impl From<#name> for <#name as UIntType>::Uint {
            fn from(value: #name) -> Self {
                value.to_uint()
            }
        }

        impl #name {
            /// Converts this type to the associated unsigned integer type
            pub fn to_uint(&self) -> <#name as UIntType>::Uint {
                let stringed_num = self.inner.to_string();
                stringed_num.parse().unwrap()
            }

            /// Creates a new instance of this type from the associated unsigned integer type
            pub fn new(num: <#name as UIntType>::Uint) -> Self {
                Self {
                    inner: BigDecimal::from(num),
                }
            }

            /// Returns a shared reference to the inner `BigDecimal` value
            pub fn as_big_decimal(&self) -> &BigDecimal {
                &self.inner
            }
        }

        impl TryFrom<BigDecimal> for #name {
            type Error = crate::Error;

            fn try_from(value: BigDecimal) -> Result<Self, Self::Error> {
                let value_ref = &value;
                if !value_ref.is_integer() {
                    return Err(crate::Error::Fractional(value));
                }
                if value_ref.to_string().parse::<u128>().is_err() {
                    return Err(crate::Error::InvalidValue(value));
                }
                Ok(Self { inner: value })
            }
        }

        impl From<#name> for BigDecimal {
            fn from(value: #name) -> Self {
                value.inner
            }
        }

        impl sqlx::Type<sqlx::Postgres> for #name {
            fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
                <BigDecimal as sqlx::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl<'q> sqlx::Encode<'q, sqlx::Postgres> for #name {
            fn encode_by_ref(
                &self,
                buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
                self.inner.encode_by_ref(buf)
            }
        }

        impl<'r> sqlx::Decode<'r, sqlx::Postgres> for #name {
            fn decode(
                value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let big_decimal = BigDecimal::decode(value)?;
                Ok(#name::try_from(big_decimal)?)
            }
        }

        #[cfg(feature = "serde")]
        impl serde::ser::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.inner.serialize(serializer)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> serde::de::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let big_decimal = BigDecimal::deserialize(deserializer)?;
                #name::try_from(big_decimal).map_err(serde::de::Error::custom)
            }
        }
    };

    gen.into()
}
