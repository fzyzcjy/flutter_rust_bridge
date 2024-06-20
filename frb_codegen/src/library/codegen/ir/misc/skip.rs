use crate::utils::namespace::Namespace;

crate::mir! {
pub struct IrSkip {
    pub namespace: Namespace,
    pub reason: IrSkipReason,
}

#[derive(Copy, PartialOrd, Ord)]
pub(crate) enum IrSkipReason {
    IgnoreBecauseFunctionNotPub,
    IgnoreBecauseFunctionGeneric,
    IgnoreBecauseTypeNotUsedByPub,
    IgnoreBecauseExplicitAttribute,
    IgnoreBecauseType,
    IgnoreBecauseParseMethodOwnerTy,
    IgnoreBecauseParseOwnerCannotFindTrait,
    IgnoreBecauseNotAllowedOwner,
    IgnoreBecauseOwnerTyShouldIgnore,
    IgnoreBecauseSelfTypeNotAllowed,
    IgnoreSilently,
    Err,
}
}

impl IrSkipReason {
    pub(crate) fn explanation_prefix(&self) -> Option<String> {
        Some(match self {
            Self::IgnoreBecauseFunctionNotPub => {
                "These functions are ignored because they are not marked as `pub`".to_owned()
            }
            Self::IgnoreBecauseFunctionGeneric => {
                "These functions are ignored because they have generic arguments".to_owned()
            }
            Self::IgnoreBecauseTypeNotUsedByPub => {
                "These types are ignored because they are not used by any `pub` functions".to_owned()
            }
            Self::IgnoreSilently => return None,
            Self::Err => {
                "These functions have error during generation (see debug logs or enable `stop_on_error: true` for more details)".to_owned()
            }
            _ => format!("These functions are ignored (category: {:?})", self)
        })
    }
}
