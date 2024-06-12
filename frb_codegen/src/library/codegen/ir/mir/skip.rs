use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirSkip {
    pub name: NamespacedName,
    pub reason: MirSkipReason,
}

#[derive(Copy, PartialOrd, Ord)]
pub(crate) enum MirSkipReason {
    IgnoredFunctionNotPub,
    IgnoredFunctionGeneric,
    IgnoredTypeNotUsedByPub,
    IgnoredByAttribute,
    IgnoredByType,
    IgnoredMisc,
    IgnoredSilently,
    Err,
}
}

impl MirSkipReason {
    pub(crate) fn explanation_prefix(&self) -> Option<String> {
        Some(match self {
            Self::IgnoredFunctionNotPub => {
                "These functions are ignored because they are not marked as `pub`".to_owned()
            }
            Self::IgnoredFunctionGeneric => {
                "These functions are ignored because they have generic arguments".to_owned()
            }
            Self::IgnoredTypeNotUsedByPub => {
                "These types are ignored because they are not used by any `pub` functions".to_owned()
            }
            Self::IgnoredSilently => return None,
            Self::Err => {
                "These functions have error during generation (see debug logs or enable `stop_on_error: true` for more details)".to_owned()
            }
            _ => format!("These functions are ignored (reason: {:?})", self)
        })
    }
}
