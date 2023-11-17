use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use crate::library::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;
use std::convert::TryInto;

pub(crate) fn generate_impl_wire2api(
    types: &[IrType],
    context: &WireRustGeneratorContext,
) -> Acc<Vec<String>> {
    let mut lines = Acc::<Vec<String>>::default();
    lines.push_acc(generate_impl_wire2api_misc());
    lines += types
        .iter()
        .map(|ty| generate_impl_wire2api_for_type(ty, context))
        .collect();
    lines += types
        .iter()
        .map(|ty| generate_impl_wire2api_jsvalue_for_type(ty, context))
        .collect();
    lines
}

fn generate_impl_wire2api_misc() -> Acc<String> {
    Acc {
        common: r#"
            pub trait Wire2Api<T> {
                fn wire2api(self) -> T;
            }

            impl<T, S> Wire2Api<Option<T>> for *mut S
            where
                *mut S: Wire2Api<T>
            {
                fn wire2api(self) -> Option<T> {
                    (!self.is_null()).then(|| self.wire2api())
                }
            }
        "#
        .to_owned(),
        io: "".to_owned(),
        wasm: r#"
            impl<T> Wire2Api<Option<T>> for JsValue where JsValue: Wire2Api<T> {
                fn wire2api(self) -> Option<T> {
                    (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
                }
            }
        "#
        .to_owned(),
    }
}

fn generate_impl_wire2api_for_type(ty: &IrType, context: &WireRustGeneratorContext) -> Acc<String> {
    let generator = WireRustGenerator::new(ty.clone(), context.clone());
    let raw: Acc<Option<String>> = generator.generate_impl_wire2api_body();
    raw.map(|body, target| {
        body.map(|body| {
            let target: Target = target.try_into().unwrap();
            format!(
                "impl Wire2Api<{api}> for {rust_wire_modifier}{rust_wire_type} {{
                    fn wire2api(self) -> {api} {{
                        {body}
                    }}
                }}",
                rust_wire_modifier = generator.rust_wire_modifier(target),
                rust_wire_type = generator.rust_wire_type(target),
                api = ty.rust_api_type(),
            )
        })
        .unwrap_or_default()
    })
}

fn generate_impl_wire2api_jsvalue_for_type(
    ty: &IrType,
    context: &WireRustGeneratorContext,
) -> Acc<String> {
    let generator = WireRustGenerator::new(ty.clone(), context.clone());
    generator
        .generate_impl_wire2api_jsvalue_body()
        .map(|body| Acc {
            wasm: format!(
                "impl Wire2Api<{api}> for JsValue {{
                    fn wire2api(self) -> {api} {{
                        {body}
                    }}
                }}",
                api = ty.rust_api_type()
            ),
            ..Default::default()
        })
        .unwrap_or_default()
}
