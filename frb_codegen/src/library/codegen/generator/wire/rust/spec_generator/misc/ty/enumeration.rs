use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::mir::ty::enumeration::MirVariantKind;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for EnumRefWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);
        src.wrapper_name.clone()
    }

    fn generate_static_checks(&self) -> Option<String> {
        let src = self.mir.get(self.context.mir_pack);
        src.wrapper_name.as_ref()?;

        let branches = src
            .variants()
            .iter()
            .map(|variant| match &variant.kind {
                MirVariantKind::Value => {
                    format!("{}::{} => {{}}", src.name.rust_style(), &variant.name)
                }
                MirVariantKind::Struct(s) => {
                    let pattern = s
                        .fields
                        .iter()
                        .map(|field| field.name.rust_style().to_owned())
                        .collect_vec();
                    let pattern = if s.is_fields_named {
                        format!(
                            "{}::{} {{ {} }}",
                            src.name.rust_style(),
                            variant.name,
                            pattern.join(",")
                        )
                    } else {
                        format!(
                            "{}::{}({})",
                            src.name.rust_style(),
                            &variant.name,
                            pattern.join(",")
                        )
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
            src.name.rust_style(),
            branches.join(","),
        ))
    }
}
