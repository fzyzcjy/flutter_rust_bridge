use crate::generalized_isolate::{Channel, IntoDart};
use crate::platform_types::{handle_to_message_port, DartAbi, SendableMessagePortHandle};
use log::warn;
use std::sync::Arc;
use std::thread::ThreadId;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
use crate::dart_opaque::thread_box::ThreadBox;
use crate::for_generated::{box_from_leak_ptr, new_leak_box_ptr};
#[cfg(not(wasm))]
pub use io::*;

#[cfg(not(wasm))]
mod auto_drop_dart_persistent_handle;

pub(crate) mod dart2rust;
mod non_clone;
pub(crate) mod rust2dart;
mod thread_box;
use non_clone::DartOpaqueNonClone;

/// Arbitrary Dart object, whose type can be even non-encodable and non-transferable.
// Implementation: Just [DartOpaqueInner] + Arc, in order to support `clone`
#[derive(Debug)]
pub struct DartOpaque {
    // TODO `Arc` is for `DartOpaque` to be clone-able.
    //      When users do not need clone (e.g. NOT used in a DartFn that is called multiple times),
    //      we can generate and use the non-Arc version to speed up.
    arc: Arc<DartOpaqueNonClone>,
}

impl DartOpaque {
    pub fn new(handle: GeneralizedDartHandle, drop_port: SendableMessagePortHandle) -> Self {
        Self {
            arc: Arc::new(DartOpaqueNonClone::new(handle, drop_port)),
        }
    }

    pub fn into_inner(mut self) -> Result<GeneralizedAutoDropDartPersistentHandle, Self> {
        let inner = Arc::try_unwrap(self.arc).map_err(|x| Self { arc: x })?;
        Ok(inner.into_inner())
    }

    fn create_dart_handle(&self) -> GeneralizedDartHandle {
        self.arc.create_dart_handle()
    }

    pub fn into_raw(self) -> *const std::ffi::c_void {
        Arc::into_raw(self.arc) as _
    }

    pub unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self {
        Self {
            arc: Arc::from_raw(raw as _),
        }
    }
}

impl Clone for DartOpaque {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}
