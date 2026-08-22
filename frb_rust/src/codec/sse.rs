use super::{BaseCodec, Rust2DartMessageTrait};
use crate::generalized_isolate::IntoDart;
use crate::handler::error::error_to_string;
use crate::platform_types::{DartAbi, PlatformGeneralizedUint8ListPtr, WireSyncRust2DartSse};
use crate::rust2dart::action::Rust2DartAction;
use byteorder::NativeEndian;
use byteorder::WriteBytesExt;
use std::any::Any;
use std::backtrace::Backtrace;
use std::io::Cursor;

/// Codec that does a simple serialization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SseCodec;

impl BaseCodec for SseCodec {
    type Message = Rust2DartMessageSse;

    fn encode_panic(error: &Box<dyn Any + Send>, backtrace: &Option<Backtrace>) -> Self::Message {
        let msg = error_to_string(error, backtrace);
        Self::encode(Rust2DartAction::Panic, |serializer| {
            // NOTE roughly copied from the auto-generated serialization of String
            let bytes = msg.into_bytes();
            (serializer.cursor)
                .write_i32::<NativeEndian>(bytes.len() as _)
                .unwrap();
            for byte in bytes {
                serializer.cursor.write_u8(byte).unwrap();
            }
        })
    }

    fn encode_close_stream() -> Self::Message {
        Self::encode(Rust2DartAction::CloseStream, |_| {})
    }
}

impl SseCodec {
    // Only to be used by generated code, thus hidden in doc
    #[doc(hidden)]
    pub fn encode(
        result_code: Rust2DartAction,
        data_fn: impl FnOnce(&mut SseSerializer),
    ) -> Rust2DartMessageSse {
        let mut serializer = SseSerializer::new();
        (serializer.cursor).write_u8(result_code as _).unwrap();
        data_fn(&mut serializer);
        Rust2DartMessageSse(serializer.cursor.into_inner())
    }
}

pub struct Rust2DartMessageSse(Vec<u8>);

impl Rust2DartMessageTrait for Rust2DartMessageSse {
    type WireSyncRust2DartType = WireSyncRust2DartSse;

    fn simplest() -> Self {
        Self(vec![])
    }

    fn into_dart_abi(self) -> DartAbi {
        self.0.into_dart()
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncRust2DartType) -> Self {
        #[cfg(not(target_family = "wasm"))]
        {
            let WireSyncRust2DartSse { ptr, len } = raw;
            Self(crate::for_generated::vec_from_leak_ptr(ptr, len))
        }

        #[cfg(target_family = "wasm")]
        Self(js_sys::Uint8Array::new(&raw).to_vec())
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
        #[cfg(not(target_family = "wasm"))]
        {
            let (ptr, len) = crate::for_generated::into_leak_vec_ptr(self.0);
            WireSyncRust2DartSse { ptr, len }
        }

        #[cfg(target_family = "wasm")]
        return <js_sys::Uint8Array>::from(self.0.as_slice()).into();
    }
}

#[derive(Debug)]
pub struct Dart2RustMessageSse {
    vec: Vec<u8>,
    data_len: i32,
}

impl Dart2RustMessageSse {
    /// # Safety
    ///
    /// This should never be called manually.
    #[allow(unused)]
    pub unsafe fn from_wire(
        ptr: PlatformGeneralizedUint8ListPtr,
        rust_vec_len: i32,
        data_len: i32,
    ) -> Self {
        #[cfg(not(target_family = "wasm"))]
        let vec = crate::for_generated::vec_from_leak_ptr(ptr, rust_vec_len);
        #[cfg(target_family = "wasm")]
        let vec = js_sys::Uint8Array::new(&ptr).to_vec();

        Self { vec, data_len }
    }
}

// TODO maybe move
pub struct SseDeserializer {
    // Only to be used for generated code
    pub cursor: Cursor<Vec<u8>>,
    data_len: i32,
}

impl SseDeserializer {
    pub fn new(message: Dart2RustMessageSse) -> Self {
        Self {
            cursor: Cursor::new(message.vec),
            data_len: message.data_len,
        }
    }

    pub fn end(self) {
        assert_eq!(self.data_len as u64, self.cursor.position());
    }
}

pub struct SseSerializer {
    pub cursor: Cursor<Vec<u8>>,
}

impl Default for SseSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl SseSerializer {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(vec![]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Dart2RustMessageSse, Rust2DartMessageSse, SseCodec, SseDeserializer};
    use crate::codec::{BaseCodec, Rust2DartMessageTrait};
    use crate::for_generated::{Rust2DartAction, SseSerializer};

    /// Represents the smallest SSE payload with no bytes.
    #[test]
    fn test_simplest() {
        assert_eq!(Rust2DartMessageSse::simplest().0, vec![]);
    }

    /// Builds an empty serializer through the default implementation.
    #[test]
    fn test_serializer_default() {
        assert_eq!(SseSerializer::default().cursor.into_inner(), vec![]);
    }

    /// Prefixes serialized data with the selected Rust-to-Dart action.
    #[test]
    fn test_encode_writes_action_and_payload() {
        let message = SseCodec::encode(Rust2DartAction::Error, |serializer| {
            serializer.cursor.get_mut().extend_from_slice(&[4, 5]);
        });

        assert_eq!(message.0, vec![Rust2DartAction::Error as u8, 4, 5]);
    }

    /// Encodes a stream close without an additional payload.
    #[test]
    fn test_close_stream_writes_only_close_action() {
        assert_eq!(
            SseCodec::encode_close_stream().0,
            vec![Rust2DartAction::CloseStream as u8]
        );
    }

    #[cfg(not(target_family = "wasm"))]
    /// Preserves SSE bytes through the synchronous ownership transfer.
    #[test]
    fn test_wire_sync_round_trip_preserves_bytes() {
        let raw = Rust2DartMessageSse(vec![1, 2, 3]).into_raw_wire_sync();
        let message = unsafe { Rust2DartMessageSse::from_raw_wire_sync(raw) };

        assert_eq!(message.0, vec![1, 2, 3]);
    }

    #[cfg(not(target_family = "wasm"))]
    /// Accepts a Dart-to-Rust payload consumed exactly to its declared length.
    #[test]
    fn test_deserializer_end_accepts_exact_consumption() {
        let (ptr, len) = crate::for_generated::into_leak_vec_ptr(vec![8, 9]);
        let message = unsafe { Dart2RustMessageSse::from_wire(ptr, len, len) };
        let mut deserializer = SseDeserializer::new(message);
        deserializer.cursor.set_position(len as u64);

        deserializer.end();
    }

    #[cfg(not(target_family = "wasm"))]
    /// Rejects a Dart-to-Rust payload that leaves declared bytes unread.
    #[test]
    #[should_panic]
    fn test_deserializer_end_rejects_incomplete_consumption() {
        let (ptr, len) = crate::for_generated::into_leak_vec_ptr(vec![8, 9]);
        let message = unsafe { Dart2RustMessageSse::from_wire(ptr, len, len) };

        SseDeserializer::new(message).end();
    }
}
