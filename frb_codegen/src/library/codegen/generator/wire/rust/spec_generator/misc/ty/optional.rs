use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl WireRustGeneratorMiscTrait for OptionalWireRustGenerator<'_> {
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let Some(inner) = optional_dart_fn_inner(&self.mir.inner) else {
            return Default::default();
        };

        let optional_safe_ident = self.mir.safe_ident();
        let dart_fn_safe_ident = inner.safe_ident();
        let parameter_types = (inner.inputs.iter())
            .map(|x| x.rust_api_type())
            .collect_vec();
        let return_type_outer = inner.output.rust_api_type();
        let dyn_fn_type = format!(
            "Box<dyn Fn({}) -> flutter_rust_bridge::DartFnFuture<{}> + Send + Sync>",
            parameter_types.join(", "),
            return_type_outer,
        );

        Acc::new_common(
            format!(
                "fn decode_Optional_{optional_safe_ident}(
                    raw: Option<flutter_rust_bridge::DartOpaque>,
                ) -> Option<{dyn_fn_type}> {{
                    raw.map(|dart_opaque| {{
                        Box::new(decode_{dart_fn_safe_ident}(dart_opaque)) as {dyn_fn_type}
                    }})
                }}"
            )
            .into(),
        )
    }

    fn generate_wire_func_call_decode_wrapper(&self) -> Option<String> {
        optional_dart_fn_inner(&self.mir.inner)
            .map(|_| format!("decode_Optional_{}", self.mir.safe_ident()))
    }

    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        optional_dart_fn_inner(&self.mir.inner)
            .map(|inner| format!("Option<{}>", inner.get_delegate().rust_api_type(),))
    }
}

fn optional_dart_fn_inner(
    ty: &MirType,
) -> Option<&crate::codegen::ir::mir::ty::dart_fn::MirTypeDartFn> {
    match ty {
        MirType::DartFn(inner) => Some(inner),
        MirType::Boxed(inner) if !inner.exist_in_real_api => optional_dart_fn_inner(&inner.inner),
        _ => None,
    }
}
