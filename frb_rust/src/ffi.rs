#[cfg(not(target_family = "wasm"))]
pub use allo_isolate::{DartCObject, IntoDart, IntoDartExceptPrimitive};

#[cfg(target_arch = "wasm32")]
pub use allo_isolate::web_ffi::*;
