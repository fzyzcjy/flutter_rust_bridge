// TODO move all these things into io.rs and wasm.rs for symmetry w.r.t. Dart?

#[cfg(not(wasm))]
pub use allo_isolate::IntoDart;
#[cfg(not(wasm))]
use dart_sys::Dart_PersistentHandle;
use log::warn;
use std::{mem, ops, sync::Arc, thread::ThreadId};

#[cfg(wasm)]
pub mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
pub mod io;
#[cfg(not(wasm))]
pub use io::*;

use crate::DartSafe;


#[no_mangle]
pub extern "C" fn initialize_frb_rust() {
    // TODO
    // #[cfg(feature = "rust-async")]
    // crate::rust_async::init();
}
