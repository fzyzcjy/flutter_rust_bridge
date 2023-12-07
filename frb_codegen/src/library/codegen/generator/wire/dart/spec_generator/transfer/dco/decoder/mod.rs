use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::WireDartTransferCstGenerator;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::{
    WireDartTransferDcoGenerator, WireDartTransferDcoGeneratorContext,
};
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpecTransferDcoDecoder {
    pub(crate) impl_decode: Acc<Vec<WireDartOutputCode>>,
}

pub(crate) fn generate(
    context: WireDartTransferDcoGeneratorContext,
    cache: &IrPackComputedCache,
) -> WireDartOutputSpecTransferDcoDecoder {
    WireDartOutputSpecTransferDcoDecoder {
        impl_decode: cache
            .distinct_output_types
            .iter()
            .map(|ty| Acc::new_common(generate_impl_decode(ty, context)))
            .collect(),
    }
}

fn generate_impl_decode(
    ty: &IrType,
    context: WireDartTransferDcoGeneratorContext,
) -> WireDartOutputCode {
    let body = WireDartTransferDcoGenerator::new(ty.clone(), context).generate_impl_decode_body();
    let api_impl_body = format!(
        "{dart_api_type} _dco_decode_{safe_ident}(dynamic raw) {{
            {body}
        }}
        ",
        dart_api_type =
            ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
        safe_ident = ty.safe_ident(),
    );
    WireDartOutputCode {
        api_impl_body,
        ..Default::default()
    }
}
