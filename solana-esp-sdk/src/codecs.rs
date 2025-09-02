/// Write ULEB128-encoded u64 into the buffer (Solana wire format helper).
pub fn uleb128_push(buf: &mut alloc::vec::Vec<u8>, mut v: u64) { /* TODO */ }

/// Read ULEB128 from a byte slice, returning (value, bytes_consumed).
pub fn uleb128_read(input: &[u8]) -> Result<(u64, usize)> { /* TODO */ }

/// Base64 encode (alloc-only, used for sendTransaction).
pub fn b64_encode(bytes: &[u8]) -> alloc::string::String { /* TODO */ }

/// Base58 decode 32 bytes (blockhash, pubkey) — gated to avoid size by default.
#[cfg(feature = "b58")]
pub fn b58_decode_32(s: &str) -> Result<[u8; 32]> { /* TODO */ }

/// Minimal JSON builder: append `"key":"value"` pair (no escaping, embedded use).
#[cfg(feature = "json-min")]
pub fn json_pair_str(dst: &mut alloc::string::String, key: &str, value: &str) { /* TODO */ }

/// Minimal JSON builder: append `"key":number` pair.
#[cfg(feature = "json-min")]
pub fn json_pair_num(dst: &mut alloc::string::String, key: &str, value: u64) { /* TODO */ }