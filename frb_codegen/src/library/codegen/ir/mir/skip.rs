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
