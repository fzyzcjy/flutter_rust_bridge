use crate::codegen::generator::misc::Target;
use crate::codegen::ir::ty::IrTypeTrait;

pub(in crate::library::codegen::generator::wire::rust) const JS_VALUE: &str = "JsValue";

pub(in crate::library::codegen::generator::wire::rust) fn rust_wire_type_add_prefix_or_js_value<
    T: IrTypeTrait,
>(
    ir: &T,
    target: Target,
) -> String {
    if let Target::Wasm = target {
        JS_VALUE.into()
    } else {
        format!("wire_{}", ir.safe_ident())
    }
}
