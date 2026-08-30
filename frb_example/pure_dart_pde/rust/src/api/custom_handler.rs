// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

#[allow(unused_imports)]
use crate::frb_generated::FLUTTER_RUST_BRIDGE_CODEGEN_VERSION;

// This file demonstrates how to use a custom handler.
// Usually there is no need for this, and the default handler is used.
//
// Here, for simplicity, we still generate code for default handler.
// But surely you can copy-paste the content of that macro and modify according to your needs.
flutter_rust_bridge::frb_generated_default_handler!();

#[cfg(frb_sanitize_runtime_shutdown)]
#[no_mangle]
pub extern "C" fn frb_shutdown_sanitizer_runtime() {
    FLUTTER_RUST_BRIDGE_HANDLER.thread_pool().0.join();
    FLUTTER_RUST_BRIDGE_HANDLER.async_runtime().shutdown();
}

// NOTE: For more tests about customizing handler types and contents, please visit `src/auxiliary/custom_handler.rs`
