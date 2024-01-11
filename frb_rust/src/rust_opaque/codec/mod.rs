use crate::codec::Rust2DartMessageTrait;

pub(crate) mod moi;
pub(crate) mod nom;

pub(crate) trait BaseRustOpaqueCodec: Clone + Copy + Send + Sync {
    type Arc<T: ?Sized>;
}
