use crate::platform_types::SendableMessagePortHandle;
use std::sync::Arc;

/// cbindgen:ignore
#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;

#[cfg(not(wasm))]
pub use io::*;

#[cfg(not(wasm))]
mod auto_drop_dart_persistent_handle;

pub(crate) mod action;
pub(crate) mod boxes;
pub(crate) mod dart2rust;
mod non_clone;
pub(crate) mod rust2dart;

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
    pub fn new(
        handle: GeneralizedDartHandle,
        dart_handler_port: SendableMessagePortHandle,
    ) -> Self {
        Self {
            arc: Arc::new(DartOpaqueNonClone::new(handle, dart_handler_port)),
        }
    }

    pub fn into_inner(self) -> Result<GeneralizedAutoDropDartPersistentHandle, Self> {
        let inner = Arc::try_unwrap(self.arc).map_err(|x| Self { arc: x })?;
        Ok(inner.into_inner())
    }

    fn create_dart_handle(&self) -> GeneralizedDartHandle {
        self.arc.create_dart_handle()
    }

    pub fn into_raw(self) -> *const std::ffi::c_void {
        Arc::into_raw(self.arc) as _
    }

    /// # Safety
    ///
    /// This should never be called manually.
    pub unsafe fn from_raw(raw: *const std::ffi::c_void) -> Self {
        Self {
            arc: Arc::from_raw(raw as _),
        }
    }

    pub(crate) fn dart_handler_port(&self) -> &SendableMessagePortHandle {
        self.arc.dart_handler_port()
    }
}

impl Clone for DartOpaque {
    fn clone(&self) -> Self {
        Self {
            arc: self.arc.clone(),
        }
    }
}
