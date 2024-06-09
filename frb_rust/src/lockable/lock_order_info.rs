use crate::lockable::lockable_order::LockableOrder;

pub struct LockableOrderInfo {
    index: usize,
    mutable: bool,
    object_order: LockableOrder,
}
