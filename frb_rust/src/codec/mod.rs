use crate::platform_types::DartAbi;
use cst::CstCodec;
use dco::DcoCodec;
use enum_dispatch::enum_dispatch;
use sse::SseCodec;
use std::any::Any;

pub(crate) mod cst;
pub(crate) mod dco;
pub(crate) mod sse;

#[enum_dispatch]
enum Codec {
    CstCodec,
    DcoCodec,
    SseCodec,
}

#[enum_dispatch(Codec)]
pub trait CodecTrait {
    type Message: Rust2DartMessageTrait;

    fn encode_panic(error: &Box<dyn Any + Send>) -> Self::Message;

    fn encode_close_stream() -> Self::Message;
}

/// An encoded message
pub trait Rust2DartMessageTrait {
    type WireSyncType;

    fn simplest() -> Self;

    fn into_dart_abi(self) -> DartAbi;

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncType) -> Self;

    fn into_raw_wire_sync(self) -> Self::WireSyncType;
}
