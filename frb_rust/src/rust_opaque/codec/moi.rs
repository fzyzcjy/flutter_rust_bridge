use super::BaseRustOpaqueCodec;
use crate::generalized_arc::map_based_arc::MapBasedArc;
use crate::generalized_arc::std_arc::StdArc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec;

impl BaseRustOpaqueCodec for MoiRustOpaqueCodec {
    // TODO
    type Arc<T: ?Sized> = MapBasedArc<T>;
}
