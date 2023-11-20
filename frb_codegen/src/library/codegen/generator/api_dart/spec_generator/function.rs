use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_function_dart_return_type,
};
use crate::codegen::ir::func::IrFunc;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

#[derive(Debug)]
pub(crate) struct GeneratedApiFunc {
    pub(crate) func_comments: String,
    pub(crate) func_expr: String,
    pub(crate) func_impl: String,
    // pub(crate) companion_field_signature: String,
}

pub(crate) fn generate_func(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    dart_enums_style: bool,
) -> GeneratedApiFunc {
    let params = generate_params(func, context, dart_enums_style).join(", ");

    let func_expr = format!(
        "{func_return_type} {func_name}({{ {params} }})",
        func_name = func.name.to_case(Case::Camel),
        func_return_type = generate_function_dart_return_type(
            &func.mode,
            &ApiDartGenerator::new(func.output.clone(), context).dart_api_type()
        ),
    );

    let func_comments = generate_dart_comments(&func.comments);

    let func_impl = generate_func_impl(func, &context.config.dart_entrypoint_class_name);

    // TODO
    // let const_meta_field_name = format!("k{}ConstMeta", func.name.to_case(Case::Pascal));
    // let companion_field_signature =
    //     format!("FlutterRustBridgeTaskConstMeta get {const_meta_field_name} => TODO;");

    GeneratedApiFunc {
        func_comments,
        func_expr,
        func_impl,
        // TODO
        // companion_field_signature,
    }
}

fn generate_params(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    dart_enums_style: bool,
) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .map(|input| {
            let required = generate_field_required_modifier(input);
            let r#default = generate_field_default(input, false, dart_enums_style);
            let type_str = ApiDartGenerator::new(input.ty.clone(), context).dart_api_type();
            let name_str = input.name.dart_style();
            format!("{required}{type_str} {name_str} {default}")
        })
        .collect_vec();
    ans.push("dynamic hint".to_owned());
    ans
}

fn generate_func_impl(func: &IrFunc, dart_entrypoint_class_name: &str) -> String {
    let param_forwards = func
        .inputs
        .iter()
        .map(|input| format!("{name}: {name}", name = input.name.dart_style()))
        .join(", ");
    format!("{dart_entrypoint_class_name}.instance.dispatcher.simpleAdder({param_forwards})")
}
