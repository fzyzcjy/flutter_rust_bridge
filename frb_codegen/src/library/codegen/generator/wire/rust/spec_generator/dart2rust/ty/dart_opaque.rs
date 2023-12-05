use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value, JS_VALUE,
};
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorDart2RustTrait for DartOpaqueWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            // TODO extract to function
            io: Some(
                "unsafe{ flutter_rust_bridge::DartOpaque::new(self.handle as _, self.port) }"
                    .to_owned(),
            ),
            wasm: Some(
                "let arr = self.dyn_into::<flutter_rust_bridge::for_generated::js_sys::Array>().unwrap();
                unsafe{ flutter_rust_bridge::DartOpaque::new(arr.get(0), arr.get(1)) }"
                    .to_owned(),
            ),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => "int64_t",
            Target::Wasm => JS_VALUE,
        }
        .into()
    }
}
