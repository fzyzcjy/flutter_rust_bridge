use super::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SseCodec;

impl BaseCodec for SseCodec {
    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi {
        todo!()
    }
}

// TODO maybe move
pub struct SseDeserializer {}

impl SseDeserializer {
    pub unsafe fn from_wire(ptr: *const u8, rust_vec_len: i32, data_len: i32) -> Self {
        todo!()
    }
}

pub struct SseSerializer {}
