mod versions;
mod consts;

pub mod types;
pub mod errors;

pub mod common {
    pub use crate::versions::common::*;
}

#[cfg(feature = "1_21_1")]
pub mod protocol {
    pub use crate::versions::v1_21_1::*;
}