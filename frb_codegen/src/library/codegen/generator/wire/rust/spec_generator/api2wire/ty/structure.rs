use crate::codegen::generator::wire::rust::spec_generator::api2wire::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use itertools::Itertools;

impl<'a> WireRustGeneratorApi2wireTrait for StructRefWireRustGenerator<'a> {
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

                let gen = WireRustGenerator::new(field.ty.clone(), self.context);
                gen.generate_convert_to_dart(format!("self{unwrap}.{field_ref}"))
            })
            .collect_vec()
            .join(",\n");

        let name = match &src.wrapper_name {
            Some(wrapper) => wrapper,
            None => &src.name,
        };

        let vec = if src.is_empty() {
            "Vec::<u8>::new()".to_string()
        } else {
            format!(
                "vec![
                    {body}
                ]"
            )
        };

        let into_into_dart = generate_impl_into_into_dart(&src.name, src.wrapper_name.as_deref());
        Some(format!(
            "impl support::IntoDart for {name} {{
                fn into_dart(self) -> support::DartAbi {{
                    {vec}.into_dart()
                }}
            }}
            impl support::IntoDartExceptPrimitive for {name} {{}}
            {into_into_dart}
            "
        ))
    }
}
