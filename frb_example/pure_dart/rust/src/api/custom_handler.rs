// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use lazy_static::lazy_static;

/// This file demonstrates how to use a custom handler.
/// Usually there is no need for this, and the default handler is used.

#[cfg(not(target_family = "wasm"))]
lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: flutter_rust_bridge::DefaultHandler<flutter_rust_bridge::for_generated::SimpleThreadPool> = {
        assert_eq!(
            FLUTTER_RUST_BRIDGE_CODEGEN_VERSION,
            flutter_rust_bridge::for_generated::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
            "Please ensure flutter_rust_bridge's codegen ({}) and runtime ({}) versions are the same",
            FLUTTER_RUST_BRIDGE_CODEGEN_VERSION,
            flutter_rust_bridge::for_generated::FLUTTER_RUST_BRIDGE_RUNTIME_VERSION,
        );

        flutter_rust_bridge::DefaultHandler::new_simple(Default::default())
    };
}

#[cfg(target_family = "wasm")]
thread_local! {
    pub static THREAD_POOL: flutter_rust_bridge::for_generated::SimpleThreadPool = Default::default();
}

#[cfg(target_family = "wasm")]
lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: flutter_rust_bridge::DefaultHandler<&'static std::thread::LocalKey<flutter_rust_bridge::for_generated::SimpleThreadPool>>
        = flutter_rust_bridge::DefaultHandler::new_simple(&THREAD_POOL);
}
