use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorDart2RustTrait for DartFnWireRustGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let closure_args = (0..self.ir.inputs.len())
            .map(|i| format!("arg{i}"))
            .collect_vec();
        let closure_args_str = closure_args.join(", ");
        let closure_args_into_dart_str = closure_args
            .iter()
            .map(|x| format!("{x}.into_into_dart().into_dart()"))
            .join(", ");

        Acc::new_common(Some(format!(
            "
            let dart_opaque: flutter_rust_bridge::DartOpaque = self.wire2api();
            flutter_rust_bridge::DartFn::new(move |{closure_args_str}| {{
                flutter_rust_bridge::for_generated::dart_fn_invoke(vec![
                    dart_opaque.into_into_dart().into_dart(), {closure_args_into_dart_str}
                ])
            }})
            "
        )))
    }

    fn rust_wire_type(&self, target: Target) -> String {
        WireRustGenerator::new(self.ir.get_delegate(), self.context).rust_wire_type(target)
    }
}
