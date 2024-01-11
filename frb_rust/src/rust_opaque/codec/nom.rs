use super::BaseRustOpaqueCodec;
use crate::generalized_arc::std_arc::StdArc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NomRustOpaqueCodec<T: ?Sized + 'static>;

impl<T: ?Sized + 'static> BaseRustOpaqueCodec<T> for NomRustOpaqueCodec<T> {
    type Arc = StdArc<T>;
}
