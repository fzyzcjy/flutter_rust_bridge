use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use log::warn;
use std::thread::ThreadId;

#[cfg(wasm)]
mod web;
#[cfg(wasm)]
pub use web::*;

#[cfg(not(wasm))]
mod io;
#[cfg(not(wasm))]
pub use io::*;

/// Arbitrary Dart object, whose type can be even non-encodable and non-transferable.
#[derive(Debug)]
pub struct DartOpaque {
    /// Dart object
    handle: Option<DartOpaqueBase>,

    /// The ID of the thread on which the Dart Object was created.
    thread_id: ThreadId,
}

/// # Safety
///
/// The implementation checks the current thread
/// and delegates it to the Dart thread when it is drops.
unsafe impl Send for DartOpaque {}
unsafe impl Sync for DartOpaque {}

impl DartOpaque {
    /// Creates a new [DartOpaque].
    ///
    /// # Safety
    ///
    /// The [DartObject] must be created on the current thread.
    pub unsafe fn new(handle: DartObject, port: OpaqueMessagePort) -> Self {
        Self {
            handle: Some(DartOpaqueBase::new(handle, Some(port))),
            thread_id: std::thread::current().id(),
        }
    }

    /// Creates a [DartOpaque] for sending to dart.
    ///
    /// # Safety
    ///
    /// The [DartObject] must be created on the current thread.
    ///
    /// The [DartOpaque] created by this method must not be dropped
    /// on a non-parent [DartObject] thread.
    pub unsafe fn new_non_droppable(handle: DartObject) -> Self {
        Self {
            handle: Some(DartOpaqueBase::new(handle, None)),
            thread_id: std::thread::current().id(),
        }
    }

    /// Tries to get a Dart [DartObject].
    /// Returns the [DartObject] if the [DartOpaque] was created on the current thread.
    pub fn try_unwrap(mut self) -> Result<DartWrapObject, Self> {
        if std::thread::current().id() == self.thread_id {
            Ok(self.handle.take().unwrap().unwrap())
        } else {
            Err(self)
        }
    }
}

impl From<DartOpaque> for DartAbi {
    fn from(mut data: DartOpaque) -> Self {
        data.handle.take().unwrap().into_raw().into_dart()
    }
}

impl Drop for DartOpaque {
    fn drop(&mut self) {
        if let Some(inner) = self.handle.take() {
            if std::thread::current().id() != self.thread_id {
                if let Some(channel) = inner.channel() {
                    let ptr = inner.into_raw();

                    if !channel.post(ptr) {
                        warn!("Drop DartOpaque after closing the port.");
                    };
                } else {
                    warn!("Drop non droppable DartOpaque.");
                }
            }
        }
    }
}

// TODO improve
pub(crate) unsafe fn wire2api_dart_opaque(raw: i64) -> DartOpaque {
    DartOpaque::new(raw as _, todo!("should remove the port argument"))
}
