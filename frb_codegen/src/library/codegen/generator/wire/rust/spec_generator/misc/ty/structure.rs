use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for StructRefWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);
        src.wrapper_name.clone()
    }

    fn generate_static_checks(&self) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);
        src.wrapper_name.as_ref()?;

        let var = if src.is_fields_named {
            src.name.name.clone()
        } else {
            // let bindings cannot shadow tuple structs
            format!("{}_", src.name.name)
        };

        let checks = src
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                let field_access = if src.is_fields_named {
                    field.name.rust_style().to_owned()
                } else {
                    i.to_string()
                };
                format!(
                    "let _: {type_str} = {var}.{field_access};\n",
                    type_str = field.ty.rust_api_type(),
                )
            })
            .collect_vec()
            .join("");

        Some(format!(
            "{{ let {var} = None::<{src_name}>.unwrap(); {checks} }} ",
            src_name = src.name.rust_style(),
        ))
    }
}
