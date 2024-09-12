use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
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

impl MirCustomSerDes {
    pub(crate) fn cleared_rust_api_type(&self) -> String {
        if let MirType::RustAutoOpaqueImplicit(ty) = &*self.rust_api_type {
            ty.raw.string.with_original_lifetime().to_owned()
        } else {
            self.rust_api_type.rust_api_type()
        }
    }
}
