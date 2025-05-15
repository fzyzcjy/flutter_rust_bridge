#[cfg(feature = "rust-async")]
#[cfg(not(target_family = "wasm"))]
mod io;

#[cfg(feature = "rust-async")]
#[cfg(target_family = "wasm")]
mod web;

#[cfg(not(feature = "rust-async"))]
mod stub;

// So it would be accessible under flutter_rust_bridge::for_generated::rust_async::*
pub use crate::rw_lock::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
#[cfg(feature = "rust-async")]
#[cfg(not(target_family = "wasm"))]
pub use io::*;
#[cfg(not(feature = "rust-async"))]
pub use stub::*;
#[cfg(feature = "rust-async")]
#[cfg(target_family = "wasm")]
pub use web::*;
