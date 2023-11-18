use crate::generator::dart::ty::*;
use crate::generator::dart::{dart_comments, dart_metadata};
use crate::target::Acc;
use crate::type_dart_generator_struct;
use crate::utils::method::FunctionName;
use crate::utils::misc::dart_maybe_implements_exception;
use crate::{ir::*, Opts};
use convert_case::{Case, Casing};

type_dart_generator_struct!(TypeStructRefGenerator, IrTypeStructRef);

impl TypeDartGeneratorTrait for TypeStructRefGenerator<'_> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        Acc {
            wasm: self.context.config.wasm_enabled.then(|| {
                format!(
                    "return [{}];",
                    self.ir
                        .get(self.context.ir_pack)
                        .fields
                        .iter()
                        .map(|field| {
                            format!(
                                "api2wire_{}(raw.{})",
                                field.ty.safe_ident(),
                                field.name.dart_style()
                            )
                        })
                        .collect_vec()
                        .join(",")
                )
            }),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_pack);
        Some(
            s.fields
                .iter()
                .map(|field| {
                    api_fill_for_field(
                        &field.ty.safe_ident(),
                        &field.name.dart_style(),
                        field.name.rust_style(),
                        field.ty.is_struct(),
                    )
                })
                .collect_vec()
                .join("\n"),
        )
    }

    fn generate_impl_wire2api_body(&self) -> String {
        let src = self.ir.get(self.context.ir_pack);
        let s = self.ir.get(self.context.ir_pack);

        let mut methods = self.context.ir_pack.funcs.iter().filter(|f| {
            let f = FunctionName::deserialize(&f.name);
            f.is_instance_method_for_struct(&src.name) || f.is_static_method_for_struct(&src.name)
        });
        let has_methods = methods.next().is_some();
        let mut inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                format!(
                    "{}: _wire2api_{}(arr[{}]),",
                    field.name.dart_style(),
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect_vec();
        if has_methods && self.context.config.use_bridge_in_method {
            inner.insert(0, "bridge: this,".to_string());
        }

        let inner = inner.join("\n");
        let cast = "final arr = raw as List<dynamic>;".to_string();
        let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
        format!(
            "{}
                {}
                return {}({});",
            cast, safe_check, s.name, inner,
        )
    }
}

#[inline]
pub(crate) fn api_fill_for_field(
    safe_ident: &str,
    dart_style: &str,
    rust_style: &str,
    is_struct: bool,
) -> String {
    if is_struct {
        format!(
            "_api_fill_to_wire_{}(apiObj.{}, wireObj.{});",
            safe_ident, dart_style, rust_style
        )
    } else {
        format!(
            "wireObj.{} = api2wire_{}(apiObj.{});",
            rust_style, safe_ident, dart_style
        )
    }
}
