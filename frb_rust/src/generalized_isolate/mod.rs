#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(wasm)]
mod web_into_dart;
#[cfg(wasm)]
pub use web_into_dart::*;

#[cfg(not(wasm))]
mod io;

#[cfg(not(wasm))]
pub use io::*;
