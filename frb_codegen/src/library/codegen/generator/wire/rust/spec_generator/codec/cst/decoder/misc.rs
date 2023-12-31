use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternClass, ExternClassMode,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

pub(crate) fn generate_class_from_fields(
    ty: impl Into<IrType>,
    context: WireRustCodecCstGeneratorContext,
    fields: &[String],
) -> WireRustOutputCode {
    let struct_name = WireRustCodecCstGenerator::new(ty.into(), context).rust_wire_type(Target::Io);
    WireRustOutputCode {
        extern_classes: vec![ExternClass {
            partial_name: struct_name,
            mode: ExternClassMode::Struct,
            body: fields.join(",\n"),
        }],
        ..Default::default()
    }
}

pub(super) const JS_VALUE: &str = "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue";

pub(super) fn rust_wire_type_add_prefix_or_js_value<T: IrTypeTrait>(
    ir: &T,
    target: Target,
) -> String {
    match target {
        Target::Io => format!("wire_cst_{}", ir.safe_ident()),
        Target::Web => JS_VALUE.into(),
    }
}
