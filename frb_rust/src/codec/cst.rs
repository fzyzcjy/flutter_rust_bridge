use super::{BaseCodec, Rust2DartMessageTrait};
use crate::platform_types::DartAbi;
use std::any::Any;
use std::backtrace::Backtrace;

// frb-coverage:ignore-start
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CstCodec;
// frb-coverage:ignore-end

impl BaseCodec for CstCodec {
    type Message = Rust2DartMessageCst;

    // frb-coverage:ignore-start
    fn encode_panic(_error: &Box<dyn Any + Send>, _backtrace: &Option<Backtrace>) -> Self::Message {
        unreachable!()
    }

    fn encode_close_stream() -> Self::Message {
        unreachable!()
    }
    // frb-coverage:ignore-end
}

pub struct Rust2DartMessageCst;

impl Rust2DartMessageTrait for Rust2DartMessageCst {
    type WireSyncRust2DartType = ();

    // frb-coverage:ignore-start
    fn simplest() -> Self {
        unreachable!()
    }

    fn into_dart_abi(self) -> DartAbi {
        unreachable!()
    }

    unsafe fn from_raw_wire_sync(_raw: Self::WireSyncRust2DartType) -> Self {
        unreachable!()
    }

    fn into_raw_wire_sync(self) -> Self::WireSyncRust2DartType {
        unreachable!()
    }
    // frb-coverage:ignore-end
}
