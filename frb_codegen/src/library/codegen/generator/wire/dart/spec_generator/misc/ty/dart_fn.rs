use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::dart_fn::DartFnOutputAction;
use crate::codegen::ir::mir::ty::MirTypeTrait;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> WireDartGeneratorMiscTrait for DartFnWireDartGenerator<'a> {
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
            Future<void> Function(int, {repeated_dynamics})
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
                  callId: callId, ptr: output.ptr, rustVecLen: output.rustVecLen, dataLen: output.dataLen);
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
