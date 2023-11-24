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
) -> Vec<Acc<WireDartOutputCode>> {
    if !matches!(ty, IrType::RustOpaque(_)) {
        return Default::default();
    }

    vec![
        generate_share_or_drop_opaque(ty, context, "share"),
        generate_share_or_drop_opaque(ty, context, "drop"),
        generate_opaque_finalizer(ty, context),
    ]
}

fn generate_share_or_drop_opaque(
    ty: &IrType,
    context: WireDartGeneratorContext,
    op_name: &str,
) -> Acc<WireDartOutputCode> {
    let ty_dart_api_type =
        ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type();
    let op_name_pascal = op_name.to_case(Case::Pascal);
    let safe_ident = ty.safe_ident();

    let definition = format!("Opaque{op_name_pascal}FnType get {op_name}Opaque{ty_dart_api_type}");

    Acc {
        common: WireDartOutputCode {
            api_body: format!("{definition};\n\n"),
            api_impl_body: format!("{definition} => wire.{op_name}_opaque_{safe_ident};\n\n"),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn generate_opaque_finalizer(
    ty: &IrType,
    context: WireDartGeneratorContext,
) -> Acc<WireDartOutputCode> {
    let ty_dart_api_type =
        ApiDartGenerator::new(ty.clone(), context.as_api_dart_context()).dart_api_type();
    let ty_dart_api_type_camel = ty_dart_api_type.to_case(Case::Camel);
    let ty_safe_ident = ty.safe_ident();
    let field_name = format!("{ty_dart_api_type_camel}Finalizer");

    let generate_platform_impl = |finalizer_arg: &str| WireDartOutputCode {
        api_impl_body: format!(
            "late final {field_name} = OpaqueTypeFinalizer({finalizer_arg});\n\n",
        ),
        ..Default::default()
    };

    Acc {
        common: WireDartOutputCode {
            api_body: format!("OpaqueTypeFinalizer get {field_name};\n\n"),
            ..Default::default()
        },
        io: generate_platform_impl(&format!("wire._drop_opaque_{ty_safe_ident}Ptr")),
        wasm: generate_platform_impl(&format!("wire.drop_opaque_{ty_safe_ident}")),
        ..Default::default()
    }
}
