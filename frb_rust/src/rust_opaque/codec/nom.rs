use super::BaseRustOpaqueCodec;
use crate::generalized_arc::std_arc::StdArc;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NomRustOpaqueCodec<T: ?Sized + 'static>(PhantomData<T>);

impl<T: ?Sized + 'static> BaseRustOpaqueCodec<T> for NomRustOpaqueCodec<T> {
    type Arc = StdArc<T>;
}
