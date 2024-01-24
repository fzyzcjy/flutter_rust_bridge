use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;
use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_logging;

fn main() -> anyhow::Result<()> {
    // If you want to see logs
    // Alternatively, use `cargo build -vvv` (instead of `cargo build`) to see logs on screen
    configure_opinionated_logging("./logs/", true)?;

    // Execute code generator with auto-detected config
    codegen::generate(
        Config::from_config_file("../flutter_rust_bridge.yaml")?.unwrap(),
        Default::default(),
    )
}
