use crate::rust_opaque::codec::moi::MoiRustOpaqueCodec;
use crate::rust_opaque::codec::nom::NomRustOpaqueCodec;
use crate::rust_opaque::codec::BaseRustOpaqueCodec;
use std::sync::Arc;

/// # Safety
///
/// This should never be called manually.
pub unsafe fn rust_arc_increment_strong_count_nom<T>(raw: usize) {
    <NomRustOpaqueCodec as BaseRustOpaqueCodec>::Arc::<T>::increment_strong_count(raw);
}

/// # Safety
///
/// This should never be called manually.
pub unsafe fn rust_arc_decrement_strong_count_nom<T>(raw: usize) {
    <NomRustOpaqueCodec as BaseRustOpaqueCodec>::Arc::<T>::decrement_strong_count(raw);
}

// This is a separate function, because it is safe (not `unsafe`)
pub fn rust_arc_increment_strong_count_moi<T: 'static>(raw: usize) {
    <MoiRustOpaqueCodec as BaseRustOpaqueCodec>::Arc::<T>::increment_strong_count(raw);
}

pub fn rust_arc_decrement_strong_count_moi<T: 'static>(raw: usize) {
    <MoiRustOpaqueCodec as BaseRustOpaqueCodec>::Arc::<T>::decrement_strong_count(raw);
}
