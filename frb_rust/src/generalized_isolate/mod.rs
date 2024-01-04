/// cbindgen:ignore
#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
#[cfg(not(wasm))]
pub use io::*;
