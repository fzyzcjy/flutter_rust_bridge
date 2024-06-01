use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirSkip {
    pub name: NamespacedName,
    pub reason: MirSkipReason,
}

#[derive(Copy, PartialOrd, Ord)]
pub(crate) enum MirSkipReason {
    Ignored,
    Err,
}
}

impl MirSkipReason {
    pub(crate) fn explanation_prefix(&self) -> &'static str {
        match self {
            Self::Ignored => "These functions are ignored",
            Self::Err => "These functions have error during generation (see debug logs for more details)",
        }
    }
}
