use crate::frb_generated::FLUTTER_RUST_BRIDGE_CODEGEN_VERSION;
use flutter_rust_bridge::frb;

#[cfg(not(target_family = "wasm"))]
flutter_rust_bridge::for_generated::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER:flutter_rust_bridge::DefaultHandler<flutter_rust_bridge::for_generated::SimpleThreadPool> = {
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
    pub static THREAD_POOL: flutter_rust_bridge::for_generated::SimpleThreadPool = flutter_rust_bridge::for_generated::SimpleThreadPool::new(None, None, Some("console.log('hi I am worker js preamble')".to_string())).unwrap();
}

#[cfg(target_family = "wasm")]
flutter_rust_bridge::for_generated::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: flutter_rust_bridge::DefaultHandler<&'static std::thread::LocalKey<flutter_rust_bridge::for_generated::SimpleThreadPool>>
        = flutter_rust_bridge::DefaultHandler::new_simple(&THREAD_POOL);
}

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
