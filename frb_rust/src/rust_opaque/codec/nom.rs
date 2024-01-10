use super::BaseRustOpaqueCodec;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NomRustOpaqueCodec;

impl BaseRustOpaqueCodec for NomRustOpaqueCodec {
    type Data<T: ?Sized> = NomRustOpaqueData<T>;
}

pub(crate) struct NomRustOpaqueData<T: ?Sized> {
    arc: Arc<T>,
}
