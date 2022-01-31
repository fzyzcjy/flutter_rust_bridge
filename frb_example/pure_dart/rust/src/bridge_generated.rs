#[cfg(not(target_family = "wasm"))]
mod bridge_generated_native;
/// cbindgen:ignore
#[cfg(target_arch = "wasm32")]
mod bridge_generated_web;
#[cfg(not(target_family = "wasm"))]
pub use bridge_generated_native::*;
#[cfg(target_arch = "wasm32")]
pub use bridge_generated_web::*;
