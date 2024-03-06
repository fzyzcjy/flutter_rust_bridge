pub(crate) mod dart2rust;
pub(crate) mod rust2dart;
pub(crate) mod utils;

use crate::for_generated::{BaseArc, StdArc};
use std::marker::PhantomData;

/// Please refer to [RustOpaque] for doc.
#[repr(transparent)]
#[derive(Debug)]
pub struct RustOpaqueBase<T: ?Sized + 'static, A: BaseArc<T>> {
    arc: A,
    _phantom: PhantomData<T>,
}

/// Please refer to `RustOpaque` for doc.
///
/// For readers migrating from old versions: The new `RustOpaque` is at `crate::frb_generated::RustOpaque`.
pub type RustOpaqueNom<T> = RustOpaqueBase<T, StdArc<T>>;

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_opaque_def {
    (default_rust_opaque = $default_rust_opaque:ident) => {
        use $crate::for_generated::StdArc;
        use $crate::RustOpaqueNom;

        /// Please refer to `RustOpaque` for doc.
        pub type RustOpaqueMoi<T> = $crate::for_generated::RustOpaqueBase<T, MoiArc<T>>;

        /// A wrapper to support [arbitrary Rust types](https://cjycode.com/flutter_rust_bridge/guides/types/arbitrary).
        pub type RustOpaque<T> = $default_rust_opaque<T>;
    };
}

// https://github.com/fzyzcjy/flutter_rust_bridge/pull/1574
#[deprecated(note = "It is empty trait and can be directly deleted")]
pub trait DartSafe {}

#[allow(deprecated)]
impl<T> DartSafe for T {}
