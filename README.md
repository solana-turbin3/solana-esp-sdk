![Solana ESP SDK](./img/banner.png "Solana ESP SDK")

A lightweight, `no_std`-first Rust SDK that lets Espressif microcontrollers (ESP32 family) sign Solana transactions, read on-chain state, and submit RPC requests. The SDK is modular and feature-gated so embedded builds only include what they need.


## Quick start (host demo)
```bash
# 1) build & flash the demo program to the esp32s3. 
cargo install espup
espup install
. /Users/user/export.esp.sh # Run everytime you restart terminal, or add to your PATH. 
cargo +esp run -p esp32s3-demo --release
```


### What is this?
- **Embedded-friendly Solana client**: `no_std` core for bare-metal targets, with an optional `std` feature for host testing.
- **Modular by design**: small, opt-in modules for crypto, codecs, instructions, message/transaction assembly, RPC, and network adapters.
- **Transport-agnostic**: a simple `RpcClient` trait powers networking across smoltcp, reqwless, ESP TLS, or a host HTTP client.


### Why build this?
- **Direct on-chain IoT**: ESP32 devices can sense, compute, and now also transact directly with Solana for secure data reporting, rewards, and decentralized IoT apps.
- **No existing `no_std` client**: `solana-program` (on-chain, `no_std`) and `solana-sdk` (off-chain, `std`) exist, but nothing supports embedded off-chain clients. This SDK fills that gap.
- **Tight resource budgets**: Embedded systems have limited flash/RAM. Feature-gated modules keep binaries small and predictable by compiling only what you use.
- **Security and correctness**: Uses `ed25519-compact` for signing and implements wire serialization to match Solana’s exact byte format, validated against the upstream SDK.


### Design and modules (feature-gated)
- **types (always on)**: `Result`, `SdkError`, tiny core types.
- **crypto (`feature = "crypto"`)**: keypairs, signing, verification.
- **codecs (`feature = "codecs"`)**: ULEB128, base64, optional base58/JSON helpers.
- **instruction (`feature = "instr"`)**: System and Memo instruction builders.
- **spl (`feature = "spl-token"`)**: SPL token helpers.
- **message`/`transaction (`feature = "tx"`)**: message compilation and transaction signing/serialization.
- **rpc (`feature = "rpc"`)**: JSON-RPC request builders and parsers; transport-agnostic.
- **net (`features = "net-smoltcp" | "net-reqwless" | "net-std"`)**: optional network adapters implementing `RpcClient`.
- **prelude**: curated re-exports of only what’s enabled.


### Examples
- **host-demo (std)**: smoke test against Solana devnet.
- **esp32s3-demo (no_std)**: embedded Wi‑Fi + `RpcClient` integration skeleton.


### How to contribute
We’re developing this SDK incrementally with small, composable modules. See `tasks.md` for milestones and current priorities. High‑impact areas where contributions are welcome:

- **Wire serialization correctness**
  - Implement message and transaction serialization to exact Solana format.
  - Add unit tests (std-only) that compare bytes with the official SDK.

- **Host RPC happy path (`std`)**
  - Implement `get_latest_blockhash`, `send_transaction_base64`, and parsing helpers in `solana-esp-sdk/src/rpc.rs`.
  - Use `serde_json` and `bs58` behind the `std` feature.

- **Embedded networking (`no_std`)**
  - Bring up Wi‑Fi on ESP32‑S3 using `esp-hal` + `esp-wifi`.
  - Implement a minimal HTTP POST client (manual, `smoltcp`, or `reqwless`) and an `RpcClient` adapter in `solana-esp-sdk/src/net.rs`.

- **On‑chain reads**
  - Add `getBalance`/`getAccountInfo` support and minimal parsers for `no_std`.

- **SPL-Token helpers**
  - Implement `spl_token_transfer` behind the `spl-token` feature with tests.

- **Hardening & size trimming**
  - Keep TLS optional, gate heavier deps behind features, adopt `heapless` where practical, and add retry/error handling for RPC.

Contribution guidelines:
- Favor **small, focused PRs** that enable or flesh out a single module/feature.
- Keep the **core `no_std` by default**; gate extras behind features.
- Minimize dependencies and be mindful of **flash/RAM size**.
- Add **tests under `std`** where possible; keep embedded paths lean.


### Outcome
- Embedded developers can integrate Solana into ESP projects with minimal overhead.
- IoT devices become first‑class Solana clients for signing, sending, and reading on-chain state.
- The repo structure enables parallel, modular contributions without bloating builds.