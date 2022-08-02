mod ty;
mod ty_boxed;
mod ty_delegate;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_primitive;
mod ty_primitive_list;
mod ty_struct;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;

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

use crate::config::Acc;
use crate::ir::IrType::*;
use crate::method_utils::{FunctionName, MethodNamingUtil};
use crate::others::*;
use crate::{ir::*, Opts};

pub struct Output {
    pub file_prelude: DartBasicCode,
    pub decl_code: DartBasicCode,
    pub impl_code: Acc<DartBasicCode>,
    pub needs_freezed: bool,
}

pub fn generate(ir_file: &IrFile, config: &Opts) -> Output {
    let dart_api_class_name = &config.dart_api_class_name();
    let dart_output_file_root = config.dart_output_root().expect("Internal error");
    let spec = DartApiSpec::from_ir_file(ir_file, config);
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
    fn from_ir_file(ir_file: &IrFile, config: &Opts) -> Self {
        let dart_api_class_name = config.dart_api_class_name();
        let dart_wire_class_name = config.dart_wire_class_name();
        let distinct_types = ir_file.distinct_types(true, true);
        let distinct_input_types = ir_file.distinct_types(true, false);
        let distinct_output_types = ir_file.distinct_types(false, true);
        debug!("distinct_input_types={:?}", distinct_input_types);
        debug!("distinct_output_types={:?}", distinct_output_types);

        let dart_funcs = ir_file
            .funcs
            .iter()
            .map(|f| generate_api_func(f, ir_file))
            .collect::<Vec<_>>();
        let dart_structs = distinct_types
            .iter()
            .map(|ty| {
                TypeDartGenerator::new(
                    ty.clone(),
                    ir_file,
                    Some(dart_api_class_name.to_string()),
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
        let dart_api_fill_to_wire_funcs = distinct_input_types
            .iter()
            .map(|ty| generate_api_fill_to_wire_func(ty, ir_file, config))
            .collect::<Vec<_>>();
        let dart_wire2api_funcs = distinct_output_types
            .iter()
            .map(|ty| generate_wire2api_func(ty, ir_file, &dart_api_class_name, config))
            .collect::<Vec<_>>();

        let dart_wasm_funcs = (config.wasm_enabled)
            .then(|| {
                ir_file
                    .funcs
                    .iter()
                    .map(generate_wasm_wire_func_decl)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let dart_wasm_module = (config.wasm_enabled)
            .then(|| generate_wasm_module(&ir_file.funcs, &dart_wire_class_name));

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
            import: "import 'package:freezed_annotation/freezed_annotation.dart';".to_string(),
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
            import 'dart:typed_data';
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

fn generate_wasm_module(funcs: &[IrFunc], dart_wire_class_name: &str) -> String {
    format!(
        "class {cls} extends FlutterRustBridgeWasmWireBase {{
            {cls}(FutureOr<WasmModule> module) : super(module);
            
            {}
        }}
        ",
        funcs
            .iter()
            .map(generate_wasm_wire_func_method)
            .collect::<Vec<_>>()
            .join("\n\n"),
        cls = dart_wire_class_name
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

fn generate_dart_implementation_body(spec: &DartApiSpec, config: &Opts) -> Acc<DartBasicCode> {
    let mut lines = Acc::<Vec<_>>::default();
    let dart_api_impl_class_name = config.dart_api_impl_class_name();
    let dart_wire_class_name = config.dart_wire_class_name();
    let dart_api_class_name = config.dart_api_class_name();
    let wasm = config.wasm_enabled;
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

    let common_import = config.wasm_enabled.then(|| {
        format!(
            "import \"{}\" if (dart.library.html) \"{}\";",
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
    });

    let common_impl_import = config
        .wasm_enabled
        .then(|| {
            let common_path: &str = Path::new(&config.dart_output_path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap();
            format!("import \"{}\";", common_path)
        })
        .unwrap_or_default();

    // The common declaration will include the class and its constructor, but its methods
    // are defined by platform-specific extensions.
    lines.push(format!(
        "class {api} extends FlutterRustBridgeBase<{wire}> implements {} {{
            factory {api}(ExternalLibrary dylib) => {api}.raw({wire}(dylib));
            {api}.raw({wire} inner) : super(inner);",
        dart_api_class_name,
        api = dart_api_impl_class_name,
        wire = dart_wire_class_name
    ));

    lines.io.push(format!(
        "extension {0}IoExt on {0} {{",
        dart_api_impl_class_name
    ));
    lines.wasm.push(format!(
        "extension {0}WasmExt on {0} {{",
        dart_api_impl_class_name
    ));

    lines.extend(dart_funcs.iter().map(|func| {
        format!(
            "{}\n\n{}",
            func.implementation, func.companion_field_implementation,
        )
    }));

    lines.push_all(section_header("api2wire"));
    lines.push_acc(dart_api2wire_funcs.clone());

    lines.io.push(section_header("api_fill_to_wire"));
    lines.io.push(dart_api_fill_to_wire_funcs.join("\n\n"));

    lines.push_all("}\n".into());

    lines.push(section_header("wire2api"));
    lines.push(dart_wire2api_funcs.join("\n\n"));

    if wasm {
        lines.wasm.push(section_header("WASM wire functions"));
        lines.wasm.push(dart_wasm_funcs.join("\n\n"));

        lines.wasm.push(section_header("WASM wire module"));
        (lines.wasm).push(dart_wasm_module.as_deref().unwrap_or_default().to_string());
    }

    let Acc { common, io, wasm } = lines.join("\n");
    Acc {
        common: DartBasicCode {
            import: common_import.unwrap_or_default(),
            body: common,
            ..Default::default()
        },
        io: DartBasicCode {
            import: common_impl_import.clone(),
            body: io,
            ..Default::default()
        },
        wasm: DartBasicCode {
            import: common_impl_import,
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
    implementation_body.map(|body| common_header + &body)
}

fn generate_file_prelude() -> DartBasicCode {
    DartBasicCode {
        import: format!("{}

                // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member
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

fn generate_api_func(func: &IrFunc, ir_file: &IrFile) -> GeneratedApiFunc {
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
                    format!(
                        "_api2wire_{}({})",
                        &input.ty.safe_ident(),
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
        IrFuncMode::Normal => "executeNormal",
        IrFuncMode::Sync => "executeSync",
        IrFuncMode::Stream { .. } => "executeStream",
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
    let f = FunctionName::deserialize(&func.name);
    let parse_success_data = if (f.is_static_method()
        && f.struct_name().unwrap() == {
            if let IrType::StructRef(IrTypeStructRef { name, freezed: _ }) = &func.output {
                name.clone()
            } else {
                "".to_string()
            }
        })
        || (input_0.is_some() && MethodNamingUtil::struct_has_methods(ir_file, input_0.unwrap()))
    {
        format!("(d) => _wire2api_{}(this, d)", func.output.safe_ident())
    } else {
        format!("_wire2api_{}", func.output.safe_ident())
    };

    let implementation = match func.mode {
        IrFuncMode::Sync => format!(
            "{} => {}(FlutterRustBridgeSyncTask(
                callFfi: () => inner.{}({}),
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
                callFfi: (port_) => inner.{}({}),
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

fn generate_wasm_wire_func_decl(func: &IrFunc) -> String {
    format!(
        "@JS(r'wasm_bindgen.wire_{name}') external {} _wire_{name}({});",
        (!func.mode.has_port_argument())
            .then(|| func.output.dart_wire_type(true))
            .unwrap_or_else(|| "void".into()),
        (func.mode.dart_port_param())
            .into_iter()
            .chain(func.inputs.iter().map(|input| {
                format!(
                    "{} {}",
                    input.ty.dart_wire_type(true),
                    input.name.rust_style()
                )
            }))
            .collect::<Vec<_>>()
            .join(","),
        name = func.name,
    )
}

fn generate_wasm_wire_func_method(func: &IrFunc) -> String {
    let params = (func.mode.dart_port_param())
        .into_iter()
        .chain(func.inputs.iter().map(|input| {
            format!(
                "{} {}",
                input.ty.dart_wire_type(true),
                input.name.rust_style()
            )
        }))
        .collect::<Vec<_>>();
    let vars = (func.mode.dart_port_var())
        .into_iter()
        .chain(func.inputs.iter().map(|input| input.name.rust_style()))
        .collect::<Vec<_>>();
    format!(
        "{} wire_{name}({}) => {};",
        (!func.mode.has_port_argument())
            .then(|| func.output.dart_wire_type(true))
            .unwrap_or_else(|| "void".into()),
        params.join(","),
        if func.mode.has_port_argument() {
            format!(
                "init.then((_) => _wire_{name}({}))",
                vars.join(","),
                name = func.name
            )
        } else {
            format!("_wire_{name}({})", vars.join(","), name = func.name)
        },
        name = func.name
    )
}

fn generate_api2wire_func(ty: &IrType, ir_file: &IrFile, config: &Opts) -> Acc<String> {
    let Acc { common, io, wasm } =
        TypeDartGenerator::new(ty.clone(), ir_file, None, config).api2wire_body();
    let wrapper = |body: &str, wasm: bool| {
        format!(
            "{} _api2wire_{}({} raw) {{
                    {}
                }}",
            ty.dart_wire_type(wasm),
            ty.safe_ident(),
            ty.dart_api_type(),
            body,
        )
    };
    Acc {
        common: common
            .map(|common| wrapper(&common, true))
            .unwrap_or_default(),
        io: io.map(|io| wrapper(&io, false)).unwrap_or_default(),
        wasm: wasm.map(|wasm| wrapper(&wasm, true)).unwrap_or_default(),
    }
}

fn generate_api_fill_to_wire_func(ty: &IrType, ir_file: &IrFile, config: &Opts) -> String {
    if let Some(body) =
        TypeDartGenerator::new(ty.clone(), ir_file, None, config).api_fill_to_wire_body()
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
            target_wire_type.dart_wire_type(false),
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
    let body = TypeDartGenerator::new(ty.clone(), ir_file, None, config).wire2api_body();
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
