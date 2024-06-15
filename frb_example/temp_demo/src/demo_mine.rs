use super::user_code::*;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

struct WithOwner<T> {
    // NOTE order of fields!
    value: T,
    owners: Vec<Box<dyn Any>>,
}

fn new(
    owner: Arc<RwLock<One>>,
    guard_builder: impl for<'this> FnOnce(&'this Arc<RwLock<One>>) -> RwLockReadGuard<'this, One>,
) {
    TODO
}
