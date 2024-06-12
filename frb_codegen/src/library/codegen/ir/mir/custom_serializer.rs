use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirCustomSerializer {
    pub inner_type: Box<MirType>,
    pub rust_api_type: Box<MirType>,
    pub dart_api_type: String,
    pub dart_encode: String,
    pub dart_decode: String,
    pub rust_encode_function: NamespacedName,
    pub rust_decode_function: NamespacedName,
}
}
