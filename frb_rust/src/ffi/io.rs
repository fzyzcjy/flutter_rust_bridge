use crate::dart_api::Dart_DeletePersistentHandle_DL_Trampolined;
use crate::dart_api::Dart_NewPersistentHandle_DL_Trampolined;

pub use super::DartAbi;
pub use super::MessagePort;
pub use allo_isolate::*;
use dart_sys::Dart_Handle;
use dart_sys::Dart_PersistentHandle;

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


struct DartOpaque(Dart_PersistentHandle);

impl DartOpaque {
    pub fn new(handle: Dart_Handle) -> Self {
        DartOpaque(unsafe {Dart_NewPersistentHandle_DL_Trampolined(handle)})
    }
}

impl Drop for DartOpaque {
    fn drop(&mut self) {
        unsafe{Dart_DeletePersistentHandle_DL_Trampolined(self.0)}
    }
} 