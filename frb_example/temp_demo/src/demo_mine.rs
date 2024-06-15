use super::user_code::*;
use ouroboros::macro_help::AliasableBox;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

struct SelfCell {
    owner: Arc<RwLock<One>>,
    dependent: AliasableBox<RwLockReadGuard<'static, One>>,
}

fn new(
    owner: Arc<RwLock<One>>,
    guard_builder: impl for<'this> FnOnce(&'this Arc<RwLock<One>>) -> RwLockReadGuard<'this, One>,
) {
    TODO
}
