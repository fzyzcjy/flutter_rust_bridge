use super::user_code::*;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

struct WithOwnerGuard {
    // NOTE order of fields!
    value: RwLockReadGuard<'static, One>,
    owners: Vec<Box<dyn Any>>,
}

fn new(
    owner: One,
    guard_builder: impl for<'this> FnOnce(&'this One) -> RwLockReadGuard<'this, One>,
) -> WithOwnerGuard {
    TODO
}
