use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(UIntWrapper)]
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

        impl From<<#name as UIntType>::Uint> for #name {
            fn from(value: <#name as UIntType>::Uint) -> Self {
                Self {
                    inner: BigUint::from(value),
                }
            }
        }

        impl #name {
            pub fn to_uint(&self) -> <#name as UIntType>::Uint {
                let bigint_bytes_be = self.inner.to_bytes_be();
                let mut bytes = [0u8; std::mem::size_of::<<#name as UIntType>::Uint>()];
                for (i, byte) in bigint_bytes_be.iter().enumerate() {
                    bytes[std::mem::size_of::<<#name as UIntType>::Uint>() - bigint_bytes_be.len() + i] = *byte;
                }
                <#name as UIntType>::Uint::from_be_bytes(bytes)
            }

            pub fn new(num: <#name as UIntType>::Uint) -> Self {
                Self {
                    inner: BigUint::from(num),
                }
            }

            pub fn as_biguint(&self) -> &BigUint {
                &self.inner
            }
        }


        impl From<#name> for BigUint {
            fn from(value: #name) -> Self {
                value.inner
            }
        }
    };

    gen.into()
}
