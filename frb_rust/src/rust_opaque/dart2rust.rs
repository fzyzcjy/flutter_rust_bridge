use super::RustOpaque;
use crate::generalized_arc::base_arc::BaseArc;
use crate::rust_opaque::codec::moi::MoiRustOpaqueCodec;
use crate::rust_opaque::codec::nom::NomRustOpaqueCodec;
use crate::rust_opaque::codec::BaseRustOpaqueCodec;

/// # Safety
///
/// This should never be called manually.
unsafe fn decode_rust_opaque_nom<T>(ptr: usize) -> RustOpaque<T, NomRustOpaqueCodec> {
    assert!(ptr != 0);
    RustOpaque {
        arc: NomRustOpaqueCodec::Arc::from_raw(ptr),
    }
}

// This does not have `unsafe` keyword, thus is a separate function
fn decode_rust_opaque_moi<T>(ptr: usize) -> RustOpaque<T, MoiRustOpaqueCodec> {
    RustOpaque {
        arc: MoiRustOpaqueCodec::Arc::from_raw(ptr),
    }
}
