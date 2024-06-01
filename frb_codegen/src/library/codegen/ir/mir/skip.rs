use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirSkip {
    pub name: NamespacedName,
    pub inner: MirSkipInner,
}

pub(crate) enum MirSkipInner {
    Ignored,
    Err(String),
}
}
