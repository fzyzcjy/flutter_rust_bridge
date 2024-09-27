// For some reason Clippy thinks that the `AtomicBool` import is unused, but it's needed by
// `base_arc_generate_tests!`
#[cfg(feature = "portable-atomic")]
#[allow(unused_imports)]
pub(crate) use portable_atomic::{AtomicBool, AtomicI32, AtomicU64};
pub(crate) use std::sync::atomic::Ordering;
#[cfg(not(feature = "portable-atomic"))]
pub(crate) use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64};
