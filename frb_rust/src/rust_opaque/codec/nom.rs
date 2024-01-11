use super::BaseRustOpaqueCodec;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NomRustOpaqueCodec;

impl BaseRustOpaqueCodec for NomRustOpaqueCodec {
    type Arc<T: ?Sized> = std::sync::Arc<T>;
}
