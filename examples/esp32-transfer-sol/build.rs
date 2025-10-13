use std::env;
use std::fs;
use std::path::Path;

fn main() {
    dotenv_build::output(dotenv_build::Config::default()).unwrap();

    let path = Path::new("private_key.txt");
    let content = fs::read_to_string(path).expect("Failed to read private_key.txt");

    // Parse the array string into a Vec<u8>
    let trimmed = content.trim_matches(|c| c == '[' || c == ']');
    let numbers: Vec<u8> = trimmed
        .split(',')
        .map(|s| s.trim().parse().expect("Failed to parse number"))
        .collect();

    assert_eq!(numbers.len(), 64, "Private key must be 64 bytes");

    // Write as a Rust const to a generated file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("private_key.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    std::io::Write::write_all(
        &mut f,
        format!(
            "pub const PRIVATE_KEY: [u8; 64] = {};\n",
            format!("{:?}", numbers)
        )
        .as_bytes(),
    )
    .unwrap();

    linker_be_nice();
    println!("cargo:rustc-link-arg=-Tdefmt.x");
    // make sure linkall.x is the last linker script (otherwise might cause problems with flip-link)
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn linker_be_nice() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let kind = &args[1];
        let what = &args[2];

        match kind.as_str() {
            "undefined-symbol" => match what.as_str() {
                "_defmt_timestamp" => {
                    eprintln!();
                    eprintln!("💡 `defmt` not found - make sure `defmt.x` is added as a linker script and you have included `use defmt_rtt as _;`");
                    eprintln!();
                }
                "_stack_start" => {
                    eprintln!();
                    eprintln!("💡 Is the linker script `linkall.x` missing?");
                    eprintln!();
                }
                "esp_wifi_preempt_enable"
                | "esp_wifi_preempt_yield_task"
                | "esp_wifi_preempt_task_create" => {
                    eprintln!();
                    eprintln!("💡 `esp-wifi` has no scheduler enabled. Make sure you have the `builtin-scheduler` feature enabled, or that you provide an external scheduler.");
                    eprintln!();
                }
                "embedded_test_linker_file_not_added_to_rustflags" => {
                    eprintln!();
                    eprintln!("💡 `embedded-test` not found - make sure `embedded-test.x` is added as a linker script for tests");
                    eprintln!();
                }
                _ => (),
            },
            // we don't have anything helpful for "missing-lib" yet
            _ => {
                std::process::exit(1);
            }
        }

        std::process::exit(0);
    }

    println!(
        "cargo:rustc-link-arg=-Wl,--error-handling-script={}",
        std::env::current_exe().unwrap().display()
    );
}
