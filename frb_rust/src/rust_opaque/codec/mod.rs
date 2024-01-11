use crate::generalized_arc::base_arc::BaseArc;

pub(crate) mod moi;
pub(crate) mod nom;

pub trait BaseRustOpaqueCodec<T: ?Sized + 'static>: Clone + Copy + Send + Sync {
    type Arc: BaseArc<T>;
}
