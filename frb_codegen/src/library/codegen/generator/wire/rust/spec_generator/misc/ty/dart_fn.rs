use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::misc::consts::HANDLER_NAME;
use convert_case::{Case, Casing};
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for DartFnWireRustGenerator<'a> {
    fn generate_wire_func_call_decode(&self, name: &str, codec_mode: CodecMode) -> String {
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
                let dart_opaque: flutter_rust_bridge::DartOpaque = {name}.{codec_mode}_decode();

                move |{closure_args_str}| {{    
                    {HANDLER_NAME}.dart_fn_invoke(vec![
                        dart_opaque.clone().into_into_dart().into_dart(),
                        {closure_args_into_dart_str}
                    ])
                }}
            }}
            ",
            codec_mode = codec_mode.to_string().to_case(Case::Snake)
        )
    }
}
