use crate::for_generated::StdArc;
use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_opaque::RustOpaqueBase;

mod api;
pub(crate) mod dart2rust_explicit;
pub(crate) mod dart2rust_implicit;
pub(crate) mod inner;
pub(crate) mod rust2dart_common;
pub(crate) mod rust2dart_explicit;
pub(crate) mod utils;

#[derive(Debug)]
pub struct RustAutoOpaqueBase<T: 'static, A: BaseArc<inner::RustAutoOpaqueInner<T>>>(
    pub(crate) RustOpaqueBase<inner::RustAutoOpaqueInner<T>, A>,
);

#[cfg(target_family = "wasm")]
pub(crate) fn web_throw_lock_error(action: &str, error: tokio::sync::TryLockError) -> ! {
    wasm_bindgen::throw_str(&format!(
        "cannot synchronously {action} rust opaque objects while it is locked on Web: {error:?}"
    ))
}

#[cfg(target_family = "wasm")]
thread_local! {
    static WEB_IS_DEDICATED_WORKER_CONTEXT: std::cell::Cell<Option<bool>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(target_family = "wasm")]
pub(crate) fn web_is_dedicated_worker_context() -> bool {
    use wasm_bindgen::JsCast;

    WEB_IS_DEDICATED_WORKER_CONTEXT.with(|cached| match cached.get() {
        Some(value) => value,
        None => {
            let value = js_sys::global()
                .dyn_ref::<web_sys::DedicatedWorkerGlobalScope>()
                .is_some();
            cached.set(Some(value));
            value
        }
    })
}

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
