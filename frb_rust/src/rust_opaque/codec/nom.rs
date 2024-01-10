use super::BaseRustOpaqueCodec;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NomRustOpaqueCodec;

impl BaseRustOpaqueCodec for NomRustOpaqueCodec {
    type Data<T> = NomRustOpaqueData<T>;
}

pub(crate) struct NomRustOpaqueData<T> {
    arc: Arc<T>,
}
