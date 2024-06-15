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
