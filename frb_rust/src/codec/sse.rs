use super::BaseCodec;
use crate::for_generated::vec_from_leak_ptr;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::{DartAbi, WireSyncReturnSse};
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::Rust2DartMessageSse;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SseCodec;

impl BaseCodec for SseCodec {
    type Rust2DartMessage = Rust2DartMessageSse;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> Vec<u8> {
        todo!()
    }
}

pub struct Rust2DartMessageSse(Vec<u8>);

impl Rust2DartMessageTrait for Rust2DartMessageSse {
    type InnerType = Vec<u8>;
    type WireSyncType = WireSyncReturnSse;

    fn new(inner: Self::InnerType) -> Self {
        Self(inner)
    }

    fn simplest() -> Self {
        Self(vec![])
    }

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self {
        let WireSyncReturnSseStruct { ptr, len } = raw;
        Self(vec_from_leak_ptr(ptr, len))
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncType {
        #[cfg(not(wasm))]
        {
            let (ptr, len) = into_leak_vec_ptr(self.0);
            return WireSyncReturnSseStruct { ptr, len };
        }

        #[cfg(wasm)]
        return self.0;
    }
}

// TODO maybe move
pub struct SseDeserializer {
    // Only to be used for generated code
    pub cursor: Cursor<Vec<u8>>,
    data_len: i32,
}

impl SseDeserializer {
    pub unsafe fn from_wire(ptr: *mut u8, rust_vec_len: i32, data_len: i32) -> Self {
        let vec = vec_from_leak_ptr(ptr, rust_vec_len);
        Self {
            cursor: Cursor::new(vec),
            data_len,
        }
    }

    pub fn end(self) {
        assert_eq!(self.data_len as u64, self.cursor.position());
    }
}

pub struct SseSerializer {
    pub cursor: Cursor<Vec<u8>>,
}

impl SseSerializer {
    pub fn new() -> Self {
        Self {
            cursor: Cursor::new(vec![]),
        }
    }
}
