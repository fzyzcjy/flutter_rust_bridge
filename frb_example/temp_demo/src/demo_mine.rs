use super::user_code::*;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

struct WithOwner<T> {
    // NOTE order of fields!
    value: T,
    owners: Vec<Box<dyn Any>>,
}

fn new<O, T>(owner: O, guard_builder: impl for<'this> FnOnce(&'this O) -> T<'this>) {
    TODO
}
