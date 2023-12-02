use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for EnumRefWireRustGenerator<'a> {
    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        match &self.ir.get(ir_pack).wrapper_name {
            Some(wrapper) => wrapper.clone(),
            None => self.ir.rust_api_type(),
        }
    }

    fn generate_impl_into_dart(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        let (name, self_path) =
            parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

        let self_ref = self.generate_access_object_core("self".to_owned());
        let variants = src
            .variants()
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let tag = format!("{idx}.into_dart()");
                match &variant.kind {
                    IrVariantKind::Value => {
                        format!("{self_path}::{} => vec![{tag}],", variant.name)
                    }
                    IrVariantKind::Struct(st) => {
                        let fields = Some(tag)
                            .into_iter()
                            .chain(st.fields.iter().map(|field| {
                                format!("{}.into_into_dart().into_dart()", field.name.rust_style())
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
                            variant.name,
                            left,
                            pattern.join(","),
                            right,
                            fields.join(",")
                        )
                    }
                }
            })
            .collect_vec();

        let into_into_dart = generate_impl_into_into_dart(&src.name, &src.wrapper_name);
        Some(format!(
            "impl flutter_rust_bridge::IntoDart for {} {{
                fn into_dart(self) -> flutter_rust_bridge::DartAbi {{
                    match {} {{
                        {}
                    }}.into_dart()
                }}
            }}
            impl flutter_rust_bridge::for_generated::IntoDartExceptPrimitive for {0} {{}}
            {into_into_dart}
            ",
            name,
            self_ref,
            variants.join("\n")
        ))
    }

    fn generate_access_object_core(&self, obj: String) -> String {
        let src = self.ir.get(self.context.ir_pack);
        match &src.wrapper_name {
            Some(_) => format!("{obj}.0"),
            None => obj,
        }
    }
}

pub(super) fn parse_wrapper_name_into_dart_name_and_self_path(
    name: &NamespacedName,
    wrapper_name: &Option<String>,
) -> (String, String) {
    match &wrapper_name {
        Some(wrapper) => (wrapper.clone(), name.rust_style()),
        None => (name.rust_style(), "Self".into()),
    }
}
