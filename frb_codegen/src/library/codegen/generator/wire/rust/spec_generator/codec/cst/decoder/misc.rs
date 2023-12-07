use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::{
    WireRustCodecCstGenerator, WireRustCodecCstGeneratorContext,
};
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;

pub(crate) fn generate_class_from_fields(
    ty: impl Into<IrType>,
    context: WireRustCodecCstGeneratorContext,
    fields: &[String],
) -> String {
    let struct_name = WireRustCodecCstGenerator::new(ty.into(), context).rust_wire_type(Target::Io);
    format!(
        r###"
            #[repr(C)]
            #[derive(Clone)]
            pub struct {struct_name} {{
                {fields}
            }}
        "###,
        struct_name = struct_name,
        fields = fields.join(",\n"),
    )
}

pub(super) const JS_VALUE: &str = "flutter_rust_bridge::for_generated::wasm_bindgen::JsValue";

pub(super) fn rust_wire_type_add_prefix_or_js_value<T: IrTypeTrait>(
    ir: &T,
    target: Target,
) -> String {
    match target {
        Target::Io => format!("wire_cst_{}", ir.safe_ident()),
        Target::Wasm => JS_VALUE.into(),
    }
}
