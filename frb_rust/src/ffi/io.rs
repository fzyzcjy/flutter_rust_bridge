use crate::Channel;
use crate::Dart_DeletePersistentHandle_DL_Trampolined;
use crate::Dart_HandleFromPersistent_DL_Trampolined;
use crate::Dart_NewPersistentHandle_DL_Trampolined;

pub use super::DartAbi;
pub use super::MessagePort;
pub use allo_isolate::*;
pub use dart_sys::Dart_Handle;
pub use dart_sys::Dart_PersistentHandle;
pub use dart_sys::_Dart_Handle;
use std::thread::ThreadId;

#[cfg(feature = "chrono")]
#[inline]
pub fn wire2api_timestamp(ts: i64) -> Timestamp {
    let s = (ts / 1_000_000) as i64;
    let ns = (ts.rem_euclid(1_000_000) * 1_000) as u32;
    Timestamp { s, ns }
}
/// a timestamp with microseconds precision
#[cfg(feature = "chrono")]
pub struct Timestamp {
    /// seconds
    pub s: i64,
    /// nanoseconds
    pub ns: u32,
}

#[cfg(test)]
#[cfg(feature = "chrono")]
mod tests {
    #[test]
    fn wire2api() {
        // input in microseconds
        let input: i64 = 3_496_567_123;
        let super::Timestamp { s, ns } = super::wire2api_timestamp(input);
        assert_eq!(s, 3_496);
        assert_eq!(ns, 567_123_000);
    }
}

#[no_mangle]
pub unsafe extern "C" fn dart_opaque_drop(ptr: usize) {
    Dart_DeletePersistentHandle_DL_Trampolined(ptr as _);
}

#[no_mangle]
pub unsafe extern "C" fn dart_opaque_get(ptr: usize) -> *mut _Dart_Handle {
    let res = Dart_HandleFromPersistent_DL_Trampolined(ptr as _);
    Dart_DeletePersistentHandle_DL_Trampolined(ptr as _);
    res
}

pub struct DartOpaque {
    handle: Dart_PersistentHandle,
    port: Channel,
    drop: bool,
    id: ThreadId,
}

unsafe impl Send for DartOpaque {}
unsafe impl Sync for DartOpaque {}

impl DartOpaque {
    pub fn new(handle: Dart_Handle, port: i64) -> Self {
        Self {
            handle: handle,
            port: Channel::new(port),
            id: std::thread::current().id(),
            drop: true,
        }
    }

    pub fn try_unwrap(self) -> Result<Dart_PersistentHandle, Self> {
        if std::thread::current().id() == self.id {
            Ok(self.handle)
        } else {
            Err(self)
        }
    }
}

impl IntoDart for DartOpaque {
    fn into_dart(mut self) -> ffi::DartCObject {
        self.drop = false;
        self.handle.into_dart()
    }
}

impl Clone for DartOpaque {
    fn clone(&self) -> Self {
        Self {
            handle: unsafe { Dart_NewPersistentHandle_DL_Trampolined(self.handle) },
            port: self.port,
            drop: true,
            id: self.id,
        }
    }
}

impl Drop for DartOpaque {
    fn drop(&mut self) {
        if self.drop {
            if std::thread::current().id() == self.id {
                unsafe { Dart_DeletePersistentHandle_DL_Trampolined(self.handle) }
            } else {
                self.port.post(self.handle.into_dart());
            }
        }
    }
}
