use crate::codegen::generator::misc::Target;
use crate::codegen::ir::ty::IrTypeTrait;

pub(super) const JS_VALUE: &str = "JsValue";

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
