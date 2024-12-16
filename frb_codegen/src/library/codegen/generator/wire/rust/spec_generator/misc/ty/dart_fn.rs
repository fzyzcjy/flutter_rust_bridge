use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use crate::library::misc::consts::HANDLER_NAME;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for DartFnWireRustGenerator<'a> {
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let safe_ident = self.mir.safe_ident();

        let num_params = self.mir.inputs.len();
        let parameter_names = (0..num_params).map(|i| format!("arg{i}")).join(", ");
        let parameter_types = (self.mir.inputs.iter())
            .map(|x| x.rust_api_type())
            .collect_vec();
        let parameter_names_and_types = (parameter_types.iter().enumerate())
            .map(|(i, x)| format!("arg{i}: {x}"))
            .join(", ");
        let into_dart_expressions = (0..num_params)
            .map(|i| format!("arg{i}.into_into_dart().into_dart(),"))
            .join("");

        let return_type_outer = self.mir.output.rust_api_type();
        let output_normal_type = self.mir.output.normal.rust_api_type();
        let output_error_type = self.mir.output.error.rust_api_type();

        let action_normal = DartFnOutputAction::Success as i32;
        let action_error = DartFnOutputAction::Error as i32;

        let maybe_unwrap_ans = if self.mir.output.api_fallible {
            ""
        } else {
            r#"let ans = ans.expect("Dart throws exception but Rust side assume it is not failable");"#
        };

        Acc::new_common(
            format!(
                "fn decode_{safe_ident}(
                    dart_opaque: flutter_rust_bridge::DartOpaque,
                ) -> impl Fn({parameter_types}) -> flutter_rust_bridge::DartFnFuture<{return_type_outer}> {{
                    use flutter_rust_bridge::IntoDart;

                    async fn body(dart_opaque: flutter_rust_bridge::DartOpaque, {parameter_names_and_types}) -> {return_type_outer} {{
                        let args = vec![{into_dart_expressions}];
                        let message = {HANDLER_NAME}.dart_fn_invoke(dart_opaque, args).await;

                        let mut deserializer = flutter_rust_bridge::for_generated::SseDeserializer::new(message);
                        let action = deserializer.cursor.read_u8().unwrap();
                        let ans = match action {{
                            {action_normal} => std::result::Result::Ok(<{output_normal_type}>::sse_decode(&mut deserializer)),
                            {action_error} => std::result::Result::Err(<{output_error_type}>::sse_decode(&mut deserializer)),
                            _ => unreachable!(),
                        }};
                        deserializer.end();
                        {maybe_unwrap_ans}ans
                    }}

                    move |{parameter_names_and_types}| {{
                        flutter_rust_bridge::for_generated::convert_into_dart_fn_future(body(
                            dart_opaque.clone(), {parameter_names}
                        ))
                    }}
                }}",
                parameter_types = parameter_types.join(", "),
            )
            .into(),
        )
    }

    fn generate_wire_func_call_decode_wrapper(&self) -> Option<String> {
        Some(format!("decode_{}", self.mir.safe_ident()))
    }

    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        Some(self.mir.get_delegate().rust_api_type())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DartFnOutputAction {
    Success = 0,
    Error = 1,
}
