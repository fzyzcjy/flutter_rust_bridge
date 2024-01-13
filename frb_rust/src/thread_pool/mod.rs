#[cfg(feature = "thread-pool")]
#[cfg(wasm)]
mod web;
#[cfg(feature = "thread-pool")]
#[cfg(wasm)]
pub use web::*;

#[cfg(feature = "thread-pool")]
#[cfg(not(wasm))]
mod io;
#[cfg(feature = "thread-pool")]
#[cfg(not(wasm))]
pub use io::*;

#[cfg(not(feature = "thread-pool"))]
mod stub;
#[cfg(not(feature = "thread-pool"))]
pub use stub::*;
