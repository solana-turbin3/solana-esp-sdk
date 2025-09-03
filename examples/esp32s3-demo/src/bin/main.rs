#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;

use esp_println::println;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_wifi::wifi::{ClientConfiguration, Configuration};
use embassy_net::{Config as NetConfig, Stack, StackResources, Runner};
use static_cell::StaticCell;

// Note: This demo intentionally does not use the Solana SDK yet.

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.4.0

    println!("boot: start");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
    println!("embassy: inited");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init = alloc::boxed::Box::leak(alloc::boxed::Box::new(
        esp_wifi::init(timer1.timer0, rng)
            .expect("Failed to initialize WIFI/BLE controller"),
    ));
    println!("wifi: init ok");

    // Configure Wi‑Fi as STA and connect
    let (mut wifi_controller, interfaces) = esp_wifi::wifi::new(wifi_init, peripherals.WIFI)
        .expect("Failed to initialize WIFI controller");
    let wifi_cfg = Configuration::Client(ClientConfiguration {
        ssid: "SSID_HERE".into(),
        password: "PASSWORD_HERE".into(),
        ..Default::default()
    });
    wifi_controller.set_configuration(&wifi_cfg).expect("wifi config");
    println!("wifi: configured, starting");
    wifi_controller.start().expect("wifi start");
    wifi_controller.connect().expect("wifi connect");
    println!("wifi: connect called");

    // embassy-net stack over esp-wifi (DHCPv4)
    static NET_STACK: StaticCell<Stack<'static>> = StaticCell::new();
    static NET_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let seed = 0x0123_4567; // any u64 random seed
    let net_cfg = NetConfig::dhcpv4(Default::default());
    let device = interfaces.sta;
    let (stack_tmp, runner) = embassy_net::new(
        device,
        net_cfg,
        NET_RESOURCES.init(StackResources::new()),
        seed,
    );
    let stack = NET_STACK.init(stack_tmp);
    println!("net: stack created, spawning runner");
    spawner.spawn(net_task(runner)).ok();
    println!("net: waiting for DHCP");
    stack.wait_config_up().await;
    println!("net: up");

    // Demo loop: print assigned IPv4 address periodically
    loop {
        if let Some(config) = stack.config_v4() {
            let ipv4 = config.address.address();
            println!("ip: {}", ipv4);
        } else {
            println!("ip: (not configured)");
        }
        Timer::after(Duration::from_secs(2)).await;
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, esp_wifi::wifi::WifiDevice<'static>>) {
    println!("net: runner start");
    runner.run().await;
}

