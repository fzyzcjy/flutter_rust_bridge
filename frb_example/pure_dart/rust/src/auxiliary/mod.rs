pub mod benchmark_raw;
// For simplicity, only test non-wasm
#[cfg(not(target_family = "wasm"))]
pub mod custom_handler;
pub mod new_module_system;
pub mod old_module_system;
pub mod protobuf_for_benchmark;
pub mod sample_types;
