use crate::codegen::ir::mir::direction::MirDirection;
use crate::codegen::ir::mir::ty::MirType;
use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirCustomSerDes {
    pub inner_type: Box<MirType>,
    pub rust_api_type: Box<MirType>,
    pub dart_api_type: String,
    pub direction: MirDirection,
    pub dart_code: String,
    pub rust_function: NamespacedName,
}

}

impl MirCustomSerDes {
    pub(crate) fn dart_encode(&self) -> &str {
        assert_eq!(self.direction, MirDirection::Dart2Rust);
        &self.dart_code
    }

    pub(crate) fn dart_decode(&self) -> &str {
        assert_eq!(self.direction, MirDirection::Rust2Dart);
        &self.dart_code
    }

    pub(crate) fn rust_encode_function(&self) -> &NamespacedName {
        assert_eq!(self.direction, MirDirection::Rust2Dart);
        &self.rust_function
    }

    pub(crate) fn rust_decode_function(&self) -> &NamespacedName {
        assert_eq!(self.direction, MirDirection::Dart2Rust);
        &self.rust_function
    }
}
