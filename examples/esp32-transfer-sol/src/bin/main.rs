#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_net::dns::DnsSocket; //
use embassy_net::tcp::client::{TcpClient, TcpClientState}; //
use embassy_net::{DhcpConfig, Runner, Stack, StackResources}; //
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::rng::Rng; //
use esp_hal::timer::timg::TimerGroup;
use esp_println as _;
use esp_println::println; //
use esp_wifi::wifi::{self, WifiController, WifiDevice, WifiEvent, WifiState}; //
use esp_wifi::EspWifiController;
use solana_esp_sdk::crypto::{Keypair, Pubkey};
use solana_esp_sdk::hash::Hash;
//
use solana_esp_sdk::net::ReqwlessAsyncClient;
use solana_esp_sdk::prelude::{AccountMeta, Instruction, Transaction};
use solana_esp_sdk::rpc::RpcClient; //

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("Panic! {:?}", info);
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

// If you are okay with using a nightly compiler, you can use the macro provided by the static_cell crate: https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");
include!(concat!(env!("OUT_DIR"), "/private_key.rs"));

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024); //

    let timer0 = TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");
    // info!("private key: {}", PRIVATE_KEY);

    let mut rng = Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);

    let esp_wifi_ctrl = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng.clone()).unwrap()
    );

    let (controller, interfaces) = esp_wifi::wifi::new(&esp_wifi_ctrl, peripherals.WIFI).unwrap();
    let wifi_interface = interfaces.sta;

    let net_seed = rng.random() as u64 | ((rng.random() as u64) << 32);
    let tls_seed = rng.random() as u64 | ((rng.random() as u64) << 32);

    let dhcp_config = DhcpConfig::default();

    let config = embassy_net::Config::dhcpv4(dhcp_config);
    // Init network stack
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        net_seed,
    );

    spawner.spawn(connection(controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    wait_for_connection(stack).await;

    let tcp_state = TcpClientState::<1, 4096, 4096>::new();
    let mut tcp = TcpClient::new(stack, &tcp_state);
    tcp.set_timeout(Some(Duration::from_secs(60)));
    let dns = DnsSocket::new(stack);
    let async_client = ReqwlessAsyncClient { tcp, dns, tls_seed };
    let rpc = RpcClient::new_async(
        "https://api.devnet.solana.com",
        solana_esp_sdk::rpc::Commitment::Finalized,
        async_client,
    );

    info!("rpc initialized!");
    let recent_hash: Hash;
    loop {
        let hash = rpc.get_latest_blockhash().await;
        match hash {
            Ok(hash) => {
                println!("hash: {}", hash);
                recent_hash = hash;
                break;
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    let keypair = Keypair::new_from_array(PRIVATE_KEY);
    let keypair_pubkey = keypair.public_key();

    println!("keypair_pubkey: {}", keypair_pubkey);

    let system_program_id = Pubkey::new([0; 32]);

    let to_pubkey = Pubkey::new([
        160, 129, 7, 170, 83, 212, 101, 228, 89, 56, 29, 12, 167, 106, 199, 150, 128, 228, 115, 53,
        77, 79, 132, 139, 127, 102, 145, 110, 234, 82, 85, 138,
    ]);

    println!("to_pubkey: {}", to_pubkey);

    let from_account = AccountMeta::new_writable(&keypair_pubkey, true);
    let to_account = AccountMeta::new_writable(&to_pubkey, false);

    let lamports: u64 = 1000;

    let mut data: [u8; 12] = [0; 12];
    data[0] = 2;
    data[4..12].copy_from_slice(&lamports.to_le_bytes());

    let instruction = Instruction {
        program_id: &system_program_id,
        data: &data,
        accounts: &[from_account, to_account],
    };

    let transaction = Transaction {
        signers: &[&keypair],
        instructions: &[instruction],
        recent_blockhash: &recent_hash,
    };

    let signature = rpc.send_transaction(&transaction).await;
    match signature {
        Ok(signature) => println!("signature: {:?}", signature),
        Err(e) => println!("Error: {:?}", e),
    }

    // let pubkey = "ALcEQcnFpwij9xBKmUuz8QAyQkwtVDxhhvrogS9VGY3P";

    // let mut data_buffer = [0; 200];
    // let mut resp_buffer = [0; 2048];

    // let data = rpc
    //     .get_data(pubkey, &mut data_buffer, &mut resp_buffer)
    //     .await;
    // match data {
    //     Ok(data) => println!("data: {:?}", data),
    //     Err(e) => println!("Error: {:?}", e),
    // }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-rc.0/examples/src/bin
}

async fn wait_for_connection(stack: Stack<'_>) {
    println!("Waiting for link to be up");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = wifi::Configuration::Client(wifi::ClientConfiguration {
                ssid: SSID.try_into().unwrap(),
                password: PASSWORD.try_into().unwrap(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
        println!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {:?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}
