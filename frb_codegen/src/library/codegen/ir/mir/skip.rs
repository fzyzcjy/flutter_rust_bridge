use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirSkip {
    pub name: NamespacedName,
    pub reason: MirSkipReason,
}

#[derive(Copy, PartialOrd, Ord)]
pub(crate) enum MirSkipReason {
    IgnoredNotPub,
    IgnoredMisc,
    Err,
}
}

impl MirSkipReason {
    pub(crate) fn explanation_prefix(&self) -> &'static str {
        match self {
            Self::IgnoredNotPub => "These functions are ignored because they are not marked as `pub`",
            Self::IgnoredMisc => "These functions are ignored",
            Self::Err => "These functions have error during generation (see debug logs for more details)",
        }
    }
}
