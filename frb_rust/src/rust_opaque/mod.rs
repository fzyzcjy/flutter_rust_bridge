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
pub type RustOpaqueNom<T> = RustOpaqueBase<T, StdArc<T>>;
/// Please refer to `RustAutoOpaque` for doc.
pub type RustAutoOpaqueNom<T> = RustAutoOpaqueBase<T, StdArc<T>>;

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_opaque_def {
    (default_rust_opaque = $default_rust_opaque:ident) => {
        /// Please refer to `RustOpaque` for doc.
        pub type RustOpaqueMoi<T> = $crate::for_generated::RustOpaqueBase<T, MoiArc<T>>;
        /// Please refer to `RustAutoOpaque` for doc.
        pub type RustAutoOpaqueMoi<T> = $crate::for_generated::RustAutoOpaqueBase<T, MoiArc<T>>;

        /// A wrapper to support [arbitrary Rust types](https://cjycode.com/flutter_rust_bridge/guides/types/arbitrary).
        ///
        /// ## Naming the inner type
        ///
        /// When an `RustOpaque<T>` is transformed into a Dart type, T's string
        /// representation undergoes some transformations to become a valid Dart type:
        /// - Rust keywords (dyn, 'static, etc.) are automatically removed.
        /// - ASCII alphanumerics are kept, all other characters are ignored.
        ///
        /// ## Trait objects
        ///
        /// Trait objects can be put behind opaque pointers. For example, this declaration can
        /// be used across the FFI border:
        ///
        /// ```rust
        /// use flutter_rust_bridge::*;
        /// use std::fmt::Debug;
        /// use std::panic::{UnwindSafe, RefUnwindSafe};
        ///
        /// pub struct DebugWrapper(pub RustOpaque<Box<dyn Debug>>);
        ///
        /// // creating a DebugWrapper using the opaque_dyn macro
        /// let wrap = DebugWrapper(opaque_dyn!("foobar"));
        /// // it's possible to name it directly
        /// pub struct DebugWrapper2(pub RustOpaque<Box<dyn Debug + Send + Sync + UnwindSafe + RefUnwindSafe>>);
        /// ```
        pub type RustOpaque<T> = $default_rust_opaque<T>;

        /// Usually this is unneeded, and just write down arbitrary types.
        /// However, when you need arbitrary types at places that are not supported yet,
        /// use `RustOpaqueOpaque<YourArbitraryType>`.
        pub type RustAutoOpaque<T> = RustOpaque<$crate::for_generated::rust_async::RwLock<T>>;
    };
}

// https://github.com/fzyzcjy/flutter_rust_bridge/pull/1574
#[deprecated(note = "It is empty trait and can be directly deleted")]
pub trait DartSafe {}

#[allow(deprecated)]
impl<T> DartSafe for T {}
