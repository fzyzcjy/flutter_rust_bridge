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

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::DartHandlerPortAction;
    use crate::generalized_isolate::IntoDart;
    use allo_isolate::ffi::DartCObjectType;

    /// Maps each Dart handler action to its stable integer discriminant.
    #[test]
    fn test_actions_encode_stable_integer_values() {
        let drop_action = DartHandlerPortAction::DartOpaqueDrop.into_dart();
        let invoke_action = DartHandlerPortAction::DartFnInvoke.into_dart();

        assert_eq!(drop_action.ty, DartCObjectType::DartInt32);
        assert_eq!(invoke_action.ty, DartCObjectType::DartInt32);
        assert_eq!(unsafe { drop_action.value.as_int32 }, 0);
        assert_eq!(unsafe { invoke_action.value.as_int32 }, 1);
    }
}
