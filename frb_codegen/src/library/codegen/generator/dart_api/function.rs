use crate::codegen::generator::dart_api::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::dart_api::misc::generate_dart_comments;
use crate::codegen::ir::func::{IrFunc, IrFuncMode};
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Primitive, StructRef};
use convert_case::{Case, Casing};
use itertools::Itertools;

#[derive(Debug)]
pub(crate) struct GeneratedApiFunc {
    pub(crate) func_comments: String,
    pub(crate) func_signature: String,
    pub(crate) companion_field_signature: String,
}

pub(crate) fn generate_func(func: &IrFunc, dart_enums_style: bool) -> GeneratedApiFunc {
    let params = generate_params(func, dart_enums_style);

    let func_expr = format!(
        "{} {}({{ {} }})",
        func.mode.dart_return_type(&func.output.dart_api_type()),
        func.name.to_case(Case::Camel),
        params.join(","),
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

fn generate_params(func: &IrFunc, dart_enums_style: bool) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .map(|input| {
            let required = generate_field_required_modifier(input);
            let r#default = generate_field_default(input, false, dart_enums_style);
            let type_str = input.ty.dart_api_type();
            let name_str = input.name.dart_style();
            format!("{required}{type_str} {name_str} {default}")
        })
        .collect_vec();
    ans.push("dynamic hint".to_owned());
    ans
}
