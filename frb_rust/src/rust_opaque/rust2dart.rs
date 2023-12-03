use super::definition::{DartSafe, RustOpaque};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;
use std::{mem, ops};

impl<T: DartSafe> From<RustOpaque<T>> for DartAbi {
    fn from(value: RustOpaque<T>) -> Self {
        let ptr = if let Some(ptr) = value.ptr {
            Arc::into_raw(ptr)
        } else {
            std::ptr::null()
        };
        let size = mem::size_of::<T>();

        vec![ptr.into_dart(), size.into_dart()].into_dart()
    }
}
