use crate::for_generated::StdArc;
use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_opaque::RustOpaqueBase;

mod api;
pub(crate) mod dart2rust_explicit;
pub(crate) mod dart2rust_implicit;
pub(crate) mod inner;
pub(crate) mod rust2dart_common;
pub(crate) mod rust2dart_explicit;

#[derive(Clone)]
pub struct RustAutoOpaqueBase<T: 'static, A: BaseArc<inner::RustAutoOpaqueInner<T>>>(
    pub(crate) RustOpaqueBase<inner::RustAutoOpaqueInner<T>, A>,
);

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
