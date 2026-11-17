pub mod api;
mod frb_generated;

/// Retain this package's bridge exports when linked into another native library.
pub fn link_bridge() {
    std::hint::black_box([
        frb_generated::frb_get_rust_content_hash as *const (),
        frb_generated::frb_pde_ffi_dispatcher_primary as *const (),
        frb_generated::frb_pde_ffi_dispatcher_sync as *const (),
        frb_generated::frb_dart_fn_deliver_output as *const (),
    ]);
}
