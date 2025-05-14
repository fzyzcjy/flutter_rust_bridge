//! RwLock implementation backed by tokio if "rust-async" feature is enabled
//! and by std otherwise.

#[cfg(not(feature = "rust-async"))]
mod sync;

#[cfg(not(feature = "rust-async"))]
pub use sync::*;

#[cfg(feature = "rust-async")]
mod tokio;
use std::fmt;
#[cfg(feature = "rust-async")]
pub use tokio::*;

#[derive(Clone, Copy, Debug)]
pub struct TryLockError;

impl fmt::Display for TryLockError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "operation would block")
    }
}
