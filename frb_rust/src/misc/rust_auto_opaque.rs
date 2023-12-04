use crate::{DartSafe, RustOpaque};
use anyhow::{Context, Result};

pub trait RustAutoOpaqueWire2Api<T> {
    fn rust_auto_opaque_wire2api(self) -> Result<T>;
}

impl<T: DartSafe> RustAutoOpaqueWire2Api<T> for RustOpaque<T> {
    fn rust_auto_opaque_wire2api(self) -> Result<T> {
        self.into_inner()
            .context("Cannot convert RustOpaque to inner value. This is probably because you are having more than one references to it.")
    }
}

impl<'a, T: ?Sized + DartSafe> RustAutoOpaqueWire2Api<&'a T> for &'a RustOpaque<T> {
    fn rust_auto_opaque_wire2api(self) -> Result<&'a T> {
        Ok(&self)
    }
}
