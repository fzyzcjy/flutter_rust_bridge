use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(crate) fn generate_impl_wire2api(
    types: &[IrType],
    context: WireRustGeneratorContext,
) -> Acc<Vec<WireRustOutputCode>> {
    let mut lines = Acc::<Vec<WireRustOutputCode>>::default();
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

fn generate_impl_wire2api_misc() -> Acc<WireRustOutputCode> {
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
        .into(),
        io: "".into(),
        wasm: r#"
            impl<T> Wire2Api<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue where JsValue: Wire2Api<T> {
                fn wire2api(self) -> Option<T> {
                    (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
                }
            }
        "#
        .into(),
    }
}

fn generate_impl_wire2api_for_type(
    ty: &IrType,
    context: WireRustGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let generator = WireRustGenerator::new(ty.clone(), context);
    let raw: Acc<Option<String>> = generator.generate_impl_wire2api_body();
    raw.map(|body, target| {
        body.map(|body| {
            // When target==Common, it means things like `rust_wire_type` should be the same
            // for Io or Wasm, so we can choose any.
            let target = target.as_target_or(Target::Io);

            let rust_wire_modifier = generator.rust_wire_modifier(target);
            let rust_wire_type = generator.rust_wire_type(target);

            generate_impl_wire2api_code_block(
                &ty.rust_api_type(),
                &format!("{rust_wire_modifier}{rust_wire_type}"),
                &body,
            )
            .into()
        })
        .unwrap_or_default()
    })
}

fn generate_impl_wire2api_jsvalue_for_type(
    ty: &IrType,
    context: WireRustGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let generator = WireRustGenerator::new(ty.clone(), context);
    generator
        .generate_impl_wire2api_jsvalue_body()
        .map(|body| Acc {
            wasm: generate_impl_wire2api_code_block(
                &ty.rust_api_type(),
                "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue",
                body.as_ref(),
            )
            .into(),
            ..Default::default()
        })
        .unwrap_or_default()
}

fn generate_impl_wire2api_code_block(api: &str, wire: &str, body: &str) -> String {
    format!(
        "impl Wire2Api<{api}> for {wire} {{
            fn wire2api(self) -> {api} {{
                {body}
            }}
        }}",
    )
}
