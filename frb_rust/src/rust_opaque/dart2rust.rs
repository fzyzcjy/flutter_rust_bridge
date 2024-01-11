use super::RustOpaque;
use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_opaque::codec::BaseRustOpaqueCodec;

macro_rules! impl_functions {
    ($postfix:ident) => {
        paste::paste! {
            /// # Safety
            ///
            /// This function should never be called manually.
            /// Retrieving an opaque pointer from Dart is an implementation detail, so this
            /// function is not guaranteed to be API-stable.
            #[cfg(not(wasm))]
            pub unsafe fn [<cst_decode_rust_opaque_ $postfix>]<T, C: BaseRustOpaqueCodec>(
                ptr: usize,
            ) -> RustOpaque<T, C> {
                [<decode_rust_opaque_innerr_ $postfix>](ptr as _)
            }

            /// # Safety
            ///
            /// This should never be called manually.
            #[cfg(wasm)]
            pub unsafe fn [<cst_decode_rust_opaque_ $postfix>]<T, C: BaseRustOpaqueCodec>(
                raw: wasm_bindgen::JsValue,
            ) -> RustOpaque<T, C> {
                #[cfg(target_pointer_width = "64")]
                {
                    compile_error!("64-bit pointers are not supported.");
                }

                [<decode_rust_opaque_innerr_ $postfix>]((raw.as_f64().unwrap() as usize) as _)
            }

            /// # Safety
            ///
            /// This should never be called manually.
            pub unsafe fn [<sse_decode_rust_opaque_ $postfix>]<T, C: BaseRustOpaqueCodec>(
                ptr: usize,
            ) -> RustOpaque<T, C> {
                [<decode_rust_opaque_innerr_ $postfix>](ptr)
            }

            /// # Safety
            ///
            /// This should never be called manually.
            unsafe fn [<decode_rust_opaque_innerr_ $postfix>]<T, C: BaseRustOpaqueCodec>(
                ptr: usize,
            ) -> RustOpaque<T, C> {
                assert!(ptr != 0);
                RustOpaque {
                    arc: C::Arc::from_raw(ptr),
                }
            }
        }
    };
}

impl_functions!(nom);
impl_functions!(moi);
