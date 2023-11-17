use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::common::ty::WireRustGeneratorCommonTrait;

impl<'a> WireRustGeneratorCommonTrait for EnumRefWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref().cloned()
    }

    fn generate_static_checks(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref()?;

        let branches: Vec<_> = src
            .variants()
            .iter()
            .map(|variant| match &variant.kind {
                IrVariantKind::Value => format!("{}::{} => {{}}", src.name, variant.name),
                IrVariantKind::Struct(s) => {
                    let pattern = s
                        .fields
                        .iter()
                        .map(|field| field.name.rust_style().to_owned())
                        .collect_vec();
                    let pattern = if s.is_fields_named {
                        format!("{}::{} {{ {} }}", src.name, variant.name, pattern.join(","))
                    } else {
                        format!("{}::{}({})", src.name, variant.name, pattern.join(","))
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
            .collect();
        Some(format!(
            "match None::<{}>.unwrap() {{ {} }}",
            src.name,
            branches.join(","),
        ))
    }
}
