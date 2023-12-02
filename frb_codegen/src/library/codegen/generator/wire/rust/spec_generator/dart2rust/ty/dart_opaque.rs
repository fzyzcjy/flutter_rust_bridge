use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorWire2apiTrait for DartOpaqueWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
            &["port: i64".to_owned(), "handle: usize".to_owned()],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
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

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        Some(
            generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                self.context,
                "Self { port: 0, handle: 0 }",
                false,
            )
            .into(),
        )
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        let rust_wire = self.rust_wire_type(Target::Io);

        Acc {
            io: ExternFunc {
                func_name: format!("new_{}", self.ir.safe_ident()),
                params: vec![],
                return_type: Some(format!(
                    "{}{}",
                    self.rust_wire_modifier(Target::Io),
                    rust_wire
                )),
                body: format!("{rust_wire}::new_with_null_ptr()"),
                target: Target::Io,
            }
            .into(),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}
