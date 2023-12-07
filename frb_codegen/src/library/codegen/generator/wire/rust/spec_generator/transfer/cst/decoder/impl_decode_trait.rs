use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::{
    WireRustTransferCstGenerator, WireRustTransferCstGeneratorContext,
};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

pub(crate) fn generate_impl_decode(
    types: &[IrType],
    context: WireRustTransferCstGeneratorContext,
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
        common: r#"
            pub trait CstDecoder<T> {
                fn cst_decode(self) -> T;
            }

            impl<T, S> CstDecoder<Option<T>> for *mut S
            where
                *mut S: CstDecoder<T>
            {
                fn cst_decode(self) -> Option<T> {
                    (!self.is_null()).then(|| self.cst_decode())
                }
            }
        "#
        .into(),
        io: "".into(),
        wasm: r#"
            impl<T> CstDecoder<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue where JsValue: CstDecoder<T> {
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
    context: WireRustTransferCstGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let generator = WireRustTransferCstGenerator::new(ty.clone(), context);
    let raw: Acc<Option<String>> = generator.generate_impl_decode_body();
    raw.map(|body, target| {
        body.map(|body| {
            // When target==Common, it means things like `rust_wire_type` should be the same
            // for Io or Wasm, so we can choose any.
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
    context: WireRustTransferCstGeneratorContext,
) -> Acc<WireRustOutputCode> {
    let generator = WireRustTransferCstGenerator::new(ty.clone(), context);
    generator
        .generate_impl_decode_jsvalue_body()
        .map(|body| Acc {
            wasm: generate_impl_decode_code_block(
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
    format!(
        "impl CstDecoder<{api}> for {wire} {{
            fn cst_decode(self) -> {api} {{
                {body}
            }}
        }}",
    )
}
