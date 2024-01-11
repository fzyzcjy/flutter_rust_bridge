use super::BaseRustOpaqueCodec;
use crate::generalized_arc::map_based_arc::{MapBasedArc, MapBasedArcValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec;

impl BaseRustOpaqueCodec for MoiRustOpaqueCodec {
    type Arc<T: ?Sized + MapBasedArcValue> = MapBasedArc<T>;
}
