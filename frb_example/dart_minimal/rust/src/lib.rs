pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

/// Retain all bridge exports when this example is statically linked by another package.
#[cfg(not(target_family = "wasm"))]
pub fn link_bridge() {
    std::hint::black_box([
        frb_generated::frb_get_rust_content_hash as *const (),
        frb_generated::frb_pde_ffi_dispatcher_primary as *const (),
        frb_generated::frb_pde_ffi_dispatcher_sync as *const (),
        frb_generated::frb_dart_fn_deliver_output as *const (),
    ]);
}
