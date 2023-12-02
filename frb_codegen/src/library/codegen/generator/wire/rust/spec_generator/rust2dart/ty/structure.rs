use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::enumeration::parse_wrapper_name_into_dart_name_and_self_path;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::pack::IrPack;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for StructRefWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let wrapper = &self.ir.get(ir_pack).wrapper_name;
        wrapper.clone().unwrap_or(self.ir.rust_api_type())
    }

    fn generate_impl_into_dart(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);

        let unwrap = match &src.wrapper_name {
            Some(_) => ".0",
            None => "",
        };
        let body = src
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_ref = if src.is_fields_named {
                    field.name.rust_style().to_string()
                } else {
                    i.to_string()
                };

                format!("self{unwrap}.{field_ref}.into_into_dart().into_dart()")
            })
            .collect_vec()
            .join(",\n");

        let (name, _) =
            parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

        let vec = if src.is_empty() {
            "Vec::<u8>::new()".to_string()
        } else {
            format!(
                "vec![
                    {body}
                ]"
            )
        };

        let into_into_dart = generate_impl_into_into_dart(&src.name, &src.wrapper_name);
        Some(format!(
            "impl flutter_rust_bridge::IntoDart for {name} {{
                fn into_dart(self) -> flutter_rust_bridge::for_generated::DartAbi {{
                    {vec}.into_dart()
                }}
            }}
            impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for {name} {{}}
            {into_into_dart}
            "
        ))
    }
}
