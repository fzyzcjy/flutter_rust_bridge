use crate::RustOpaque;
use anyhow::{Context, Result};

pub trait RustAutoOpaqueWire2Api<T> {
    fn rust_auto_opaque_wire2api(self) -> Result<T>;
}

impl<T> RustAutoOpaqueWire2Api<T> for RustOpaque<T> {
    fn rust_auto_opaque_wire2api(self) -> Result<T> {
        self.into_inner()
            .context("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")
    }
}
