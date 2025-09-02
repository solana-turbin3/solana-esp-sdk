# Solana ESP SDK (skeleton)


A lightweight, `no_std`-first Rust SDK to sign and submit Solana transactions from Espressif microcontrollers via `esp-hal`.


## Quick start (host demo)
```bash
# 1) build & run the std-based demo on your laptop
cargo run -p host-demo
```


> Note: The current skeleton includes placeholder serialization and a minimal RPC parser under `std`. Replace the base58/blockhash decode with a fixed 32-byte hash during early testing, or wire a proper base58->Hash conversion.


## Embedded (ESP32-S3)
- Add `esp-hal`, `esp-wifi`, and a TCP/IP stack (e.g., `smoltcp`), then implement `RpcClient` by opening a socket and POSTing JSON to your RPC endpoint.
- Provide RNG to `Keypair::generate` from the HAL RNG peripheral.
- Keep TLS optional (feature `tls`).


## Crate features
- `std`: enables host testing helpers and JSON parsing via `serde_json`.
- `spl-token`: adds SPL token transfer instruction helper.
- `tls`: reserved for future TLS adapters.


## Status
This is a minimal skeleton to establish project shape, module boundaries, and example usage. You’ll need to:
- Finish Solana wire-format serialization to match upstream exactly.
- Implement a real base58 decoder for blockhashes and signatures (host-demo shows bs58 usage under `std`).
- Add proper error handling and retries for RPC.
- Flesh out the embedded example with Wi‑Fi bring‑up and an `RpcClient` implementation.