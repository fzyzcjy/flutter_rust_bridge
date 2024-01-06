use crate::RustOpaque;
use anyhow::{Context, Result};
use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

impl<T> RustOpaque<RwLock<T>> {
    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    pub fn rust_auto_opaque_decode_sync_owned(self) -> Result<T> {
        // frb-coverage:ignore-end
        Ok(self.into_inner()
            .context("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")?
            .into_inner())
    }

    pub fn rust_auto_opaque_decode_sync_ref(&self) -> Result<RwLockReadGuard<'_, T>> {
        Ok(self.blocking_read())
    }

    pub fn rust_auto_opaque_decode_sync_ref_mut(&self) -> Result<RwLockWriteGuard<'_, T>> {
        Ok(self.blocking_write())
    }
}

pub fn rust_auto_opaque_encode<T>(value: T) -> RustOpaque<RwLock<T>> {
    RustOpaque::new(RwLock::new(value))
}
