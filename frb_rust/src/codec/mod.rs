use crate::platform_types::DartAbi;
use backtrace::Backtrace;
use std::any::Any;

pub(crate) mod cst;
pub(crate) mod dco;
pub(crate) mod sse;

pub trait BaseCodec: Clone + Copy + Send {
    type Message: Rust2DartMessageTrait;

    fn encode_panic(error: &Box<dyn Any + Send>, backtrace: &Option<Backtrace>) -> Self::Message;

    fn encode_close_stream() -> Self::Message;
}

/// An encoded message
pub trait Rust2DartMessageTrait {
    type WireSyncRust2DartType;

    fn simplest() -> Self;

    fn into_dart_abi(self) -> DartAbi;

    unsafe fn from_raw_wire_sync(raw: Self::WireSyncRust2DartType) -> Self;

    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType;
}
