use crate::RustOpaque;
use anyhow::{anyhow, Context, Result};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

impl<T> RustOpaque<RwLock<T>> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    pub fn rust_auto_opaque_decode_owned(self) -> Result<T> {
        // frb-coverage:ignore-end
        self.into_inner()
            .context("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")?
            .into_inner().map_err(|_| anyhow!("lock is poisoned"))
    }

    pub fn rust_auto_opaque_decode_ref(&self) -> Result<RwLockReadGuard<'_, T>> {
        self.read().map_err(|_| anyhow!("lock is poisioned"))
    }

    pub fn rust_auto_opaque_decode_ref_mut(&self) -> Result<RwLockWriteGuard<'_, T>> {
        self.write().map_err(|_| anyhow!("lock is poisioned"))
    }
}

pub fn rust_auto_opaque_encode<T>(value: T) -> RustOpaque<RwLock<T>> {
    RustOpaque::new(RwLock::new(value))
}
