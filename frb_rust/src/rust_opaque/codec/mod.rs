use crate::generalized_arc::base_arc::BaseArc;

pub(crate) mod moi;
pub(crate) mod nom;

pub(crate) trait BaseRustOpaqueCodec: Clone + Copy + Send + Sync {
    type Arc<T: ?Sized>: BaseArc<T>;
}
