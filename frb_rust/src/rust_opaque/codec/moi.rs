use super::BaseRustOpaqueCodec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoiRustOpaqueCodec;

impl BaseRustOpaqueCodec for MoiRustOpaqueCodec {
    // TODO
    type Arc<T: ?Sized> = std::sync::Arc<T>;
}
