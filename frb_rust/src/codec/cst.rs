use super::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnCstSrc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CstCodec;

impl BaseCodec for CstCodec {
    type WireSyncReturn = (); // should not be used
    type WireSyncReturnSrc = WireSyncReturnCstSrc;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi {
        unreachable!()
    }
}
