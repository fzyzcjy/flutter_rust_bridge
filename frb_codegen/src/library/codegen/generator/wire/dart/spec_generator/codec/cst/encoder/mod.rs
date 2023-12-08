use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::codec::base::WireDartCodecOutputSpec;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::{
    WireDartCodecCstGenerator, WireDartCodecCstGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Optional;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use serde::Serialize;

mod misc;
pub(crate) mod ty;

pub(crate) fn generate(
    context: WireDartCodecCstGeneratorContext,
    types: &[IrType],
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
    ty: &IrType,
    context: WireDartCodecCstGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let generator = WireDartCodecCstGenerator::new(ty.clone(), context);
    generator
        .encode_func_body()
        .map(|raw_body, target: TargetOrCommon| {
            raw_body
                .map(|raw_body| {
                    let code = format!(
                        "{} cst_encode_{}({} raw) {{
                            {raw_body}
                        }}",
                        WireDartCodecCstGenerator::new(ty.clone(), context)
                            .dart_wire_type(target.as_target_or(Target::Io)),
                        ty.safe_ident(),
                        ApiDartGenerator::new(ty.clone(), context.as_api_dart_context())
                            .dart_api_type(),
                    );

                    let (body, api_impl_body) = match target {
                        TargetOrCommon::Common => (code, "".into()),
                        TargetOrCommon::Io | TargetOrCommon::Wasm => {
                            ("".into(), format!("@protected\n{code}"))
                        }
                    };

                    WireDartOutputCode {
                        api_impl_body,
                        body,
                        ..Default::default()
                    }
                })
                .unwrap_or_default()
        })
}

fn generate_encode_api_fill_to_wire_func(
    ty: &IrType,
    context: WireDartCodecCstGeneratorContext,
) -> WireDartOutputCode {
    if let Some(body) =
        WireDartCodecCstGenerator::new(ty.clone(), context).encode_api_fill_to_wire_body()
    {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        WireDartOutputCode {
            api_impl_body: format!(
                "void _cst_api_fill_to_wire_{}({} apiObj, {} wireObj) {{
                    {body}
                }}",
                ty.safe_ident(),
                ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type(),
                WireDartCodecCstGenerator::new(target_wire_type.clone(), context)
                    .dart_wire_type(Target::Io),
            ),
            ..Default::default()
        }
    } else {
        Default::default()
    }
}
