use crate::codegen::generator::wire::rust::api2wire::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::api2wire::ty::WireRustGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use itertools::Itertools;

impl<'a> WireRustGeneratorApi2wireTrait for EnumRefWireRustGenerator<'a> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);

        let (name, self_path): (&str, &str) = match &src.wrapper_name {
            Some(wrapper) => (wrapper, &src.name),
            None => (&src.name, "Self"),
        };
        let self_ref = self.self_access("self".to_owned());
        let variants = src
            .variants()
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let tag = format!("{idx}.into_dart()");
                match &variant.kind {
                    IrVariantKind::Value => {
                        format!("{self_path}::{} => vec![{tag}],", variant.name.raw)
                    }
                    IrVariantKind::Struct(st) => {
                        let fields = Some(tag)
                            .into_iter()
                            .chain(st.fields.iter().map(|field| {
                                let gen =
                                    WireRustGenerator::new(field.ty.clone(), self.context.clone());
                                gen.generate_convert_to_dart(field.name.rust_style().to_owned())
                            }))
                            .collect_vec();
                        let pattern = st
                            .fields
                            .iter()
                            .map(|field| field.name.rust_style().to_owned())
                            .collect_vec();
                        let (left, right) = st.brackets_pair();
                        format!(
                            "{}::{}{}{}{} => vec![{}],",
                            self_path,
                            variant.name.raw,
                            left,
                            pattern.join(","),
                            right,
                            fields.join(",")
                        )
                    }
                }
            })
            .collect_vec();

        let into_into_dart = generate_impl_into_into_dart(&src.name, src.wrapper_name.as_deref());
        Some(format!(
            "impl support::IntoDart for {} {{
                fn into_dart(self) -> support::DartAbi {{
                    match {} {{
                        {}
                    }}.into_dart()
                }}
            }}
            impl support::IntoDartExceptPrimitive for {0} {{}}
            {into_into_dart}
            ",
            name,
            self_ref,
            variants.join("\n")
        ))
    }

    fn self_access(&self, obj: String) -> String {
        let src = self.ir.get(self.context.ir_pack);
        match &src.wrapper_name {
            Some(_) => format!("{obj}.0"),
            None => obj,
        }
    }
}
