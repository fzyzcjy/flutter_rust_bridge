use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(crate) fn generate_impl_decode(
    types: &[IrType],
    context: WireRustCodecCstGeneratorContext,
) -> Acc<Vec<WireRustOutputCode>> {
    let mut lines = Acc::<Vec<WireRustOutputCode>>::default();
    lines.push_acc(generate_impl_decode_misc());
    lines += types
        .iter()
        .map(|ty| generate_impl_decode_for_type(ty, context))
        .collect();
    lines += types
        .iter()
        .map(|ty| generate_impl_decode_jsvalue_for_type(ty, context))
        .collect();
    lines
}

fn generate_impl_decode_misc() -> Acc<WireRustOutputCode> {
    Acc {
        common: "".into(),
        io: "".into(),
        web: r#"
            impl<T> CstDecode<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue where JsValue: CstDecode<T> {
                fn cst_decode(self) -> Option<T> {
                    (!self.is_null() && !self.is_undefined()).then(|| self.cst_decode())
                }
            }
        "#
        .into(),
    }
}

fn generate_impl_decode_for_type(
    ty: &IrType,
    context: WireRustCodecCstGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let generator = WireRustCodecCstGenerator::new(ty.clone(), context);
    let raw: Acc<Option<String>> = generator.generate_impl_decode_body();
    raw.map(|body, target| {
        body.map(|body| {
            // When target==Common, it means things like `rust_wire_type` should be the same
            // for Io or Web, so we can choose any.
            let target = target.as_target_or(Target::Io);

            let rust_wire_modifier = generator.rust_wire_modifier(target);
            let rust_wire_type = generator.rust_wire_type(target);

            generate_impl_decode_code_block(
                &ty.rust_api_type(),
                &format!("{rust_wire_modifier}{rust_wire_type}"),
                &body,
            )
            .into()
        })
        .unwrap_or_default()
    })
}

fn generate_impl_decode_jsvalue_for_type(
    ty: &IrType,
    context: WireRustCodecCstGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let generator = WireRustCodecCstGenerator::new(ty.clone(), context);
    generator
        .generate_impl_decode_jsvalue_body()
        .map(|body| Acc {
            web: generate_impl_decode_code_block(
                &ty.rust_api_type(),
                "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue",
                body.as_ref(),
            )
            .into(),
            ..Default::default()
        })
        .unwrap_or_default()
}

fn generate_impl_decode_code_block(api: &str, wire: &str, body: &str) -> String {
    let body = body.trim();
    format!(
        "impl CstDecode<{api}> for {wire} {{
            fn cst_decode(self) -> {api} {{
                {body}
            }}
        }}",
    )
}
