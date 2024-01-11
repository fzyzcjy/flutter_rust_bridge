use super::BaseRustOpaqueCodec;
use crate::generalized_arc::map_based_arc::MapBasedArc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec;

impl<T: ?Sized + 'static> BaseRustOpaqueCodec<T> for MoiRustOpaqueCodec {
    type Arc = MapBasedArc<T>;
}
