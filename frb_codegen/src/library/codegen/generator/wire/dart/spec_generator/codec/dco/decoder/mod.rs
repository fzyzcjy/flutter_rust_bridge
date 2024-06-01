use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::comments::generate_codec_comments;
use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecOutputSpec;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::{
    WireDartCodecDcoGenerator, WireDartCodecDcoGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::{
    DartApiImplClassMethod, WireDartOutputCode,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireDartCodecDcoGeneratorContext,
    types: &[MirType],
) -> WireDartCodecOutputSpec {
    WireDartCodecOutputSpec {
        inner: (types.iter())
            .map(|ty| Acc::new_common(generate_impl_decode(ty, context)))
            .collect(),
    }
}

fn generate_impl_decode(
    ty: &MirType,
    context: WireDartCodecDcoGeneratorContext,
) -> WireDartOutputCode {
    let body = WireDartCodecDcoGenerator::new(ty.clone(), context).generate_impl_decode_body();
    let signature = format!(
        "{dart_api_type} dco_decode_{safe_ident}(dynamic raw)",
        dart_api_type =
            ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
        safe_ident = ty.safe_ident(),
    );
    WireDartOutputCode {
        api_impl_class_methods: vec![DartApiImplClassMethod {
            signature,
            body: Some(format!(
                "{}\n{body}",
                generate_codec_comments(CodecMode::Dco),
            )),
        }],
        ..Default::default()
    }
}
