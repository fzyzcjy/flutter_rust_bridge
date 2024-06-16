use super::user_code::*;
use std::any::Any;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

struct WithOwnerGuard {
    // NOTE order of fields MUST be correct, which affects drop order
    value: RwLockReadGuard<'static, One>,
    owners: Vec<Box<dyn Any>>,
}

fn new(
    owner: Arc<RwLock<One>>,
    builder: impl for<'this> FnOnce(&'this RwLock<One>) -> RwLockReadGuard<'this, One>,
) -> WithOwnerGuard {
    let owner_illegal_static_reference = unsafe { ::ouroboros::macro_help::change_lifetime(&*owner) };
    let owner2 = owner.clone();
    WithOwnerGuard {
        value: builder(owner_illegal_static_reference),
        owners: vec![Box::new(owner2)],
    }
}