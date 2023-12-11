use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;
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

        let return_type = self.ir.output.rust_api_type();

        Acc::new_common(
            format!(
                "fn decode_{safe_ident}(
                    dart_opaque: flutter_rust_bridge::DartOpaque,
                ) -> impl Fn({parameter_types}) -> flutter_rust_bridge::DartFnFuture<{return_type}> {{
                    use flutter_rust_bridge::IntoDart;

                    async fn body(dart_opaque: flutter_rust_bridge::DartOpaque, {parameter_names_and_types}) -> {return_type} {{
                        let args = vec![{into_dart_expressions}];
                        let message = FLUTTER_RUST_BRIDGE_HANDLER.dart_fn_invoke(dart_opaque, args).await;
                        <{return_type}>::sse_decode_single(message)
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
}
