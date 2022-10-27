#[cfg(wasm)]
pub type DartAbi = wasm_bindgen::JsValue;
#[cfg(not(wasm))]
pub type DartAbi = allo_isolate::ffi::DartCObject;

#[cfg(wasm)]
pub trait IntoDart {
    fn into_dart(self) -> DartAbi;
}
use std::{ffi::c_void, sync::Arc};

#[cfg(not(wasm))]
pub use allo_isolate::IntoDart;

#[cfg(wasm)]
pub type MessagePort = web::PortLike;
#[cfg(not(wasm))]
pub type MessagePort = i64;

#[cfg(wasm)]
pub mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
pub type Channel = allo_isolate::Isolate;

#[cfg(not(wasm))]
pub mod io;
#[cfg(not(wasm))]
pub use io::*;

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

/// Equivalent to a [`Arc::clone()`], but directly in terms of raw pointers.
///
/// # Safety
///
/// This function should never be called manually.
extern "C" fn share_arc<T>(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<T>::increment_strong_count(ptr as _);
        ptr
    }
}

/// Dropper opaque data.
///
/// # Safety
///
/// This function should never be called manually.
extern "C" fn drop_arc<T>(ptr: *const c_void) {
    // Dart has ownership of this copy of Arc, and can only share out clones,
    // so this is safe to call exactly once.
    unsafe {
        Arc::<T>::decrement_strong_count(ptr as _);
    }
}
