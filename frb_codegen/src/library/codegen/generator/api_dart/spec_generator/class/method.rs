use crate::codegen::generator::api_dart;
use crate::codegen::generator::api_dart::spec_generator::class::proxy_variant;
use crate::codegen::generator::api_dart::spec_generator::function::{
    compute_params_str, ApiDartGeneratedFunction, ApiDartGeneratedFunctionParam,
};
use crate::codegen::generator::api_dart::spec_generator::misc::generate_dart_comments;
use crate::codegen::ir::mir::func::{
    MirFunc, MirFuncAccessorMode, MirFuncArgMode, MirFuncDefaultConstructorMode, MirFuncImplMode,
    MirFuncImplModeDartOnly, MirFuncOwnerInfo, MirFuncOwnerInfoMethod, MirFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::if_then_some;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use crate::utils::dart_keywords;
use convert_case::{Case, Casing};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub(crate) struct GenerateApiMethodConfig {
    pub mode_static: GenerateApiMethodMode,
    pub mode_non_static: GenerateApiMethodMode,
}

impl GenerateApiMethodConfig {
    pub(crate) const COMBINED: GenerateApiMethodConfig = GenerateApiMethodConfig {
        mode_static: GenerateApiMethodMode::DeclAndImpl,
        mode_non_static: GenerateApiMethodMode::DeclAndImpl,
    };

    fn get(&self, method_mode: &MirFuncOwnerInfoMethodMode) -> GenerateApiMethodMode {
        match method_mode {
            MirFuncOwnerInfoMethodMode::Static => self.mode_static,
            MirFuncOwnerInfoMethodMode::Instance => self.mode_non_static,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum GenerateApiMethodMode {
    Nothing,
    DeclOnly,
    DeclAndImpl,
}

pub(crate) struct GeneratedApiMethods {
    pub(crate) num_methods: usize,
    pub(crate) code: String,
    pub(crate) header: DartHeaderCode,
}

struct GeneratedApiMethod {
    code: String,
    header: DartHeaderCode,
}

pub(crate) fn generate_api_methods(
    owner_ty: &MirType,
    context: ApiDartGeneratorContext,
    config: &GenerateApiMethodConfig,
    dart_class_name: &str,
) -> GeneratedApiMethods {
    let query_class_name = compute_class_name_for_querying_methods(owner_ty, context);
    let methods = get_methods_of_ty(&query_class_name, &context.mir_pack.funcs_all, context)
        .iter()
        .filter_map(|func| generate_api_method(func, context, config, dart_class_name))
        .collect_vec();
    GeneratedApiMethods {
        num_methods: methods.len(),
        code: methods.iter().map(|x| x.code.clone()).join("\n"),
        header: (methods.iter().map(|x| x.header.clone())).fold(Default::default(), |a, b| a + b),
    }
}

fn compute_class_name_for_querying_methods(
    ty: &MirType,
    context: ApiDartGeneratorContext,
) -> String {
    ApiDartGenerator::new(ty.clone(), context).dart_api_type() // match ty {
                                                               //     MirType::EnumRef(ty) => ty.ident.0.clone(),
                                                               //     MirType::StructRef(ty) => ty.ident.0.clone(),
                                                               //     MirType::TraitDef(ty) => ty.name.clone(),
                                                               //     MirType::Delegate(MirTypeDelegate::ProxyVariant(ty)) => {
                                                               //         compute_class_name_for_querying_methods(&ty.inner)
                                                               //     }
                                                               //     MirType::Delegate(MirTypeDelegate::Lifetimeable(ty)) => {
                                                               //         compute_class_name_for_querying_methods(&MirType::Delegate(
                                                               //             MirTypeDelegate::RustAutoOpaqueExplicit(ty.delegate.clone()),
                                                               //         ))
                                                               //     }
                                                               //     MirType::Delegate(MirTypeDelegate::RustAutoOpaqueExplicit(ty)) => {
                                                               //         compute_class_name_for_querying_methods(&MirType::RustOpaque(ty.inner.clone()))
                                                               //     }
                                                               //     MirType::RustAutoOpaqueImplicit(ty) => {
                                                               //         compute_class_name_for_querying_methods(&MirType::RustOpaque(ty.inner.clone()))
                                                               //     }
                                                               //     MirType::RustOpaque(ty) => {
                                                               //         lazy_static! {
                                                               //             static ref FILTER: Regex =
                                                               //                 Regex::new(r"^flutter_rust_bridge::for_generated::RustAutoOpaqueInner<(.*)>$")
                                                               //                     .unwrap();
                                                               //         }
                                                               //         let name = FILTER.replace_all(&ty.inner.0.with_static_lifetime(), "$1").to_string();
                                                               //         NamespacedName::new(ty.namespace.clone(), name)
                                                               //     }
                                                               //     _ => panic!("compute_query_class_name see unknown ty={ty:?}"),
                                                               // }
}

// TODO move
pub(crate) fn dart_constructor_postfix(
    name: &str,
    all_funcs: &[MirFunc],
    context: ApiDartGeneratorContext,
) -> &'static str {
    if has_default_dart_constructor(name, all_funcs, context) {
        ".raw"
    } else {
        ""
    }
}

fn has_default_dart_constructor(
    name: &str,
    all_funcs: &[MirFunc],
    context: ApiDartGeneratorContext,
) -> bool {
    get_methods_of_ty(name, all_funcs, context).iter().any(|m| {
        m.default_constructor_mode() == Some(MirFuncDefaultConstructorMode::DartConstructor)
    })
}

fn get_methods_of_ty<'a>(
    name: &str,
    all_funcs: &'a [MirFunc],
    context: ApiDartGeneratorContext,
) -> Vec<&'a MirFunc> {
    (all_funcs.iter())
        .filter(|f| {
            matches!(
                &f.owner, MirFuncOwnerInfo::Method(m)
                // if m.owner_ty_name().as_ref() == Some(name)
                if ApiDartGenerator::new(m.owner_ty.clone(), context).dart_api_type() == name
            )
        })
        .collect_vec()
}

fn generate_api_method(
    func: &MirFunc,
    context: ApiDartGeneratorContext,
    config: &GenerateApiMethodConfig,
    dart_class_name: &str,
) -> Option<GeneratedApiMethod> {
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
        dart_class_name,
    );

    let maybe_implementation = match config.get(&method_info.mode) {
        GenerateApiMethodMode::Nothing => return None,
        GenerateApiMethodMode::DeclOnly => "".to_owned(),
        GenerateApiMethodMode::DeclAndImpl => format!(
            "=>{}",
            generate_implementation(func, context, method_info, &params)
        ),
    };

    let code = format!("{comments}{signature}{maybe_implementation};\n\n");

    Some(GeneratedApiMethod {
        code,
        header: api_dart_func.header,
    })
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

#[allow(clippy::too_many_arguments)]
fn generate_signature(
    func: &MirFunc,
    method_info: &MirFuncOwnerInfoMethod,
    func_params: &[ApiDartGeneratedFunctionParam],
    default_constructor_mode: Option<MirFuncDefaultConstructorMode>,
    api_dart_func: &ApiDartGeneratedFunction,
    method_name: &str,
    dart_class_name: &str,
) -> String {
    let is_static_method = method_info.mode == MirFuncOwnerInfoMethodMode::Static;
    let maybe_static = if is_static_method { "static" } else { "" };
    let (return_type, func_params, maybe_accessor) = match func.accessor {
        Some(MirFuncAccessorMode::Getter) => (
            api_dart_func.func_return_type.as_ref(),
            "".to_owned(),
            "get",
        ),
        Some(MirFuncAccessorMode::Setter) => (
            "",
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
            api_dart_func.func_return_type.as_ref(),
            format!("({})", compute_params_str(func_params, func.arg_mode)),
            "",
        ),
    };

    if default_constructor_mode == Some(MirFuncDefaultConstructorMode::DartConstructor) {
        return format!("factory {dart_class_name}{func_params}");
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
        dart_keywords::escape(
            method_info
                .actual_method_dart_name
                .as_ref()
                .unwrap_or(&method_info.actual_method_name)
                .to_case(Case::Camel),
        )
    }
}

fn generate_implementation(
    func: &MirFunc,
    context: ApiDartGeneratorContext,
    method_info: &MirFuncOwnerInfoMethod,
    params: &[ApiDartGeneratedFunctionParam],
) -> String {
    match &func.impl_mode {
        MirFuncImplMode::Normal => {
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
        MirFuncImplMode::NoImpl => "should_not_reach_here".to_owned(),
        MirFuncImplMode::DartOnly(inner) => match inner {
            MirFuncImplModeDartOnly::CreateProxyVariant(inner) => {
                proxy_variant::compute_func_implementation(inner, context, func.mode)
            }
        },
    }
}
