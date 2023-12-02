use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::{
    WireRustGenerator, WireRustGeneratorContext,
};
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;

pub(crate) fn generate_class_from_fields(
    ty: impl Into<IrType>,
    context: WireRustGeneratorContext,
    fields: &[String],
) -> String {
    let struct_name = WireRustGenerator::new(ty.into(), context).rust_wire_type(Target::Io);
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
    if let Target::Wasm = target {
        JS_VALUE.into()
    } else {
        format!("wire_{}", ir.safe_ident())
    }
}
