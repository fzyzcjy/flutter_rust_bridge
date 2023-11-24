use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use convert_case::{Case, Casing};

pub(crate) fn generate_api_impl_opaque(
    ty: &IrType,
    context: WireDartGeneratorContext,
) -> Acc<WireDartOutputCode> {
    if !matches!(ty, IrType::RustOpaque(_)) {
        return Default::default();
    }

    let api_type = ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type();
    let api_type_camel = api_type.to_case(Case::Camel);

    let generate_platform_impl = |finalizer_type: &str, finalizer_arg: &str| {
        WireDartOutputCode {
            api_impl_body: format!(
                "late final {finalizer_type} {api_type_camel}Finalizer = {finalizer_type}({finalizer_arg});",
            ),
            ..Default::default()
        }
    };

    Acc {
        common: WireDartOutputCode {
            api_impl_body: format!(
                "
                OpaqueDropFnType get dropOpaque{ty_dart_api_type} => wire.drop_opaque_{safe_ident};
                OpaqueShareFnType get shareOpaque{ty_dart_api_type} => wire.share_opaque_{safe_ident};
                ",
                ty_dart_api_type = ApiDartGenerator::new(ty.clone(), context.as_api_dart_context())
                    .dart_api_type(),
                safe_ident = ty.safe_ident(),
            ),
            ..Default::default()
        },
        io: generate_platform_impl(
            "OpaqueTypeFinalizer",
            &format!("wire._drop_opaque_{api_type}Ptr"),
        ),
        wasm: generate_platform_impl(
            "Finalizer<PlatformPointer>",
            &format!("wire.drop_opaque_{api_type}"),
        ),
    }
}
