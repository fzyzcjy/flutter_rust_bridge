use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorCommonTrait for StructRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }

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
            src_name = src.name,
        ))
    }

    fn generate_imports(&self) -> Option<Vec<String>> {
        let api_struct = self.ir.get(self.context.ir_pack);
        if api_struct.path.is_some() {
            Some(vec![format!(
                "use {};",
                api_struct.path.as_ref().unwrap().join("::")
            )])
        } else {
            None
        }
    }
}
