/// cbindgen:ignore
#[cfg(target_family = "wasm")]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
