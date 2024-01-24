fn main() {
    // If you want to see logs
    lib_flutter_rust_bridge::utils::logs::configure_opinionated_logging("./logs", true).unwrap();
    // Execute code generator with auto-detected config
    lib_flutter_rust_bridge::codegen::generate_auto().unwrap();
}
