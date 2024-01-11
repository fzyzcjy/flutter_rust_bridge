use super::BaseRustOpaqueCodec;
use crate::generalized_arc::map_based_arc::MapBasedArc;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec<T: ?Sized + 'static>(PhantomData<T>);

impl<T: ?Sized + 'static> BaseRustOpaqueCodec<T> for MoiRustOpaqueCodec<T> {
    type Arc = MapBasedArc<T>;
}
