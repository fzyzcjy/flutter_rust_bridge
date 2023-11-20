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
//
// pub(crate) fn generate_opaque_getters(ty: &IrType) -> GeneratedApiFunc {
//
// }
