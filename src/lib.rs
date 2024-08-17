pub mod u128;
pub mod u16;
pub mod u32;
pub mod u64;
pub mod u8;

use thiserror::Error;
pub use u128::*;
pub use u16::*;
pub use u32::*;
pub use u64::*;
pub use u8::*;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord, Copy, Error)]
pub enum Error {
    #[error("Value is too large to be made into target type")]
    Overflow,
}

pub trait UIntType {
    type Uint;
}
