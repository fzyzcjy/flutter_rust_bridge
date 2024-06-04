use crate::codegen::generator::api_dart;
use crate::codegen::generator::api_dart::spec_generator::function::{
    compute_params_str, ApiDartGeneratedFunction, ApiDartGeneratedFunctionParam,
};
use crate::codegen::generator::api_dart::spec_generator::misc::generate_dart_comments;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncAccessorMode, MirFuncArgMode, MirFuncDefaultConstructorMode, MirFuncOwnerInfo,
    MirFuncOwnerInfoMethod, MirFuncOwnerInfoMethodMode,
};
use crate::if_then_some;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::utils::namespace::NamespacedName;
use convert_case::{Case, Casing};
use itertools::Itertools;
use std::cmp::min;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum GenerateApiMethodMode {
    SeparatedDecl,
    SeparatedImpl,
    Combined,
}

pub(crate) fn generate_api_methods(
    generalized_class_name: &NamespacedName,
    context: ApiDartGeneratorContext,
    mode: GenerateApiMethodMode,
) -> Vec<String> {
    get_methods_of_enum_or_struct(generalized_class_name, &context.mir_pack.funcs)
        .iter()
        .map(|func| generate_api_method(func, context, mode))
        .collect_vec()
}

// TODO move
pub(crate) fn dart_constructor_postfix(
    name: &NamespacedName,
    all_funcs: &[MirFunc],
) -> &'static str {
    if has_default_dart_constructor(name, all_funcs) {
        ".raw"
    } else {
        ""
    }
}

fn has_default_dart_constructor(name: &NamespacedName, all_funcs: &[MirFunc]) -> bool {
    get_methods_of_enum_or_struct(name, all_funcs)
        .iter()
        .any(|m| {
            m.default_constructor_mode() == Some(MirFuncDefaultConstructorMode::DartConstructor)
        })
}

fn get_methods_of_enum_or_struct<'a>(
    name: &NamespacedName,
    all_funcs: &'a [MirFunc],
) -> Vec<&'a MirFunc> {
    (all_funcs.iter())
        .filter(|f| matches!(&f.owner, MirFuncOwnerInfo::Method(m) if m.owner_ty_name().as_ref() == Some(name)))
        .collect_vec()
}

fn generate_api_method(
    func: &MirFunc,
    context: ApiDartGeneratorContext,
    mode: GenerateApiMethodMode,
) -> String {
    let api_dart_func = api_dart::spec_generator::function::generate(func, context).unwrap();

    let method_info =
        if_then_some!(let MirFuncOwnerInfo::Method(info) = &func.owner , info).unwrap();
    let default_constructor_mode = func.default_constructor_mode();

    let skip_names = compute_skip_names(method_info);
    let params = (api_dart_func.func_params.iter())
        .filter(|param| !skip_names.contains(&&param.name_str[..]))
        .cloned()
        .collect_vec();
    let method_name = generate_method_name(method_info, default_constructor_mode);

    let comments = generate_comments(func, default_constructor_mode);
    let signature = generate_signature(
        func,
        method_info,
        &params,
        default_constructor_mode,
        &api_dart_func,
        &method_name,
    );

    let maybe_implementation = match mode {
        GenerateApiMethodMode::SeparatedDecl => generate_implementation_separated_decl_forward(
            func,
            default_constructor_mode,
            &method_name,
        ),
        GenerateApiMethodMode::SeparatedImpl | GenerateApiMethodMode::Combined => Some(
            generate_implementation_normal(func, context, method_info, &params),
        ),
    };
    let maybe_implementation = (maybe_implementation.map(|x| format!("=>{x}"))).unwrap_or_default();

    format!("{comments}{signature}{maybe_implementation};\n\n")
}

fn compute_skip_names(method_info: &MirFuncOwnerInfoMethod) -> Vec<&'static str> {
    let mut ans = vec![];
    if method_info.mode != MirFuncOwnerInfoMethodMode::Static {
        ans.push("that");
    }
    // if func.accessor.is_some() {
    //     ans.push("hint");
    // }
    ans
}

fn generate_comments(
    func: &MirFunc,
    default_constructor_mode: Option<MirFuncDefaultConstructorMode>,
) -> String {
    let mut ans = String::new();
    if default_constructor_mode == Some(MirFuncDefaultConstructorMode::StaticMethod) {
        ans += "  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.\n";
    }
    ans += &generate_dart_comments(&func.comments);
    ans
}

fn generate_signature(
    func: &MirFunc,
    method_info: &MirFuncOwnerInfoMethod,
    func_params: &[ApiDartGeneratedFunctionParam],
    default_constructor_mode: Option<MirFuncDefaultConstructorMode>,
    api_dart_func: &ApiDartGeneratedFunction,
) -> String {
    let is_static_method = method_info.mode == MirFuncOwnerInfoMethodMode::Static;
    let maybe_static = if is_static_method { "static" } else { "" };
    let return_type = &api_dart_func.func_return_type;
    let (func_params, maybe_accessor) = match func.accessor {
        Some(MirFuncAccessorMode::Getter) => ("".to_owned(), "get"),
        Some(MirFuncAccessorMode::Setter) => (
            // TODO: merge with below
            format!(
                "({})",
                (func_params.iter())
                    .map(|x| x.full(MirFuncArgMode::Positional))
                    .join(", ")
            ),
            "set",
        ),
        None => (
            format!("({})", compute_params_str(func_params, func.arg_mode)),
            "",
        ),
    };

    if default_constructor_mode == Some(MirFuncDefaultConstructorMode::DartConstructor) {
        return format!("factory {return_type}{func_params}");
    }

    format!("{maybe_static} {return_type} {maybe_accessor} {method_name}{func_params}")
}

fn generate_method_name(
    method_info: &MirFuncOwnerInfoMethod,
    default_constructor_mode: Option<MirFuncDefaultConstructorMode>,
) -> String {
    if default_constructor_mode.is_some() {
        "newInstance".to_owned()
    } else {
        (method_info
            .actual_method_dart_name
            .as_ref()
            .unwrap_or(&method_info.actual_method_name))
        .to_case(Case::Camel)
    }
}

fn generate_implementation_normal(
    func: &MirFunc,
    context: ApiDartGeneratorContext,
    method_info: &MirFuncOwnerInfoMethod,
    params: &[ApiDartGeneratedFunctionParam],
) -> String {
    let dart_entrypoint_class_name = &context.config.dart_entrypoint_class_name;
    let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

    let func_name = func.name_dart_wire();

    let arg_names = params
        .iter()
        .map(|x| format!("{name}: {name}", name = x.name_str))
        .join(", ");

    if method_info.mode == MirFuncOwnerInfoMethodMode::Static {
        format!("{dart_api_instance}.{func_name}({arg_names})")
    } else {
        let extra_arg_name = func.inputs[0].inner.name.dart_style();
        format!("{dart_api_instance}.{func_name}({extra_arg_name}: this, {arg_names})")
    }
}

fn generate_implementation_separated_decl_forward(
    func: &MirFunc,
    default_constructor_mode: Option<MirFuncDefaultConstructorMode>,
    method_info: &MirFuncOwnerInfoMethod,
    method_name: &str,
) -> Option<String> {
    if method_info.mode != MirFuncOwnerInfoMethodMode::Static {
        return None;
    }

    let owner_ty_name = method_info.owner_ty_name().unwrap().name;

    let maybe_method_name =
        if default_constructor_mode == Some(MirFuncDefaultConstructorMode::DartConstructor) {
            "".to_owned()
        } else {
            format!(".{method_name}")
        };

    Some(format!("{owner_ty_name}Impl{maybe_method_name}({params})"))
}
