use crate::platform_types::DartAbi;
use std::any::Any;
use std::panic::{RefUnwindSafe, UnwindSafe};

pub(crate) mod cst;
pub(crate) mod dco;
pub(crate) mod sse;

pub trait BaseCodec: Clone + Copy + UnwindSafe + RefUnwindSafe + Send {
    type Message: Rust2DartMessageTrait;

    fn encode_panic(error: &Box<dyn Any + Send>) -> Self::Message;

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
