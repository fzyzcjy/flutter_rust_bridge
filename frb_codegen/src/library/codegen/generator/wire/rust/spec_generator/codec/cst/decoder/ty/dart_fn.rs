use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::misc::consts::HANDLER_NAME;
use itertools::Itertools;

impl<'a> WireRustCodecCstGeneratorDecoderTrait for DartFnWireRustCodecCstGenerator<'a> {
    fn generate_wire_func_call_decode(&self, name: &str) -> String {
        let closure_args = (0..self.ir.inputs.len())
            .map(|i| format!("arg{i}"))
            .collect_vec();
        let closure_args_str = closure_args.join(", ");
        let closure_args_into_dart_str = closure_args
            .iter()
            .map(|x| format!("{x}.into_into_dart().into_dart()"))
            .join(", ");

        format!(
            "
            {{
                use flutter_rust_bridge::IntoDart;
                let dart_opaque: flutter_rust_bridge::DartOpaque = {name}.cst_decode();

                move |{closure_args_str}| {{    
                    {HANDLER_NAME}.dart_fn_invoke(vec![
                        dart_opaque.clone().into_into_dart().into_dart(),
                        {closure_args_into_dart_str}
                    ])
                }}
            }}
            "
        )
    }

    fn generate_wire_func_param_api_type(&self) -> Option<String> {
        Some(self.ir.get_delegate().rust_api_type())
    }

    fn rust_wire_type(&self, target: Target) -> String {
        WireRustCodecCstGenerator::new(self.ir.get_delegate(), self.context).rust_wire_type(target)
    }
}
