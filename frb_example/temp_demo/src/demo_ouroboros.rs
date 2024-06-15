use super::user_code::*;
use ouroboros::self_referencing;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockReadGuard};

#[self_referencing]
pub struct OneAndGuard {
    one: Arc<RwLock<One>>,
    #[borrows(one)]
    #[covariant]
    guard: RwLockReadGuard<'this, One>,
}

pub fn main() -> anyhow::Result<()> {
    Ok(())
}
