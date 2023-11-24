use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for StructRefWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref().map(|x| x.name.clone()).to_owned()
    }

    fn generate_static_checks(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        if src.wrapper_name.is_none() {
            return None;
        }

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
                    field.name.raw.clone()
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
            src_name = src.name.name,
        ))
    }

    fn generate_imports(&self) -> Option<Vec<String>> {
        let api_struct = self.ir.get(self.context.ir_pack);
        (api_struct.path.as_ref()).map(|path| vec![format!("use {};", path.join("::"))])
    }
}
