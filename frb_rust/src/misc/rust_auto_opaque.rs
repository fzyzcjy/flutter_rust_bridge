use crate::{DartSafe, RustOpaque};
use anyhow::{anyhow, Context, Result};
use std::ops::Deref;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

impl<T: DartSafe> RustOpaque<RwLock<T>> {
    pub fn rust_auto_opaque_wire2api_owned(self) -> Result<T> {
        self.into_inner()
            .context("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")?
            .into_inner().map_err(|_| anyhow!("lock is poisoned"))
    }

    pub fn rust_auto_opaque_wire2api_ref(&self) -> Result<RwLockReadGuard<'_, T>> {
        self.read().map_err(|_| anyhow!("lock is poisioned"))
    }

    pub fn rust_auto_opaque_wire2api_ref_mut(&self) -> Result<RwLockWriteGuard<'_, T>> {
        self.write().map_err(|_| anyhow!("lock is poisioned"))
    }
}

pub fn rust_auto_opaque_encode<T>(value: T) -> RustOpaque<RwLock<T>> {
    RustOpaque::new(RwLock::new(value))
}
