#[cfg(not(target_family = "wasm"))]
mod native;

/// cbindgen:ignore
#[cfg(target_arch = "wasm32")]
mod wasm;
