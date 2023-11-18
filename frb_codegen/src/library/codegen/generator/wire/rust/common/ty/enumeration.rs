use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorCommonTrait for EnumRefWireRustGenerator<'a> {
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

        let branches = src
            .variants()
            .iter()
            .map(|variant| match &variant.kind {
                IrVariantKind::Value => format!("{}::{} => {{}}", src.name, &variant.name.raw),
                IrVariantKind::Struct(s) => {
                    let pattern = s
                        .fields
                        .iter()
                        .map(|field| field.name.rust_style().to_owned())
                        .collect_vec();
                    let pattern = if s.is_fields_named {
                        format!(
                            "{}::{} {{ {} }}",
                            src.name,
                            variant.name.raw,
                            pattern.join(",")
                        )
                    } else {
                        format!("{}::{}({})", src.name, &variant.name.raw, pattern.join(","))
                    };

                    let checks = s
                        .fields
                        .iter()
                        .map(|field| {
                            format!(
                                "let _: {} = {};\n",
                                field.ty.rust_api_type(),
                                field.name.rust_style(),
                            )
                        })
                        .collect_vec();

                    format!("{} => {{ {} }}", pattern, checks.join(""))
                }
            })
            .collect_vec();

        Some(format!(
            "match None::<{}>.unwrap() {{ {} }}",
            src.name,
            branches.join(","),
        ))
    }

    fn generate_imports(&self) -> Option<Vec<String>> {
        let api_enum = self.ir.get(self.context.ir_pack);
        Some(vec![format!("use {};", api_enum.path.join("::"))])
    }
}
