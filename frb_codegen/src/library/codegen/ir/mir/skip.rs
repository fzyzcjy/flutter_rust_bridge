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
