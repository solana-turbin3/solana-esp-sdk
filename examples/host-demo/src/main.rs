use solana_esp_sdk::{prelude::*, rpc::SimpleRpc};
use rand::rngs::OsRng;


fn main() {
// 1) Generate keypair (host; on device use esp RNG)
let mut rng = OsRng;
let payer = Keypair::generate(&mut rng);


// 2) RPC client (Devnet)
let mut rpc = SimpleRpc::new(
"https://api.devnet.solana.com",
solana_esp_sdk::rpc::host_impl::UreqClient,
);


// 3) Get recent blockhash
let blockhash = rpc.get_latest_blockhash().expect("blockhash");


// 4) Build a tiny transfer (to self, 0 lamports for smoke test)
let tx = compile_simple_transfer(&payer, payer.pubkey(), 0, blockhash).expect("tx");
let b64 = tx.to_base64().expect("b64");


// 5) Submit
let sig = rpc.send_transaction_base64(&b64).expect("send_tx");
println!("submitted: {}", sig);
}