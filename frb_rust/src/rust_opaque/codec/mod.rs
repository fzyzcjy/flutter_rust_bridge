use crate::generalized_arc::base_arc::BaseArc;

pub(crate) mod moi;
pub(crate) mod nom;

pub trait BaseRustOpaqueCodec: Clone + Copy + Send + Sync {
    type ArcValue;
    type Arc<T: ?Sized + Self::ArcValue>: BaseArc<T>;
}
