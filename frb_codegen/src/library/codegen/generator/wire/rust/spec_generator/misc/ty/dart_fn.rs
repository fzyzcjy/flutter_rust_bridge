use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::result::IrMaybeResult;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::misc::consts::HANDLER_NAME;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for DartFnWireRustGenerator<'a> {
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let safe_ident = self.ir.safe_ident();

        let num_params = self.ir.inputs.len();
        let parameter_names = (0..num_params).map(|i| format!("arg{i}")).join(", ");
        let parameter_types = (self.ir.inputs.iter())
            .map(|x| x.rust_api_type())
            .collect_vec();
        let parameter_names_and_types = (parameter_types.iter().enumerate())
            .map(|(i, x)| format!("arg{i}: {x}"))
            .join(", ");
        let into_dart_expressions = (0..num_params)
            .map(|i| format!("arg{i}.into_into_dart().into_dart(),"))
            .join("");

        let return_type_outer = self.ir.output.rust_api_type();
        let return_type_inner = self.ir.output.delegate.rust_api_type();
        let return_type_inner_to_outer = generate_return_type_inner_to_outer(&self.ir.output);

        Acc::new_common(
            format!(
                "fn decode_{safe_ident}(
                    dart_opaque: flutter_rust_bridge::DartOpaque,
                ) -> impl Fn({parameter_types}) -> flutter_rust_bridge::DartFnFuture<{return_type_outer}> {{
                    use flutter_rust_bridge::IntoDart;

                    async fn body(dart_opaque: flutter_rust_bridge::DartOpaque, {parameter_names_and_types}) -> {return_type_outer} {{
                        let args = vec![{into_dart_expressions}];
                        let message = {HANDLER_NAME}.dart_fn_invoke(dart_opaque, args).await;
                        let decoded = <{return_type_inner}>::sse_decode_single(message);
                        {return_type_inner_to_outer}
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
        Some(format!("decode_{}", self.ir.safe_ident()))
    }

    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        Some(self.ir.get_delegate().rust_api_type())
    }
}

fn generate_return_type_inner_to_outer(ir: &IrMaybeResult) -> String {
    let delegate_type = ir.delegate.rust_api_type();
    if let Some(error) = ir.error {
        format!(
            "match decoded {{
                {delegate_type}::Ok(value) => std::result::Result::Ok(value),
                {delegate_type}::Err(value) => std::result::Result::Err(value),
            }}"
        )
    } else {
        format!("decoded")
    }
}
