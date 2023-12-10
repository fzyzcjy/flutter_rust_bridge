use super::{BaseCodec, Rust2DartMessageTrait};
use crate::for_generated::into_leak_vec_ptr;
use crate::for_generated::vec_from_leak_ptr;
use crate::generalized_isolate::IntoDart;
use crate::handler::error::error_to_string;
use crate::platform_types::{DartAbi, PlatformGeneralizedUint8ListPtr, WireSyncRust2DartSse};
use crate::rust2dart::action::Rust2DartAction;
use byteorder::NativeEndian;
use byteorder::WriteBytesExt;
use std::any::Any;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SseCodec;

impl BaseCodec for SseCodec {
    type Message = Rust2DartMessageSse;

    fn encode_panic(error: &Box<dyn Any + Send>) -> Self::Message {
        let msg = error_to_string(error);
        Self::encode(Rust2DartAction::Panic, |serializer| {
            // NOTE roughly copied from the auto-generated serialization of String
            let bytes = msg.into_bytes();
            (serializer.cursor).write_i32::<NativeEndian>(bytes.len() as _);
            for byte in bytes {
                serializer.cursor.write_u8(byte);
            }
        })
    }

    fn encode_close_stream() -> Self::Message {
        Self::encode(Rust2DartAction::CloseStream, |_| {})
    }
}

impl SseCodec {
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
        #[cfg(not(wasm))]
        {
            let WireSyncRust2DartSse { ptr, len } = raw;
            Self(vec_from_leak_ptr(ptr, len))
        }

        #[cfg(wasm)]
        Self(js_sys::Uint8Array::new(&raw).to_vec())
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
        #[cfg(not(wasm))]
        {
            let (ptr, len) = into_leak_vec_ptr(self.0);
            WireSyncRust2DartSse { ptr, len }
        }

        #[cfg(wasm)]
        return <js_sys::Uint8Array>::from(self.0.as_slice()).into();
    }
}

pub struct Dart2RustMessageSse {
    vec: Vec<u8>,
    data_len: i32,
}

impl Dart2RustMessageSse {
    pub unsafe fn from_wire(
        ptr: PlatformGeneralizedUint8ListPtr,
        rust_vec_len: i32,
        data_len: i32,
    ) -> Self {
        #[cfg(not(wasm))]
        let vec = vec_from_leak_ptr(ptr, rust_vec_len);
        #[cfg(wasm)]
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
