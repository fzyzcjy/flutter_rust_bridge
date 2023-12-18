use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DartHandlerPortAction {
    DartOpaqueDrop = 0,
    DartFnInvoke = 1,
}

impl IntoDart for DartHandlerPortAction {
    fn into_dart(self) -> DartAbi {
        (self as i32).into_dart()
    }
}
