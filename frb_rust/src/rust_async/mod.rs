#[cfg(feature = "rust-async")]
#[cfg(not(wasm))]
mod io;
#[cfg(feature = "rust-async")]
#[cfg(not(wasm))]
pub use io::*;

#[cfg(feature = "rust-async")]
#[cfg(wasm)]
mod web;
#[cfg(feature = "rust-async")]
#[cfg(wasm)]
pub use web::*;

#[cfg(not(feature = "rust-async"))]
mod stub;
#[cfg(not(feature = "rust-async"))]
pub use stub::*;
