mod ty;
mod ty_boxed;
mod ty_delegate;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_primitive;
mod ty_primitive_list;
mod ty_struct;

use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;

use itertools::Itertools;
pub use ty::*;
pub use ty_boxed::*;
pub use ty_delegate::*;
pub use ty_enum::*;
pub use ty_general_list::*;
pub use ty_optional::*;
pub use ty_primitive::*;
pub use ty_primitive_list::*;
pub use ty_struct::*;

use convert_case::{Case, Casing};
use log::debug;

use crate::ir::IrType::*;
use crate::method_utils::{FunctionName, MethodNamingUtil};
use crate::others::*;
use crate::target::Target::*;
use crate::target::{Acc, Target};
use crate::{ir::*, Opts};

pub struct Output {
    pub file_prelude: DartBasicCode,
    pub decl_code: DartBasicCode,
    pub impl_code: Acc<DartBasicCode>,
    pub needs_freezed: bool,
}

pub fn generate(ir_file: &IrFile, config: &Opts, wasm_funcs: &[IrFuncLike]) -> Output {
    let dart_api_class_name = &config.dart_api_class_name();
    let dart_output_file_root = config.dart_output_root().expect("Internal error");
    let spec = DartApiSpec::from(ir_file, config, wasm_funcs);
    let DartApiSpec {
        dart_funcs,
        dart_structs,
        ..
    } = &spec;
    let needs_freezed = spec.needs_freezed;
    let common_header = generate_common_header();

    let decl_code = generate_dart_declaration_code(
        &common_header,
        generate_freezed_header(dart_output_file_root, needs_freezed),
        generate_import_header(get_dart_imports(ir_file)),
        generate_dart_declaration_body(dart_api_class_name, dart_funcs, dart_structs),
    );

    let impl_code = generate_dart_implementation_code(
        &common_header,
        generate_dart_implementation_body(&spec, config),
    );

    let file_prelude = generate_file_prelude();

    Output {
        file_prelude,
        decl_code,
        impl_code,
        needs_freezed,
    }
}

struct DartApiSpec {
    dart_funcs: Vec<GeneratedApiFunc>,
    dart_structs: Vec<String>,
    dart_api2wire_funcs: Acc<String>,
    dart_api_fill_to_wire_funcs: Vec<String>,
    dart_wire2api_funcs: Vec<String>,
    dart_wasm_funcs: Vec<String>,
    dart_wasm_module: Option<String>,
    needs_freezed: bool,
}

impl DartApiSpec {
    fn from(ir_file: &IrFile, config: &Opts, extra_funcs: &[IrFuncLike]) -> Self {
        let dart_api_class_name = config.dart_api_class_name();
        let dart_wire_class_name = config.dart_wire_class_name();
        let distinct_types = ir_file.distinct_types(true, true);
        let distinct_input_types = ir_file.distinct_types(true, false);
        let distinct_output_types = ir_file.distinct_types(false, true);
        debug!("distinct_input_types={:?}", distinct_input_types);
        debug!("distinct_output_types={:?}", distinct_output_types);

        let dart_structs = distinct_types
            .iter()
            .map(|ty| {
                TypeDartGenerator::new(
                    ty.clone(),
                    ir_file,
                    // Some(dart_api_class_name.to_string()),
                    config,
                )
                .structs()
            })
            .collect::<Vec<_>>();
        let dart_api2wire_funcs = distinct_input_types
            .iter()
            .map(|ty| generate_api2wire_func(ty, ir_file, config))
            .collect::<Acc<_>>()
            .join("\n");
        let dart_funcs = ir_file
            .funcs
            .iter()
            .map(|f| generate_api_func(f, ir_file, &dart_api2wire_funcs.common))
            .collect::<Vec<_>>();
        let dart_api_fill_to_wire_funcs = distinct_input_types
            .iter()
            .map(|ty| generate_api_fill_to_wire_func(ty, ir_file, config))
            .collect::<Vec<_>>();
        let dart_wire2api_funcs = distinct_output_types
            .iter()
            .map(|ty| generate_wire2api_func(ty, ir_file, &dart_api_class_name, config))
            .collect::<Vec<_>>();

        let ir_wasm_func_exports = config.wasm_enabled.then(|| {
            ir_file
                .funcs
                .iter()
                .map(|fun| IrFuncLike::from_ir(fun, Target::Wasm))
                .chain(extra_funcs.iter().cloned())
                .collect::<Vec<_>>()
        });
        let dart_wasm_funcs = if let Some(exports) = &ir_wasm_func_exports {
            exports.iter().map(generate_wasm_wire_func_decl).collect()
        } else {
            Default::default()
        };
        let dart_wasm_module = (ir_wasm_func_exports.as_ref()).map(|exports| {
            generate_wasm_module(exports, &dart_wire_class_name, &config.dart_wasm_module())
        });

        let needs_freezed = distinct_types.iter().any(|ty| match ty {
            EnumRef(_) => true,
            StructRef(st) => st.freezed,
            _ => false,
        });

        DartApiSpec {
            dart_funcs,
            dart_structs,
            dart_api2wire_funcs,
            dart_api_fill_to_wire_funcs,
            dart_wire2api_funcs,
            dart_wasm_funcs,
            dart_wasm_module,
            needs_freezed,
        }
    }
}

fn generate_freezed_header(dart_output_file_root: &str, needs_freezed: bool) -> DartBasicCode {
    if needs_freezed {
        DartBasicCode {
            import: "import 'package:freezed_annotation/freezed_annotation.dart' hide protected;"
                .to_string(),
            part: format!("part '{}.freezed.dart';", dart_output_file_root),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    }
}

fn generate_import_header(imports: HashSet<&IrDartImport>) -> DartBasicCode {
    if !imports.is_empty() {
        DartBasicCode {
            import: imports
                .iter()
                .map(|it| match &it.alias {
                    Some(alias) => format!("import '{}' as {};", it.uri, alias),
                    _ => format!("import '{}';", it.uri),
                })
                .collect::<Vec<_>>()
                .join("\n"),
            part: "".to_string(),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    }
}

fn generate_common_header() -> DartBasicCode {
    DartBasicCode {
        import: "import 'dart:convert';
            import 'dart:async';
            import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';"
            .to_string(),
        part: "".to_string(),
        body: "".to_string(),
    }
}

fn get_dart_imports(ir_file: &IrFile) -> HashSet<&IrDartImport> {
    ir_file
        .struct_pool
        .values()
        .flat_map(|s| s.dart_metadata.iter().flat_map(|it| &it.library))
        .collect()
}

fn generate_wasm_module<'a>(
    funcs: impl IntoIterator<Item = &'a IrFuncLike>,
    dart_wire_class_name: &str,
    dart_wasm_module_name: &str,
) -> String {
    format!(
        "class {cls} extends FlutterRustBridgeWasmWireBase<{wasm}> {{
            {cls}(FutureOr<WasmModule> module) : super(WasmModule.cast<{wasm}>(module));
            
            {}
        }}
        ",
        funcs
            .into_iter()
            .map(generate_wasm_wire_func_method)
            .collect::<Vec<_>>()
            .join("\n\n"),
        cls = dart_wire_class_name,
        wasm = dart_wasm_module_name,
    )
}

fn generate_dart_declaration_body(
    dart_api_class_name: &str,
    dart_funcs: &[GeneratedApiFunc],
    dart_structs: &[String],
) -> String {
    format!(
        "abstract class {} {{
            {}
        }}

        {}
        ",
        dart_api_class_name,
        dart_funcs
            .iter()
            .map(|func| format!(
                "{}{}\n\n{}",
                func.comments, func.signature, func.companion_field_signature,
            ))
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_structs.join("\n\n"),
    )
}

fn section_header(header: &str) -> String {
    format!("// Section: {}\n", header)
}

/// A Dart bridge module consists of several members:
/// - An `_Impl` class exposing the public Rust functions
/// - One or more `_Platform` classes implementing platform-specific helpers
///
/// The `_Impl` class takes a `_Platform _platform` instance as a private member,
/// and the `_Platform` exposes all of its methods decorated as `@protected`.
fn generate_dart_implementation_body(spec: &DartApiSpec, config: &Opts) -> Acc<DartBasicCode> {
    let mut lines = Acc::<Vec<_>>::default();
    let dart_api_impl_class_name = config.dart_api_impl_class_name();
    let dart_wire_class_name = config.dart_wire_class_name();
    let dart_api_class_name = config.dart_api_class_name();
    let dart_platform_class_name = config.dart_platform_class_name();
    let DartApiSpec {
        dart_funcs,
        dart_api2wire_funcs,
        dart_api_fill_to_wire_funcs,
        dart_wire2api_funcs,
        dart_wasm_funcs,
        dart_wasm_module,
        dart_structs: _,
        needs_freezed: _,
    } = spec;

    lines.push_acc(Acc::new(|target| match target {
        Common => format!(
            "class {impl} implements {} {{
                final {plat} _platform;
                factory {impl}(ExternalLibrary dylib) => {impl}.raw({plat}(dylib));

                /// Only valid on web/WASM platforms.
                factory {impl}.wasm(FutureOr<WasmModule> module) =>
                    {impl}(module as ExternalLibrary);
                {impl}.raw(this._platform);",
            dart_api_class_name,
            impl = dart_api_impl_class_name,
            plat = dart_platform_class_name,
        ),
        Io => format!(
            "class {plat} extends FlutterRustBridgeBase<{wire}> {{
                {plat}(ffi.DynamicLibrary dylib) : super({wire}(dylib));",
            plat = dart_platform_class_name,
            wire = dart_wire_class_name,
        ),
        Wasm => format!(
            "class {plat} extends FlutterRustBridgeBase<{wire}> with FlutterRustBridgeSetupMixin {{
                {plat}(FutureOr<WasmModule> dylib) : super({wire}(dylib)) {{
                    setupMixinConstructor();
                }}
                Future<void> setup() => inner.init;",
            plat = dart_platform_class_name,
            wire = dart_wire_class_name,
        ),
    }));

    lines.extend(dart_funcs.iter().map(|func| {
        format!(
            "{}\n\n{}",
            func.implementation, func.companion_field_implementation,
        )
    }));

    lines.push("}\n".into());

    lines.push_all(section_header("api2wire"));
    lines.push_acc(dart_api2wire_funcs.clone());

    lines.io.push(section_header("api_fill_to_wire"));
    lines.io.push(dart_api_fill_to_wire_funcs.join("\n\n"));

    lines.push_acc(Acc::new(|target| match target {
        Io | Wasm => "}\n".into(),
        Common => "".into(),
    }));

    lines.push(section_header("wire2api"));
    lines.push(dart_wire2api_funcs.join("\n\n"));

    if config.wasm_enabled {
        let dart_wasm_module_name = config.dart_wasm_module();
        lines.wasm.push(section_header("WASM wire module"));
        lines.wasm.push(format!(
            "@JS('wasm_bindgen') external {} get wasmModule;",
            dart_wasm_module_name,
        ));
        lines.wasm.push(format!(
            "@JS() @anonymous class {wasm} implements WasmModule {{
                external Object /* Promise */ call([String? moduleName]);
                external {wasm} bind(dynamic thisArg, String moduleName);
            ",
            wasm = dart_wasm_module_name,
        ));
        lines.wasm.push(dart_wasm_funcs.join("\n\n"));
        lines.wasm.push("}\n".into());

        lines.wasm.push(section_header("WASM wire connector"));
        (lines.wasm).push(dart_wasm_module.as_deref().unwrap_or_default().to_string());
    }

    let Acc { common, io, wasm } = lines.join("\n");
    let impl_import = if config.wasm_enabled {
        format!(
            "import '{}';
            export '{0}';
            import 'package:meta/meta.dart';",
            Path::new(&config.dart_output_path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap()
        )
    } else {
        "import 'package:meta/meta.dart';".into()
    };
    let common_import = if config.wasm_enabled {
        format!(
            "import '{}' if (dart.library.html) '{}'; import 'package:meta/meta.dart';",
            config
                .dart_io_output_path()
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap(),
            config
                .dart_wasm_output_path()
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap(),
        )
    } else {
        String::new()
    };
    Acc {
        common: DartBasicCode {
            body: common,
            import: common_import,
            ..Default::default()
        },
        io: DartBasicCode {
            import: impl_import.clone(),
            body: io,
            ..Default::default()
        },
        wasm: DartBasicCode {
            import: impl_import,
            body: wasm,
            ..Default::default()
        },
    }
}

fn generate_dart_declaration_code(
    common_header: &DartBasicCode,
    freezed_header: DartBasicCode,
    import_header: DartBasicCode,
    declaration_body: String,
) -> DartBasicCode {
    common_header
        + &freezed_header
        + &import_header
        + &DartBasicCode {
            import: "".to_string(),
            part: "".to_string(),
            body: declaration_body,
        }
}

fn generate_dart_implementation_code(
    common_header: &DartBasicCode,
    implementation_body: Acc<DartBasicCode>,
) -> Acc<DartBasicCode> {
    implementation_body.map(|body, _| common_header + &body)
}

fn generate_file_prelude() -> DartBasicCode {
    DartBasicCode {
        import: format!("{}

                // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names
                ",
                        CODE_HEADER
        ),
        part: "".to_string(),
        body: "".to_string(),
    }
}

#[derive(Debug)]
struct GeneratedApiFunc {
    signature: String,
    implementation: String,
    comments: String,
    companion_field_signature: String,
    companion_field_implementation: String,
}

#[derive(Debug)]
struct GeneratedApiMethod {
    signature: String,
    implementation: String,
}

fn generate_api_func(
    func: &IrFunc,
    ir_file: &IrFile,
    common_api2wire_body: &str,
) -> GeneratedApiFunc {
    let raw_func_param_list = func
        .inputs
        .iter()
        .map(|input| {
            format!(
                "{}{} {}",
                input.ty.dart_required_modifier(),
                input.ty.dart_api_type(),
                input.name.dart_style()
            )
        })
        .collect::<Vec<_>>();
    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let wire_param_list = [
        if func.mode.has_port_argument() {
            vec!["port_".to_string()]
        } else {
            vec![]
        },
        func.inputs
            .iter()
            .map(|input| {
                // edge case: ffigen performs its own bool-to-int conversions
                if let Primitive(IrTypePrimitive::Bool) = input.ty {
                    input.name.dart_style()
                } else {
                    let func = format!("api2wire_{}", input.ty.safe_ident());
                    format!(
                        "{}{}({})",
                        if common_api2wire_body.contains(&func) {
                            ""
                        } else {
                            "_platform."
                        },
                        func,
                        &input.name.dart_style()
                    )
                }
            })
            .collect::<Vec<_>>(),
    ]
    .concat();

    let func_expr = format!(
        "{} {}({{ {} }})",
        func.mode.dart_return_type(&func.output.dart_api_type()),
        func.name.to_case(Case::Camel),
        full_func_param_list.join(","),
    );

    let execute_func_name = match func.mode {
        IrFuncMode::Normal => "_platform.executeNormal",
        IrFuncMode::Sync => "_platform.executeSync",
        IrFuncMode::Stream { .. } => "_platform.executeStream",
    };

    let const_meta_field_name = format!("k{}ConstMeta", func.name.to_case(Case::Pascal));

    let signature = format!("{};", func_expr);

    let comments = dart_comments(&func.comments);

    let task_common_args = format!(
        "
        constMeta: {},
        argValues: [{}],
        hint: hint,
        ",
        const_meta_field_name,
        func.inputs
            .iter()
            .map(|input| input.name.dart_style())
            .collect::<Vec<_>>()
            .join(", "),
    );

    let input_0 = func.inputs.get(0).as_ref().map(|x| &x.ty);
    let input_0_struct_name = if let Some(StructRef(IrTypeStructRef { name, .. })) = &input_0 {
        Some(name)
    } else {
        None
    };
    let f = FunctionName::deserialize(&func.name);
    let func_output_struct_name = if let StructRef(IrTypeStructRef { name, .. }) = &func.output {
        Some(name)
    } else {
        None
    };
    let parse_success_data = if (f.is_static_method()
        && f.struct_name().unwrap() == {
            if let IrType::StructRef(IrTypeStructRef { name, freezed: _ }) = &func.output {
                name.clone()
            } else {
                "".to_string()
            }
        })
        // If struct has a method with first element `input0`
        || (input_0_struct_name.is_some() && MethodNamingUtil::has_methods(input_0_struct_name.unwrap(), ir_file))
        //If output is a struct with methods
        || (func_output_struct_name.is_some()
            && MethodNamingUtil::has_methods(func_output_struct_name.unwrap(), ir_file))
    {
        format!("(d) => _wire2api_{}(this, d)", func.output.safe_ident())
    } else {
        format!("_wire2api_{}", func.output.safe_ident())
    };

    let implementation = match func.mode {
        IrFuncMode::Sync => format!(
            "{} => {}(FlutterRustBridgeSyncTask(
                callFfi: () => _platform.inner.{}({}),
                {}
            ));",
            func_expr,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            task_common_args,
        ),
        _ => format!(
            "{} => {}(FlutterRustBridgeTask(
                callFfi: (port_) => _platform.inner.{}({}),
                parseSuccessData: {},
                {}
            ));",
            func_expr,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            parse_success_data,
            task_common_args,
        ),
    };

    let companion_field_signature = format!(
        "FlutterRustBridgeTaskConstMeta get {};",
        const_meta_field_name,
    );

    let companion_field_implementation = format!(
        "
        FlutterRustBridgeTaskConstMeta get {} => const FlutterRustBridgeTaskConstMeta(
            debugName: \"{}\",
            argNames: [{}],
        );
        ",
        const_meta_field_name,
        func.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.name.dart_style()))
            .collect::<Vec<_>>()
            .join(", "),
    );

    GeneratedApiFunc {
        signature,
        implementation,
        comments,
        companion_field_signature,
        companion_field_implementation,
    }
}

fn rust_wire_to_dart_wire(ty: &str) -> Cow<str> {
    let ty = ty.trim();
    if matches!(ty, "void" | "WireSyncReturnStruct") {
        return ty.into();
    }
    if ty.starts_with("*mut") {
        return format!("int /* {} */", ty).into();
    }
    format!("dynamic /* {} */", ty).into()
}

fn generate_wasm_wire_func_decl(func: &IrFuncLike) -> String {
    format!(
        "external {} {name}({});",
        rust_wire_to_dart_wire(&func.output),
        func.inputs
            .iter()
            .map(|(key, ty)| format!("{} {}", ty, key))
            .join(","),
        name = func.name,
    )
}

fn generate_wasm_wire_func_method(func: &IrFuncLike) -> String {
    format!(
        "{out} {name}({}) => wasmModule.{name}({});",
        func.inputs
            .iter()
            .map(|(key, ty)| format!("{} {}", ty, key))
            .join(","),
        func.inputs.iter().map(|(key, _)| key.to_string()).join(","),
        name = func.name,
        out = if func.has_port_argument {
            "void".into()
        } else {
            rust_wire_to_dart_wire(&func.output)
        },
    )
}

fn generate_api2wire_func(ty: &IrType, ir_file: &IrFile, config: &Opts) -> Acc<String> {
    TypeDartGenerator::new(ty.clone(), ir_file, config)
        .api2wire_body()
        .map(|body, target| {
            body.map(|body| {
                format!(
                    "@protected
                    {} api2wire_{}({} raw) {{
                        {}
                    }}",
                    ty.dart_wire_type(target),
                    ty.safe_ident(),
                    ty.dart_api_type(),
                    body,
                )
            })
            .unwrap_or_default()
        })
}

fn generate_api_fill_to_wire_func(ty: &IrType, ir_file: &IrFile, config: &Opts) -> String {
    if let Some(body) = TypeDartGenerator::new(ty.clone(), ir_file, config).api_fill_to_wire_body()
    {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        format!(
            "void _api_fill_to_wire_{}({} apiObj, {} wireObj) {{
                {}
            }}",
            ty.safe_ident(),
            ty.dart_api_type(),
            target_wire_type.dart_wire_type(Target::Io),
            body,
        )
    } else {
        "".to_string()
    }
}

fn generate_wire2api_func(
    ty: &IrType,
    ir_file: &IrFile,
    dart_api_class_name: &str,
    config: &Opts,
) -> String {
    let extra_argument = if matches!(ty, StructRef(IrTypeStructRef { name, freezed: _ }) if MethodNamingUtil::has_methods(name, ir_file))
    {
        format!("{} bridge,", dart_api_class_name)
    } else {
        "".to_string()
    };
    let body = TypeDartGenerator::new(ty.clone(), ir_file, config).wire2api_body();
    format!(
        "{} _wire2api_{}({}dynamic raw) {{
            {}
        }}
        ",
        ty.dart_api_type(),
        ty.safe_ident(),
        extra_argument,
        body,
    )
}

fn gen_wire2api_simple_type_cast(s: &str) -> String {
    format!("return raw as {};", s)
}

/// A trailing newline is included if comments is not empty.
fn dart_comments(comments: &[IrComment]) -> String {
    let mut comments = comments
        .iter()
        .map(IrComment::comment)
        .collect::<Vec<_>>()
        .join("\n");
    if !comments.is_empty() {
        comments.push('\n');
    }
    comments
}

fn dart_metadata(metadata: &[IrDartAnnotation]) -> String {
    let mut metadata = metadata
        .iter()
        .map(|it| match &it.library {
            Some(IrDartImport {
                alias: Some(alias), ..
            }) => format!("@{}.{}", alias, it.content),
            _ => format!("@{}", it.content),
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !metadata.is_empty() {
        metadata.push('\n');
    }
    metadata
}
