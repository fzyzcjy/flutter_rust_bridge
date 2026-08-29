use crate::lockable::base::Lockable;
use crate::lockable::order::LockableOrder;

pub struct LockableOrderInfo {
    pub(crate) object_order: LockableOrder,
    pub(crate) index: usize,
    pub(crate) mutable: bool,
}

impl LockableOrderInfo {
    pub fn new(object: &impl Lockable, index: usize, mutable: bool) -> Self {
        Self {
            object_order: object.lockable_order(),
            index,
            mutable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::for_generated::Lockable;
    use crate::RustAutoOpaqueNom;

    #[test]
    /// Records the source order, argument index, and mutability.
    fn records_lockable_metadata() {
        let opaque = RustAutoOpaqueNom::new(42);
        let info = LockableOrderInfo::new(&opaque.0, 3, true);

        assert!(info.object_order == opaque.0.lockable_order());
        assert_eq!(info.index, 3);
        assert!(info.mutable);
    }
}
