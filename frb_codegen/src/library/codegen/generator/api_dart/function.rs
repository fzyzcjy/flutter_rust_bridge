use crate::codegen::generator::api_dart::base::{ApiDartGenerator, ApiDartGeneratorContext};
use crate::codegen::generator::api_dart::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::misc::{
    generate_dart_comments, generate_function_dart_return_type,
};
use crate::codegen::ir::func::IrFunc;
use crate::library::codegen::generator::api_dart::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

#[derive(Debug)]
pub(crate) struct GeneratedApiFunc {
    pub(crate) func_comments: String,
    pub(crate) func_signature: String,
    pub(crate) companion_field_signature: String,
}

pub(crate) fn generate_func(
    func: &IrFunc,
    context: &ApiDartGeneratorContext,
    dart_enums_style: bool,
) -> GeneratedApiFunc {
    let params = generate_params(func, context, dart_enums_style).join(", ");

    let func_expr = format!(
        "{func_return_type} {func_name}({{ {params} }})",
        func_name = func.name.to_case(Case::Camel),
        func_return_type = generate_function_dart_return_type(
            &func.mode,
            &ApiDartGenerator::new(func.output.clone(), context.clone()).dart_api_type()
        ),
    );
    let func_signature = format!("{func_expr};");

    let func_comments = generate_dart_comments(&func.comments);

    let const_meta_field_name = format!("k{}ConstMeta", func.name.to_case(Case::Pascal));
    let companion_field_signature =
        format!("FlutterRustBridgeTaskConstMeta get {const_meta_field_name};");

    GeneratedApiFunc {
        func_comments,
        func_signature,
        companion_field_signature,
    }
}

fn generate_params(
    func: &IrFunc,
    context: &ApiDartGeneratorContext,
    dart_enums_style: bool,
) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .map(|input| {
            let required = generate_field_required_modifier(input);
            let r#default = generate_field_default(input, false, dart_enums_style);
            let type_str = ApiDartGenerator::new(input.ty.clone(), context.clone()).dart_api_type();
            let name_str = input.name.dart_style();
            format!("{required}{type_str} {name_str} {default}")
        })
        .collect_vec();
    ans.push("dynamic hint".to_owned());
    ans
}
