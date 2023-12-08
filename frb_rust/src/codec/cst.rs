use super::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;
use crate::rust2dart::wire_sync_return_src::WireSyncReturnCstWrapper;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CstCodec;

impl BaseCodec for CstCodec {
    type WireSyncReturnWrapper = WireSyncReturnCstWrapper;

    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> () {
        unreachable!()
    }
}
