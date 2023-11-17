use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorCommonTrait for StructRefWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref().cloned()
    }

    fn generate_static_checks(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref()?;

        let var = if src.is_fields_named {
            src.name.clone()
        } else {
            // let bindings cannot shadow tuple structs
            format!("{}_", src.name)
        };
        let checks = src
            .fields
            .iter()
            .enumerate()
            .map(|(i, field)| {
                format!(
                    "let _: {} = {}.{};\n",
                    field.ty.rust_api_type(),
                    var,
                    if src.is_fields_named {
                        field.name.raw.to_string()
                    } else {
                        i.to_string()
                    },
                )
            })
            .collect_vec()
            .join("");
        Some(format!(
            "{{ let {} = None::<{}>.unwrap(); {} }} ",
            var, src.name, checks
        ))
    }
}
