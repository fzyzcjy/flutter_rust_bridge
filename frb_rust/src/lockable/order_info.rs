use crate::lockable::order::LockableOrder;

pub struct LockableOrderInfo {
    pub index: usize,
    pub mutable: bool,
    pub object_order: LockableOrder,
}
