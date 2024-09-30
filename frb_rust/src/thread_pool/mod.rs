#[cfg(feature = "thread-pool")]
#[cfg(target_family = "wasm")]
mod web;
#[cfg(feature = "thread-pool")]
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(feature = "thread-pool")]
#[cfg(not(target_family = "wasm"))]
mod io;
#[cfg(feature = "thread-pool")]
#[cfg(not(target_family = "wasm"))]
pub use io::*;

#[cfg(not(feature = "thread-pool"))]
mod stub;
#[cfg(not(feature = "thread-pool"))]
pub use stub::*;
