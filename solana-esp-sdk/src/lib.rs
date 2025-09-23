#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

// ─────────────────────────────────────────────────────────────
// Feature sanity checks (keep dependency graph explicit)
// ─────────────────────────────────────────────────────────────

#[cfg(all(feature = "spl-token", not(feature = "instr")))]
compile_error!("feature `spl-token` requires `instr` to be enabled.");

#[cfg(all(feature = "tx", not(feature = "instr")))]
compile_error!("feature `tx` requires `instr` (instruction builders) to be enabled.");

// ─────────────────────────────────────────────────────────────
// Core (always-on, no_std-friendly)
// ─────────────────────────────────────────────────────────────

pub mod types;

pub mod hash;

// Optional lightweight codecs (uleb128, base64, optional base58/json)
#[cfg(feature = "codecs")]
#[cfg_attr(docsrs, doc(cfg(feature = "codecs")))]
pub mod codecs;

// ─────────────────────────────────────────────────────────────
// Instruction builders (opt-in)
// ─────────────────────────────────────────────────────────────

#[cfg(feature = "instr")]
#[cfg_attr(docsrs, doc(cfg(feature = "instr")))]
pub mod instruction;

#[cfg(feature = "spl-token")]
#[cfg_attr(docsrs, doc(cfg(feature = "spl-token")))]
pub mod spl;

// ─────────────────────────────────────────────────────────────
// Crypto (keys + signing) (opt-in)
// ─────────────────────────────────────────────────────────────

#[cfg(feature = "crypto")]
#[cfg_attr(docsrs, doc(cfg(feature = "crypto")))]
pub mod crypto;

// ─────────────────────────────────────────────────────────────
// Message/Transaction assembly (opt-in)
// ─────────────────────────────────────────────────────────────

#[cfg(feature = "tx")]
#[cfg_attr(docsrs, doc(cfg(feature = "tx")))]
pub mod message;

#[cfg(feature = "tx")]
#[cfg_attr(docsrs, doc(cfg(feature = "tx")))]
pub mod transaction;

// ─────────────────────────────────────────────────────────────
/* Transport-agnostic RPC (opt-in)
   - Provides RpcClient trait + tiny JSON-RPC helpers.
   - No sockets/TLS here; see `net` adapters.
*/
// ─────────────────────────────────────────────────────────────

#[cfg_attr(docsrs, doc(cfg(feature = "rpc")))]
pub mod rpc;

// ─────────────────────────────────────────────────────────────
/* Network adapters (opt-in)
   - Compile only when an adapter feature is set.
   - Examples: `net-smoltcp`, `net-reqwless`, or a std-host shim.
*/
// ─────────────────────────────────────────────────────────────

#[cfg(any(feature = "net-smoltcp", feature = "net-reqwless", feature = "net-std"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "net-smoltcp", feature = "net-reqwless", feature = "net-std")))
)]
pub mod net;

// ─────────────────────────────────────────────────────────────
// Minimal prelude (ergonomic imports without pulling extras)
// ─────────────────────────────────────────────────────────────

/// Curated re-exports for common use-cases. Import with:
/// `use solana_esp_sdk::prelude::*;`
pub mod prelude {
    // Always-available core error/result
    pub use crate::types::{Result, SdkError};

    // Crypto (keys + signing)
    #[cfg(feature = "crypto")]
    #[cfg_attr(docsrs, doc(cfg(feature = "crypto")))]
    pub use crate::crypto::*;

    // Instruction builders
    #[cfg(feature = "instr")]
    #[cfg_attr(docsrs, doc(cfg(feature = "instr")))]
    pub use crate::instruction::*;

    // SPL-Token helpers
    #[cfg(feature = "spl-token")]
    #[cfg_attr(docsrs, doc(cfg(feature = "spl-token")))]
    pub use crate::spl::*;

    // Message/Transaction
    #[cfg(feature = "tx")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tx")))]
    pub use crate::message::*;
    #[cfg(feature = "tx")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tx")))]
    pub use crate::transaction::*;

    // RPC trait + helpers
    #[cfg(feature = "rpc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rpc")))]
    pub use crate::rpc::*;

    // Optional codecs
    #[cfg(feature = "codecs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "codecs")))]
    pub use crate::codecs::*;
}
