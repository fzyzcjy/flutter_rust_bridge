use super::BaseRustOpaqueCodec;
use crate::generalized_arc::map_based_arc::MapBasedArc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec<T: ?Sized + 'static>;

impl<T: ?Sized + 'static> BaseRustOpaqueCodec<T> for MoiRustOpaqueCodec<T> {
    type Arc = MapBasedArc<T>;
}
