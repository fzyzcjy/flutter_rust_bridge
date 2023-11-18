use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use itertools::Itertools;

pub(crate) mod ty;

pub(crate) struct WireDartOutputSpecWire2api {
    impl_wire2api: Acc<Vec<WireDartOutputCode>>,
}

pub(super) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecWire2api {
    WireDartOutputSpecWire2api {
        impl_wire2api: cache
            .distinct_output_types
            .iter()
            .map(|ty| generate_impl_wire2api_func(ty, ir_pack, dart_api_class_name, config))
            .collect(),
    }
}

fn generate_impl_wire2api_func(
    ty: &IrType,
    ir_pack: &IrPack,
    _dart_api_class_name: &str,
    config: &Opts,
) -> String {
    let body = TypeDartGenerator::new(ty.clone(), ir_pack, config).wire2api_body();
    format!(
        "{} _wire2api_{}({} raw) {{
            {}
        }}
        ",
        ty.dart_api_type(),
        ty.safe_ident(),
        ty.dart_param_type(),
        body,
    )
}
