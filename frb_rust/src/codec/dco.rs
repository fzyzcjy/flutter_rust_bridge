use super::BaseCodec;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcoCodec;

impl BaseCodec for DcoCodec {
    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi {
        if result_code == Rust2DartAction::CloseStream {
            vec![result_code.into_dart()].into_dart()
        } else {
            vec![result_code.into_dart(), data.into_dart()].into_dart()
        }
    }
}
