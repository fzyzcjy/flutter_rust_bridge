#[cfg(feature = "rust-async")]
#[cfg(not(target_family = "wasm"))]
mod io;
#[cfg(feature = "rust-async")]
#[cfg(not(target_family = "wasm"))]
pub use io::*;

#[cfg(feature = "rust-async")]
#[cfg(target_family = "wasm")]
mod web;
#[cfg(feature = "rust-async")]
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(feature = "rust-async"))]
mod stub;
#[cfg(not(feature = "rust-async"))]
pub use stub::*;
