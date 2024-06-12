use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirCustomSerDes {
    pub inner_type: Box<MirType>,
    pub rust_api_type: Box<MirType>,
    pub dart_api_type: String,
    pub dart2rust: MirCustomSerDesHalf,
    pub rust2dart: MirCustomSerDesHalf,
}

pub struct MirCustomSerDesHalf {
    pub dart_code: String,
    pub rust_function: NamespacedName,
}
}
