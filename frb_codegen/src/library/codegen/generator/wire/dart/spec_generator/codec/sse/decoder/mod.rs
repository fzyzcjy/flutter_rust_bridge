use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::lang::Lang::DartLang;
use crate::codegen::generator::codec::sse::ty::{CodecSseTy, CodecSseTyContext};
use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecOutputSpec;
use crate::codegen::generator::wire::dart::spec_generator::codec::sse::base::WireDartCodecSseGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::ty::IrType;

pub(crate) fn generate(
    context: WireDartCodecSseGeneratorContext,
    types: &[IrType],
) -> WireDartCodecOutputSpec {
    let mut inner = Default::default();
    inner += (types.iter())
        .map(|ty| generate_for_type(ty, context))
        .collect();
    WireDartCodecOutputSpec { inner }
}

fn generate_for_type(
    ty: &IrType,
    context: WireDartCodecSseGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let dart_api_type =
        ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type();
    let safe_ident = ty.safe_ident();
    let body =
        CodecSseTy::new(ty, CodecSseTyContext::new(context.ir_pack)).generate_decode(&DartLang);

    Acc::new_common(
        format!(
            "
            {dart_api_type} _sse_decode_{safe_ident}(Serializer serializer) {{
                {body}
            }}
            "
        )
        .into(),
    )
}
