use super::*;

#[derive(Debug)]
pub(crate) struct GeneratedApiFunc {
    pub(crate) signature: String,
    pub(crate) implementation: String,
    pub(crate) comments: String,
    pub(crate) companion_field_signature: String,
    pub(crate) companion_field_implementation: String,
}

// DONE
// pub(crate) fn generate_api_func(
//     func: &IrFunc,
//     ir_pack: &IrPack,
//     common_api2wire_body: &str,
//     dart_enums_style: bool,
// ) -> GeneratedApiFunc {
// }

pub(crate) fn generate_opaque_getters(ty: &IrType) -> GeneratedApiFunc {
    let signature = format!(
        "
    DropFnType get dropOpaque{0};
    ShareFnType get shareOpaque{0};
    OpaqueTypeFinalizer get {0}Finalizer;
    ",
        ty.dart_api_type(),
    );

    let implementation = format!(
        "
        DropFnType get dropOpaque{0} => _platform.inner.drop_opaque_{0};
        ShareFnType get shareOpaque{0} => _platform.inner.share_opaque_{0};
        OpaqueTypeFinalizer get {0}Finalizer => _platform.{0}Finalizer;
        ",
        ty.dart_api_type()
    );

    GeneratedApiFunc {
        signature,
        implementation,
        comments: String::new(),
        companion_field_signature: String::new(),
        companion_field_implementation: String::new(),
    }
}
