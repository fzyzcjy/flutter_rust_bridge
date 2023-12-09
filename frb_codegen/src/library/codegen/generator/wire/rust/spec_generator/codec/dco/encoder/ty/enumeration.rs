use crate::codegen::generator::codec::sse::ty::enumeration::generate_enum_encode_rust;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for EnumRefWireRustCodecDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        match &self.ir.get(ir_pack).wrapper_name {
            Some(wrapper) => wrapper.clone(),
            None => self.ir.rust_api_type(),
        }
    }

    fn generate_impl_into_dart(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let (name, self_path) =
            parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

        let body = generate_enum_encode_rust();

        let into_into_dart = generate_impl_into_into_dart(&src.name, &src.wrapper_name);
        Some(format!(
            "impl flutter_rust_bridge::IntoDart for {name} {{
                fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {{
                    {body}
                }}
            }}
            impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for {name} {{}}
            {into_into_dart}
            ",
        ))
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        let src = self.ir.get(self.context.ir_pack);
        match &src.wrapper_name {
            Some(_) => format!("{obj}.0"),
            None => obj,
        }
    }
}

pub(super) fn parse_wrapper_name_into_dart_name_and_self_path(
    name: &NamespacedName,
    wrapper_name: &Option<String>,
) -> (String, String) {
    match &wrapper_name {
        Some(wrapper) => (wrapper.clone(), name.rust_style()),
        None => (name.rust_style(), "Self".into()),
    }
}
