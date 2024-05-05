use crate::for_generated::StdArc;
use crate::rust_async::RwLock;
use crate::rust_auto_opaque::order::RustAutoOpaqueOrder;
use crate::rust_opaque::RustOpaqueBase;

pub(crate) mod dart2rust;
mod order;
pub(crate) mod inner;

pub type RustAutoOpaqueBase<T, A> = RustOpaqueBase<inner::RustAutoOpaqueInner<T>, A>;

/// Please refer to `RustAutoOpaque` for doc.
pub type RustAutoOpaqueNom<T> = RustAutoOpaqueBase<T, StdArc<inner::RustAutoOpaqueInner<T>>>;

#[doc(hidden)]
#[macro_export]
macro_rules! frb_generated_rust_auto_opaque_def {
    (default_rust_auto_opaque = $default_rust_auto_opaque:ident) => {
        use $crate::RustAutoOpaqueNom;

        /// Please refer to `RustAutoOpaque` for doc.
        pub type RustAutoOpaqueMoi<T> = $crate::for_generated::RustAutoOpaqueBase<
            T,
            MoiArc<$crate::for_generated::RustAutoOpaqueInner<T>>,
        >;

        /// Usually this is unneeded, and just write down arbitrary types.
        /// However, when you need arbitrary types at places that are not supported yet,
        /// use `RustOpaqueOpaque<YourArbitraryType>`.
        pub type RustAutoOpaque<T> = $default_rust_auto_opaque<T>;
    };
}
