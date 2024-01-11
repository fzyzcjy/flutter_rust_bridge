pub(crate) mod moi;
pub(crate) mod nom;

pub trait BaseRustOpaqueCodec: Clone + Copy + Send + Sync {
    type Arc<T: ?Sized>;
}
