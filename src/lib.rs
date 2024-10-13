mod versions;
mod consts;

pub mod types;
pub mod errors;

/// Things that every version of protocol support
pub mod common {
    pub use crate::versions::common::*;
}

/// Documentation: <https://wiki.vg/Protocol> | 1.21.1 - 767
#[cfg(feature = "1_21_1")]
pub mod protocol {
    pub const PROTOCOL_VERSION: i32 = 767;
    pub const VERSION_NAME: &str = "1.21.1";
    pub use crate::versions::v1_21_1::*;
}