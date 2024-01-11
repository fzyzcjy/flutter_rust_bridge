use super::BaseRustOpaqueCodec;
use crate::generalized_arc::std_arc::StdArc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NomRustOpaqueCodec;

impl BaseRustOpaqueCodec for NomRustOpaqueCodec {
    type Arc<T: ?Sized> = StdArc<T>;
}
