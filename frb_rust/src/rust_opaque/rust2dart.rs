use super::{DartSafe, RustOpaque};
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::sync::Arc;
use std::{mem, ops};

impl<T: DartSafe> From<RustOpaque<T>> for DartAbi {
    fn from(value: RustOpaque<T>) -> Self {
        let ptr = Arc::into_raw(value.arc);
        let size = mem::size_of::<T>();
        vec![ptr.into_dart(), size.into_dart()].into_dart()
    }
}
