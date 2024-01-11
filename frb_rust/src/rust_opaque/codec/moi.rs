use super::BaseRustOpaqueCodec;
use crate::generalized_arc::map_based_arc::MapBasedArc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec;

impl BaseRustOpaqueCodec for MoiRustOpaqueCodec {
    type Arc<T: ?Sized> = MapBasedArc<T>;
}
