mod func;
mod ty;
mod ty_boxed;
mod ty_dart_opaque;
mod ty_delegate;
mod ty_dynamic;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_primitive;
mod ty_primitive_list;
mod ty_record;
mod ty_rust_opaque;
mod ty_struct;
mod ty_sync_return;
mod wasm;

use func::*;
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::Path;
use wasm::*;

use itertools::Itertools;
pub use ty::*;
pub use ty_boxed::*;
pub use ty_dart_opaque::*;
pub use ty_delegate::*;
pub use ty_dynamic::*;
pub use ty_enum::*;
pub use ty_general_list::*;
pub use ty_optional::*;
pub use ty_primitive::*;
pub use ty_primitive_list::*;
pub use ty_record::*;
pub use ty_rust_opaque::*;
pub use ty_struct::*;
pub use ty_sync_return::*;

use convert_case::{Case, Casing};

use crate::ir::IrType::*;
use crate::target::Target::*;
use crate::target::{Acc, Target};
use crate::utils::method::{FunctionName, MethodNamingUtil};
use crate::utils::misc::{get_deduplicate_type, is_multi_blocks_case, PathExt};
use crate::{ir::*, Opts};
use crate::{others::*, transformer};

thread_local!(pub static COMMON_API2WIRE: RefCell<String> = RefCell::new("".into()));

#[derive(Debug)]
pub struct Output {
    pub file_prelude: DartBasicCode,
    pub decl_code: DartBasicCode,
    pub impl_code: Acc<DartBasicCode>,
    pub needs_freezed: bool,
}

pub fn generate(
    ir_file: &IrFile,
    config: &Opts,
    all_configs: &[Opts],
    wasm_funcs: &[IrFuncDisplay],
) -> Output {
    let dart_api_class_name = &config.dart_api_class_name();
    let dart_output_file_root = config.dart_output_root().expect("Internal error");
    let spec = DartApiSpec::from(ir_file, config, all_configs, wasm_funcs);
    let DartApiSpec {
        dart_funcs,
        dart_structs,
        ..
    } = &spec;

    log::debug!("block {:?} the spec is: {spec:?}", ir_file.block_index); //TODO: delete

    let needs_freezed = spec.needs_freezed;
    let mut common_header = generate_common_header();
    common_header.import += &config.extra_headers;

    let decl_code = generate_dart_declaration_code(
        &common_header,
        generate_freezed_header(dart_output_file_root, needs_freezed),
        generate_import_header(get_dart_imports(ir_file), spec.import_array.as_deref()),
        generate_dart_declaration_body(dart_api_class_name, dart_funcs, dart_structs),
    );

    let impl_code = generate_dart_implementation_code(
        &common_header,
        generate_dart_implementation_body(&spec, config, all_configs),
    );

    let file_prelude = generate_file_prelude();

    Output {
        file_prelude,
        decl_code,
        impl_code, // TODO: impl_code.common contains "import codes", may put it into `decl_code` in future?
        needs_freezed,
    }
}

#[derive(Debug)]
struct DartApiSpec {
    dart_funcs: Vec<GeneratedApiFunc>,
    dart_structs: Vec<String>,
    dart_api2wire_funcs: Acc<String>,
    dart_opaque_funcs: Acc<String>,
    dart_api_fill_to_wire_funcs: Vec<String>,
    dart_wire2api_funcs: Vec<String>,
    dart_wasm_funcs: Vec<String>,
    dart_wasm_module: Option<String>,
    needs_freezed: bool,
    import_array: Option<String>,
}

impl DartApiSpec {
    fn from(
        ir_file: &IrFile,
        config: &Opts,
        all_configs: &[Opts],
        extra_funcs: &[IrFuncDisplay],
    ) -> Self {
        let dart_api_class_name = config.dart_api_class_name();
        let dart_wire_class_name = config.dart_wire_class_name();
        let distinct_types = ir_file.distinct_types(true, true, all_configs);
        let distinct_input_types = ir_file.distinct_types(true, false, all_configs);
        let distinct_output_types = ir_file.distinct_types(false, true, all_configs);

        let dart_structs = get_deduplicate_type(&distinct_types)
            .iter()
            .map(|ty| TypeDartGenerator::new(ty.clone(), ir_file, config).structs())
            .collect::<Vec<_>>();

        // essential shared dart_api2wire funcs
        let shared_dart_api2wire_funcs = if is_multi_blocks_case(all_configs) {
            let shared_config = all_configs.last().unwrap();
            assert!(shared_config.shared);

            let raw_ir_file = shared_config.get_ir_file(all_configs).unwrap();
            let shared_ir_file = transformer::transform(raw_ir_file);
            let distinct_input_types = shared_ir_file.distinct_types(true, false, all_configs);
            Some(
                distinct_input_types
                    .iter()
                    .map(|ty| generate_api2wire_func(ty, &shared_ir_file, shared_config, &None))
                    .collect::<Acc<_>>()
                    .join("\n"),
            )
        } else {
            None
        };

        let dart_api2wire_funcs = get_dart_api2wire_funcs(
            &distinct_input_types,
            ir_file,
            config,
            &shared_dart_api2wire_funcs,
        );

        let common = COMMON_API2WIRE.with(|data| {
            let mut v = data.borrow_mut();
            *v = dart_api2wire_funcs.common.clone();
            v.clone()
        });

        let dart_funcs = (ir_file
            .funcs(true)
            .iter()
            .map(|f| generate_api_func(f, ir_file, &common, &shared_dart_api2wire_funcs)))
        .chain(
            distinct_output_types
                .iter()
                .filter(|ty| ty.is_rust_opaque() || ty.is_sync_rust_opaque())
                .map(generate_opaque_getters),
        )
        .collect::<Vec<_>>();

        let dart_api_fill_to_wire_funcs = distinct_input_types
            .iter()
            .map(|ty| {
                generate_api_fill_to_wire_func(ty, ir_file, config, &shared_dart_api2wire_funcs)
            })
            .collect::<Vec<_>>();

        let dart_wire2api_funcs = get_deduplicate_type(&distinct_output_types)
            .iter()
            .map(|ty| generate_wire2api_func(ty, ir_file, dart_api_class_name, config))
            .collect::<Vec<_>>();

        let dart_opaque_funcs = distinct_output_types
            .iter()
            .filter(|ty| ty.is_rust_opaque() || ty.is_sync_rust_opaque())
            .map(generate_opaque_func)
            .collect::<Acc<_>>()
            .join("\n");

        let ir_wasm_func_exports = config.wasm_enabled.then(|| {
            ir_file
                .funcs(true)
                .iter()
                .map(|fun| IrFuncDisplay::from_ir(fun, Target::Wasm))
                .chain(extra_funcs.iter().cloned())
                .collect::<Vec<_>>()
        });
        let dart_wasm_funcs = ir_wasm_func_exports
            .as_ref()
            .map(|exports| exports.iter().map(generate_wasm_wire_func_decl).collect())
            .unwrap_or_default();
        let dart_wasm_module = ir_wasm_func_exports.as_ref().map(|exports| {
            generate_wasm_wire(exports, &dart_wire_class_name, &config.dart_wasm_module())
        });

        let needs_freezed = distinct_types.iter().any(|ty| match ty {
            EnumRef(_) => true,
            StructRef(st) => st.freezed,
            _ => false,
        });

        let import_array = distinct_types
            .iter()
            .any(IrType::is_array)
            .then(|| "import 'package:collection/collection.dart';".to_owned());

        DartApiSpec {
            dart_funcs,
            dart_structs,
            dart_api2wire_funcs,
            dart_opaque_funcs,
            dart_api_fill_to_wire_funcs,
            dart_wire2api_funcs,
            dart_wasm_funcs,
            dart_wasm_module,
            needs_freezed,
            import_array,
        }
    }
}

pub fn get_dart_api2wire_funcs(
    distinct_input_types: &[IrType],
    ir_file: &IrFile,
    config: &Opts,
    shared_dart_api2wire_funcs: &Option<Acc<String>>,
) -> Acc<String> {
    let dart_api2wire_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_api2wire_func(ty, ir_file, config, shared_dart_api2wire_funcs))
        .collect::<Acc<_>>()
        .join("\n");
    dart_api2wire_funcs
}

fn generate_freezed_header(dart_output_file_root: &str, needs_freezed: bool) -> DartBasicCode {
    if needs_freezed {
        DartBasicCode {
            import: "import 'package:freezed_annotation/freezed_annotation.dart' hide protected;"
                .to_string(),
            part: format!("part '{dart_output_file_root}.freezed.dart';"),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    }
}

fn generate_import_header(
    imports: HashSet<&IrDartImport>,
    import_array: Option<&str>,
    // TODO: delete?
    // shared_mod: Option<&str>,
    // wasm_enabled: bool,
) -> DartBasicCode {
    log::debug!("dart imports:{:?}", imports); //TODO: delete
    log::debug!("dart import_array:{:?}", import_array); //TODO: delete
    let code = if !imports.is_empty() || import_array.is_some() {
        DartBasicCode {
            import: imports
                .iter()
                .map(|it| match &it.alias {
                    Some(alias) => format!("import '{}' as {};", it.uri, alias),
                    _ => format!("import '{}';", it.uri),
                })
                .collect::<Vec<_>>()
                .join("\n")
                + import_array.unwrap_or(""),
            part: "".to_string(),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    };

    // TODO: import shared module if essential
    // if let Some(shared_mod) = shared_mod {
    //     code.import += &format!("\nimport '{}';", shared_mod);
    //     code.import += &format!("\nexport '{}';", shared_mod);
    //     if config.wasm_enabled {
    //         code.import += &format!(
    //             "import '{}' if (dart.library.html) '{}';",
    //             config
    //                 .dart_io_output_path()
    //                 .file_name()
    //                 .and_then(OsStr::to_str)
    //                 .unwrap(),
    //             config
    //                 .dart_wasm_output_path()
    //                 .file_name()
    //                 .and_then(OsStr::to_str)
    //                 .unwrap(),
    //         );
    //     }
    // }

    code
}

fn generate_common_header() -> DartBasicCode {
    DartBasicCode {
        import: format!(
            "{}{}",
            "import 'dart:convert';
            import 'dart:async';
            import 'package:meta/meta.dart';
            import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';",
            if cfg!(feature = "uuid") {
                "\nimport 'package:uuid/uuid.dart';"
            } else {
                ""
            },
        ),
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

fn generate_dart_declaration_body(
    dart_api_class_name: &str,
    dart_funcs: &[GeneratedApiFunc],
    dart_structs: &[String],
) -> String {
    format!(
        "
        abstract class {0} {{
            {1}
        }}

        {2}
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
    format!("// Section: {header}\n")
}

/// A Dart bridge module consists of several members:
/// - An `_Impl` class exposing the public Rust functions
/// - One or more `_Platform` classes implementing platform-specific helpers
///
/// The `_Impl` class takes a `_Platform _platform` instance as a private member,
/// and the `_Platform` exposes all of its methods decorated as `@protected`.
fn generate_dart_implementation_body(
    spec: &DartApiSpec,
    config: &Opts,
    all_configs: &[Opts],
) -> Acc<DartBasicCode> {
    let mut lines = Acc::<Vec<_>>::default();
    let dart_api_impl_class_name = config.dart_api_impl_class_name();
    let dart_wire_class_name = config.dart_wire_class_name();
    let dart_api_class_name = config.dart_api_class_name();
    let dart_platform_class_name = config.dart_platform_class_name();
    let DartApiSpec {
        dart_funcs,
        dart_api2wire_funcs,
        dart_opaque_funcs,
        dart_api_fill_to_wire_funcs,
        dart_wire2api_funcs,
        dart_wasm_funcs,
        dart_wasm_module,
        ..
    } = spec;
    let shared_config = if is_multi_blocks_case(all_configs) {
        all_configs.last()
    } else {
        None
    };
    lines.push_acc(Acc {
        common: {
            let implement = &dart_api_impl_class_name;
            let plat = &dart_platform_class_name;
            let mut basic_code = match shared_config.is_none() || config.shared {
                true => {
                    format!(
                        "class {implement} implements {dart_api_class_name} {{
                            final {plat} _platform;
                            factory {implement}(ExternalLibrary dylib) => {implement}.raw({plat}(dylib));
                            {implement}.raw(this._platform);"
                    )
                },
                false => {
                    let shared_config = shared_config.unwrap();
                    let shared_impl = shared_config.dart_api_impl_class_name();
                    let shared_plat = shared_config.dart_platform_class_name();
                    format!(
                        "class {implement} implements {dart_api_class_name} {{
                            final {plat} _platform;
                            final {shared_plat} _sharedPlatform;
                            final {shared_impl} _sharedImpl;

                            factory {implement}(ExternalLibrary dylib) {{
                                final platform = {plat}(dylib);
                                final sharedPlatform = {shared_plat}(dylib);
                                final sharedImpl = {shared_impl}(dylib);
                                return {implement}.raw(platform, sharedPlatform,sharedImpl);
                            }}

                         {implement}.raw(this._platform, this._sharedPlatform, this._sharedImpl);"
                    )
                },
            };
            let wasm_factory_code = &format!("/// Only valid on web/WASM platforms.
                                                     factory {implement}.wasm(FutureOr<WasmModule> module) =>
                                                    {implement}(module as ExternalLibrary);");
            basic_code += &format!("{wasm_factory_code}\n");
            basic_code
        },
        io: {
            let plat = &dart_platform_class_name;
            let wire = &dart_wire_class_name;
            let (shared_definition, shared_initialization) =
            match shared_config.is_none() || config.shared{
                true => ("".to_string(), "".to_string()),
                false => {
                    let shared_plat = shared_config.unwrap().dart_platform_class_name();
                    (format!("final {} _sharedPlatform;", shared_plat), format!("_sharedPlatform = {}(dylib),", shared_plat))
                },
            };
            format!(
                "class {plat} extends FlutterRustBridgeBase<{wire}> {{
                {shared_definition}
                {plat}(ffi.DynamicLibrary dylib) :{shared_initialization} super({wire}(dylib));"
            )
        },
        wasm: {
            let plat = &dart_platform_class_name;
            let wire = &dart_wire_class_name;
            let (shared_definition, shared_initialization) =
            match shared_config.is_none() || config.shared{
                true => ("".to_string(), "".to_string()),
                false => {
                    let shared_plat = shared_config.unwrap().dart_platform_class_name();
                    (format!("final {} _sharedPlatform;", shared_plat), format!("_sharedPlatform = {}(dylib),", shared_plat))
                }
            };

            format!(
            "class {plat} extends FlutterRustBridgeBase<{wire}> with FlutterRustBridgeSetupMixin {{
                {shared_definition}
                {plat}(FutureOr<WasmModule> dylib) :{shared_initialization} super({wire}(dylib)) {{
                    setupMixinConstructor();
                }}
                Future<void> setup() => inner.init;",
            )
        },
    });

    lines.extend(dart_funcs.iter().map(|func| {
        format!(
            "{}\n\n{}",
            func.implementation, func.companion_field_implementation,
        )
    }));

    lines.push_acc(Acc {
        common: "void dispose() {_platform.dispose();}".to_owned(),
        ..Default::default()
    });

    lines.push(section_header("wire2api"));
    lines.push(dart_wire2api_funcs.join("\n\n"));

    lines.push("}\n".into());

    lines.push_all(section_header("api2wire"));
    lines.push_acc(dart_api2wire_funcs.clone());

    lines.push_all(section_header("finalizer"));
    lines.push_acc(dart_opaque_funcs.clone());

    lines.io.push(section_header("api_fill_to_wire"));
    lines.io.push(dart_api_fill_to_wire_funcs.join("\n\n"));

    lines.push_acc(Acc::new(|target| match target {
        Io | Wasm => "}\n".into(),
        Common => "".into(),
    }));

    if config.wasm_enabled {
        push_wasm_module(
            &mut lines,
            dart_wasm_funcs,
            dart_wasm_module.as_deref(),
            config,
        );
    }

    let Acc { common, io, wasm } = lines.join("\n");
    let mut impl_import_for_io = if config.wasm_enabled {
        format!(
            "import '{}'; export '{0}';",
            Path::new(&config.dart_output_path)
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap()
        )
    } else {
        "".into()
    };
    let mut impl_import_for_web = impl_import_for_io.clone();

    // If WASM is not enabled, the common and IO branches are
    // combined into one, making this import statement invalid.
    let mut common_import = if config.wasm_enabled {
        format!(
            "import '{}' if (dart.library.html) '{}';",
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
        "".into()
    };

    // In case of multi-blocks, add extra shared imports
    if let Some(shared_config) = shared_config {
        let dart_paths = shared_config.get_dart_output_paths();

        // 1. the common file
        common_import += &format!(
            "\nimport '{}';\nexport '{}';",
            dart_paths.base_path.get_file_name(),
            dart_paths.base_path.get_file_name(),
        );

        if config.wasm_enabled {
            common_import += &format!(
                "import '{}' if (dart.library.html) '{}';",
                dart_paths.io_path.get_file_name(),
                dart_paths.wasm_path.get_file_name(),
            );
        }

        if config.wasm_enabled {
            // 2. io file: import shared module
            impl_import_for_io += &format!("\nimport '{}';", dart_paths.io_path.get_file_name(),);

            // 3. wasm file: import shared module
            impl_import_for_web +=
                &format!("\nimport '{}';", dart_paths.wasm_path.get_file_name(),);
        }
    }

    Acc {
        common: DartBasicCode {
            body: common,
            import: common_import,
            ..Default::default()
        },
        io: DartBasicCode {
            import: impl_import_for_io,
            body: io,
            ..Default::default()
        },
        wasm: DartBasicCode {
            import: impl_import_for_web,
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
    log::debug!("common_header:\n{:?}", common_header); //TODO: delete
    log::debug!("freezed_header:\n{:?}", freezed_header); //TODO: delete
    log::debug!("import_header:\n{:?}", import_header); //TODO: delete

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
            // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const
            ",
            code_header()
        ),
        part: "".to_string(),
        body: "".to_string(),
    }
}

#[derive(Debug)]
pub(crate) struct GeneratedApiFunc {
    signature: String,
    implementation: String,
    comments: String,
    companion_field_signature: String,
    companion_field_implementation: String,
}

fn generate_api2wire_func(
    ty: &IrType,
    ir_file: &IrFile,
    config: &Opts,
    shared_dart_api2wire_funcs: &Option<Acc<String>>,
) -> Acc<String> {
    let generator = TypeDartGenerator::new(ty.clone(), ir_file, config);
    generator
        .api2wire_body(shared_dart_api2wire_funcs)
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

fn generate_api_fill_to_wire_func(
    ty: &IrType,
    ir_file: &IrFile,
    config: &Opts,
    shared_dart_api2wire_funcs: &Option<Acc<String>>,
) -> String {
    if let Some(body) = TypeDartGenerator::new(ty.clone(), ir_file, config)
        .api_fill_to_wire_body(shared_dart_api2wire_funcs)
    {
        let target_wire_type = match ty {
            Optional(inner) => &inner.inner,
            it => it,
        };

        let prefix = if !config.shared { "_" } else { "" };
        format!(
            "void {}api_fill_to_wire_{}({} apiObj, {} wireObj) {{
                {}
            }}",
            prefix,
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
    _dart_api_class_name: &str,
    config: &Opts,
) -> String {
    let prefix = if !config.shared { "_" } else { "" };
    let body = TypeDartGenerator::new(ty.clone(), ir_file, config).wire2api_body();
    format!(
        "{} {}wire2api_{}({} raw) {{
            {}
        }}
        ",
        ty.dart_api_type(),
        prefix,
        ty.safe_ident(),
        ty.dart_param_type(),
        body
    )
}

fn generate_opaque_func(ty: &IrType) -> Acc<String> {
    let api_type = ty.dart_api_type();
    let generate_impl = |finalizer_type: &str, finalizer_arg: &str| {
        format!(
            "late final {finalizer_type} _{api_type}Finalizer = {finalizer_type}({finalizer_arg});
            {finalizer_type} get {api_type}Finalizer => _{api_type}Finalizer;",
        )
    };

    Acc {
        io: generate_impl(
            "OpaqueTypeFinalizer",
            &format!("inner._drop_opaque_{api_type}Ptr"),
        ),
        wasm: generate_impl(
            "Finalizer<PlatformPointer>",
            &format!("inner.drop_opaque_{api_type}"),
        ),
        ..Default::default()
    }
}

#[cfg(feature = "chrono")]
fn gen_wire2api_chrono(chrono_type: &IrTypeTime) -> String {
    format!("return _wire2api_Chrono_{}(raw);", chrono_type)
}

fn gen_wire2api_simple_type_cast(s: &str) -> String {
    format!("return raw as {s};")
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
