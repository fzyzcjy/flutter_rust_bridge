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
