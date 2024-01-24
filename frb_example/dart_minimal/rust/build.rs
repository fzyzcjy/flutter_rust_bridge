use lib_flutter_rust_bridge_codegen::codegen;
use lib_flutter_rust_bridge_codegen::codegen::Config;
use lib_flutter_rust_bridge_codegen::utils::logs::configure_opinionated_logging;

fn main() {
    // If you want to see logs
    configure_opinionated_logging("./logs", true).unwrap();
    // Execute code generator with auto-detected config
    codegen::generate(Config::from_files_auto().unwrap(), Default::default()).unwrap();
}
