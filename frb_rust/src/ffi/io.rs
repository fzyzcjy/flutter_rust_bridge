use crate::Channel;

pub use super::DartAbi;
pub use super::MessagePort;
use crate::Dart_DeletePersistentHandle_DL_Trampolined;
use crate::Dart_HandleFromPersistent_DL_Trampolined;
use crate::Dart_NewPersistentHandle_DL_Trampolined;
pub use allo_isolate::*;
pub use dart_sys::Dart_Handle;
pub use dart_sys::Dart_PersistentHandle;

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

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn new_dart_opaque(handle: Dart_Handle) -> usize {
    Dart_NewPersistentHandle_DL_Trampolined(handle) as _
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn get_dart_object(ptr: usize) -> Dart_Handle {
    Dart_HandleFromPersistent_DL_Trampolined(ptr as _)
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn drop_dart_object(ptr: usize) {
    Dart_DeletePersistentHandle_DL_Trampolined(ptr as _);
}

#[derive(Debug)]
/// Option for correct drop.
pub struct DartHandleWrap(Option<Dart_PersistentHandle>);

impl DartHandleWrap {
    pub fn from_raw(ptr: Dart_PersistentHandle) -> Self {
        Self(Some(ptr))
    }

    pub fn into_raw(mut self) -> Dart_PersistentHandle {
        self.0.take().unwrap()
    }
}

impl Drop for DartHandleWrap {
    fn drop(&mut self) {
        if let Some(inner) = self.0 {
            unsafe { Dart_DeletePersistentHandle_DL_Trampolined(inner) }
        }
    }
}

#[derive(Debug)]
pub struct DartOpaqueBase {
    inner: DartHandleWrap,
    drop_port: Option<MessagePort>,
}

impl DartOpaqueBase {
    pub fn new(handle: Dart_PersistentHandle, port: MessagePort) -> Self {
        Self {
            inner: DartHandleWrap::from_raw(handle),
            drop_port: Some(port),
        }
    }

    /// Creates a new wrapper for send to dart.
    ///
    /// # Safety
    ///
    /// Must be used for Dart objects created on the current thread
    /// only to be sent to the Dart side.
    pub unsafe fn new_non_dropable(handle: Dart_Handle) -> Self {
        Self {
            inner: DartHandleWrap::from_raw(handle),
            drop_port: None,
        }
    }

    pub fn into_raw(&mut self) -> Dart_PersistentHandle {
        self.inner.0.take().unwrap()
    }

    pub fn unwrap(self) -> DartHandleWrap {
        self.inner
    }

    pub fn channel(&self) -> Channel {
        Channel::new(self.drop_port.unwrap())
    }
}
