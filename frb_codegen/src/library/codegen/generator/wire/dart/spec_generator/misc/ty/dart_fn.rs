use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::dart_fn::DartFnOutputAction;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use itertools::Itertools;

impl WireDartGeneratorMiscTrait for DartFnWireDartGenerator<'_> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        let num_params = self.mir.inputs.len();
        let raw_parameter_names = (0..num_params).map(|i| format!("rawArg{i}")).join(", ");
        let parameter_names = (0..num_params).map(|i| format!("arg{i}")).join(", ");
        let repeated_dynamics = (0..num_params).map(|_i| "dynamic".to_string()).join(", ");
        let decode_block = (self.mir.inputs.iter().enumerate())
            .map(|(i, ty)| {
                format!(
                    "final arg{i} = dco_decode_{}(rawArg{i});\n",
                    ty.safe_ident()
                )
            })
            .join("");
        let mir_safe_ident = self.mir.safe_ident();
        let dart_api_type =
            ApiDartGenerator::new(self.mir.clone(), self.context.as_api_dart_context())
                .dart_api_type();

        let output_normal_dart_api_type = ApiDartGenerator::new(
            self.mir.output.normal.clone(),
            self.context.as_api_dart_context(),
        )
        .dart_api_type();
        let output_error_dart_api_type = ApiDartGenerator::new(
            self.mir.output.error.clone(),
            self.context.as_api_dart_context(),
        )
        .dart_api_type();
        let output_normal_safe_ident = self.mir.output.normal.safe_ident();
        let output_error_safe_ident = self.mir.output.error.safe_ident();

        let action_normal = DartFnOutputAction::Success as i32;
        let action_error = DartFnOutputAction::Error as i32;

        let api_impl_body = format!(
            r#"
            Future<void> Function(dynamic, {repeated_dynamics})
                encode_{mir_safe_ident}({dart_api_type} raw) {{
              return (callId, {raw_parameter_names}) async {{
                {decode_block}

                Box<{output_normal_dart_api_type}>? rawOutput;
                Box<{output_error_dart_api_type}>? rawError;
                try {{
                    rawOutput = Box(await raw({parameter_names}));
                }} catch (e, s) {{
                    rawError = Box(AnyhowException("$e\n\n$s"));
                }}

                final serializer = SseSerializer(generalizedFrbRustBinding);
                assert((rawOutput != null) ^ (rawError != null));
                if (rawOutput != null) {{
                    serializer.buffer.putUint8({action_normal});
                    sse_encode_{output_normal_safe_ident}(rawOutput.value, serializer);
                }} else {{
                    serializer.buffer.putUint8({action_error});
                    sse_encode_{output_error_safe_ident}(rawError!.value, serializer);
                }}
                final output = serializer.intoRaw();

                generalizedFrbRustBinding.dartFnDeliverOutput(
                  callId: dcoDecodePrimitiveInt(callId), ptr: output.ptr, rustVecLen: output.rustVecLen, dataLen: output.dataLen);
              }};
            }}
            "#,
        );
        Some(Acc::new_common(WireDartOutputCode {
            api_impl_class_body: api_impl_body,
            ..Default::default()
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::test_utils;
    use crate::codegen::ir::mir::ty::dart_fn::{MirDartFnOutput, MirTypeDartFn};
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::MirType;

    /// Emits callback decoding, both result encodings, and delivery for Dart functions.
    #[test]
    fn dart_function_extra_functions_encode_success_error_and_delivery() {
        let pack = test_utils::pack();
        let api_dart_config = test_utils::api_dart_config();
        let wire_dart_config = test_utils::wire_dart_config(false);
        let wire_rust_config = test_utils::wire_rust_config(false);
        let mir = MirTypeDartFn {
            inputs: vec![MirType::Primitive(MirTypePrimitive::I32)],
            output: Box::new(MirDartFnOutput {
                normal: MirType::Primitive(MirTypePrimitive::I32),
                error: MirType::Primitive(MirTypePrimitive::Bool),
                api_fallible: true,
            }),
        };
        let mir_safe_ident = mir.safe_ident();
        let generator = DartFnWireDartGenerator::new(
            mir,
            WireDartGeneratorContext {
                mir_pack: &pack,
                config: &wire_dart_config,
                wire_rust_config: &wire_rust_config,
                api_dart_config: &api_dart_config,
            },
        );

        let output = generator.generate_extra_functions().unwrap();
        let body = &output.common.api_impl_class_body;
        assert!(body.contains(&format!("encode_{mir_safe_ident}")));
        assert!(body.contains("Future<void> Function(dynamic, dynamic)"));
        assert!(body.contains("final arg0 = dco_decode_i_32(rawArg0);"));
        assert!(body.contains("try {"));
        assert!(body.contains("} catch (e, s) {"));
        assert!(body.contains("sse_encode_i_32(rawOutput.value, serializer);"));
        assert!(body.contains("sse_encode_bool(rawError!.value, serializer);"));
        assert!(body.contains(&format!(
            "serializer.buffer.putUint8({});",
            DartFnOutputAction::Success as i32
        )));
        assert!(body.contains(&format!(
            "serializer.buffer.putUint8({});",
            DartFnOutputAction::Error as i32
        )));
        assert!(body.contains("generalizedFrbRustBinding.dartFnDeliverOutput("));
        assert!(body.contains("callId: dcoDecodePrimitiveInt(callId)"));
    }
}
