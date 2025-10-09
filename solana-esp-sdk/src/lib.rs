#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
extern crate alloc;

pub mod types;

pub mod hash;
pub mod signature;

pub mod instruction;

pub mod crypto;

pub mod transaction;

pub mod rpc;

#[cfg(any(feature = "net-smoltcp", feature = "net-reqwless"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "net-smoltcp", feature = "net-reqwless")))
)]
pub mod net;

pub mod prelude {

    pub use crate::types::{Result, SdkError};

    pub use crate::crypto::*;

    pub use crate::instruction::*;

    pub use crate::transaction::*;

    pub use crate::rpc::*;
}
