use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirCustomSerDes {
    pub inner_type: Box<MirType>,
    pub rust_api_type: Box<MirType>,
    pub dart_api_type: String,
    pub direction: MirCustomSerDesDirection,
    pub dart_code: String,
    pub rust_function: NamespacedName,
}

#[derive(Copy)]
pub enum MirCustomSerDesDirection {
    Rust2Dart,
    Dart2Rust,
}
}
