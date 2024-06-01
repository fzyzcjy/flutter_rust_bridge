use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirSkip {
    pub name: NamespacedName,
    pub reason: MirSkipReason,
}

#[derive(Copy)]
pub(crate) enum MirSkipReason {
    Ignored,
    Err,
}
}
