use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::misc::comments::generate_codec_comments;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecOutputSpec;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::{
    WireDartCodecCstGenerator, WireDartCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::{
    DartApiImplClassMethod, WireDartOutputCode,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::Optional;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireDartCodecCstGeneratorContext,
    types: &[MirType],
) -> WireDartCodecOutputSpec {
    let mut inner = Acc::<Vec<WireDartOutputCode>>::default();
    inner += (types.iter())
        .map(|ty| generate_encode_func(ty, context))
        .collect();
    inner += (types.iter())
        .map(|ty| Acc::new_io(generate_encode_api_fill_to_wire_func(ty, context)))
        .collect();
    WireDartCodecOutputSpec { inner }
}

fn generate_encode_func(
    ty: &MirType,
    context: WireDartCodecCstGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let generator = WireDartCodecCstGenerator::new(ty.clone(), context);
    generator
        .generate_encode_func_body()
        .map(|raw_body, target: TargetOrCommon| {
            raw_body
                .map(|body| {
                    let signature = format!(
                        "{} cst_encode_{}({} raw)",
                        WireDartCodecCstGenerator::new(ty.clone(), context)
                            .dart_wire_type(target.as_target_or(Target::Io)),
                        ty.safe_ident(),
                        ApiDartGenerator::new(ty.clone(), context.as_api_dart_context())
                            .dart_api_type(),
                    );

                    WireDartOutputCode {
                        api_impl_class_methods: vec![DartApiImplClassMethod {
                            signature,
                            body: Some(format!(
                                "{}\n{body}",
                                generate_codec_comments(CodecMode::Cst),
                            )),
                        }],
                        ..Default::default()
                    }
                })
                .unwrap_or_default()
        })
}

// the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
// frb-coverage:ignore-start
fn generate_encode_api_fill_to_wire_func(
    ty: &MirType,
    context: WireDartCodecCstGeneratorContext,
) -> WireDartOutputCode {
    // frb-coverage:ignore-end
    if let Some(body) =
        WireDartCodecCstGenerator::new(ty.clone(), context).generate_encode_api_fill_to_wire_body()
    {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        let signature = format!(
            "void cst_api_fill_to_wire_{}({} apiObj, {} wireObj)",
            ty.safe_ident(),
            ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
            WireDartCodecCstGenerator::new(target_wire_type.clone(), context)
                .dart_wire_type(Target::Io),
        );

        WireDartOutputCode {
            api_impl_class_methods: vec![DartApiImplClassMethod {
                signature,
                body: Some(body),
            }],
            ..Default::default()
        }
    } else {
        Default::default()
    }
}
