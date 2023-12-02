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

#[cfg(not(wasm))]
pub type Channel = allo_isolate::Isolate;

use crate::DartSafe;

/// see [uuid::Bytes](https://docs.rs/uuid/1.1.2/uuid/type.Bytes.html)
#[cfg(feature = "uuid")]
const UUID_SIZE_IN_BYTES: usize = 16;

#[cfg(feature = "uuid")]
#[inline]
pub fn wire2api_uuid_ref(id: &[u8]) -> uuid::Uuid {
    uuid::Uuid::from_bytes(
        *<&[u8] as std::convert::TryInto<&[u8; UUID_SIZE_IN_BYTES]>>::try_into(id)
            .expect("invalid uuid slice"),
    )
}

#[cfg(feature = "uuid")]
#[inline]
pub fn wire2api_uuid(id: Vec<u8>) -> uuid::Uuid {
    wire2api_uuid_ref(id.as_slice())
}

#[cfg(feature = "uuid")]
#[inline]
pub fn wire2api_uuids(ids: Vec<u8>) -> Vec<uuid::Uuid> {
    ids.as_slice()
        .chunks(UUID_SIZE_IN_BYTES)
        .map(wire2api_uuid_ref)
        .collect::<Vec<uuid::Uuid>>()
}

#[no_mangle]
pub extern "C" fn initialize_frb_rust() {
    // TODO
    // #[cfg(feature = "rust-async")]
    // crate::rust_async::init();
}
