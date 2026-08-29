use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use crate::library::misc::consts::HANDLER_NAME;
use itertools::Itertools;

impl WireRustGeneratorMiscTrait for DartFnWireRustGenerator<'_> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dart_fn::{MirDartFnOutput, MirTypeDartFn};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    fn generated_body(api_fallible: bool) -> String {
        let pack = test_utils::pack();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let api_dart_config = test_utils::api_dart_config();
        let generator = DartFnWireRustGenerator::new(
            MirTypeDartFn {
                inputs: vec![MirType::Primitive(MirTypePrimitive::I32)],
                output: Box::new(MirDartFnOutput {
                    normal: MirType::Primitive(MirTypePrimitive::I32),
                    error: MirType::Primitive(MirTypePrimitive::Bool),
                    api_fallible,
                }),
            },
            WireRustGeneratorContext {
                mir_pack: &pack,
                config: &wire_rust_config,
                wire_dart_config: &wire_dart_config,
                api_dart_config: &api_dart_config,
            },
        );

        generator.generate_related_funcs().common.body
    }

    /// Keeps Rust and Dart callback result action bytes distinct and stable.
    #[test]
    fn output_actions_use_the_wire_protocol_discriminants() {
        assert_eq!(DartFnOutputAction::Success as i32, 0);
        assert_eq!(DartFnOutputAction::Error as i32, 1);
    }

    /// Emits both callback result actions, completes decoding, and preserves fallibility.
    #[test]
    fn related_function_decodes_success_error_and_fallible_results() {
        let body = generated_body(true);

        assert!(body.contains("fn decode_DartFn_Inputs_i_32_Output_i_32_bool("));
        assert!(body.contains("async fn body(dart_opaque: flutter_rust_bridge::DartOpaque, arg0: i32) -> std::result::Result<i32, bool>"));
        assert!(body.contains("0 => std::result::Result::Ok(<i32>::sse_decode(&mut deserializer))"));
        assert!(
            body.contains("1 => std::result::Result::Err(<bool>::sse_decode(&mut deserializer))")
        );
        assert!(body.contains("deserializer.end();"));
        assert!(!body.contains("Dart throws exception but Rust side assume it is not failable"));
    }

    /// Unwraps callback failures only for infallible Rust API function types.
    #[test]
    fn related_function_unwraps_only_infallible_results() {
        let body = generated_body(false);

        assert!(body.contains(
            "async fn body(dart_opaque: flutter_rust_bridge::DartOpaque, arg0: i32) -> i32"
        ));
        assert!(body.contains("let ans = ans.expect(\"Dart throws exception but Rust side assume it is not failable\");"));
        assert!(body.contains("move |arg0: i32| {"));
        assert!(body.contains("convert_into_dart_fn_future(body("));
        assert!(body.contains("dart_opaque.clone(), arg0"));
    }
}
