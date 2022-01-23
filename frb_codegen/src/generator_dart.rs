use convert_case::{Case, Casing};
use log::debug;
use std::borrow::Cow;

use crate::api_types::ApiType::*;
use crate::api_types::*;
use crate::others::*;

pub fn generate(
    api_file: &ApiFile,
    dart_api_class_name: &str,
    dart_api_impl_class_name: &str,
    dart_wire_class_name: &str,
) -> (DartBasicCode, DartBasicCode, DartBasicCode, DartBasicCode) {
    let distinct_types = api_file.distinct_types(true, true);
    let distinct_input_types = api_file.distinct_types(true, false);
    let distinct_output_types = api_file.distinct_types(false, true);
    debug!("distinct_input_types={:?}", distinct_input_types);
    debug!("distinct_output_types={:?}", distinct_output_types);

    let dart_func_signatures_and_implementations = api_file
        .funcs
        .iter()
        .map(generate_api_func)
        .collect::<Vec<_>>();
    let dart_structs = distinct_types
        .iter()
        .filter_map(|ty| match ty {
            StructRef(s) => Some(generate_api_struct(s.get(api_file))),
            EnumRef(enu) => Some(generate_api_enum(enu.get(api_file))),
            _ => None,
        })
        .collect::<Vec<_>>();
    let wire_structs = distinct_input_types
        .iter()
        .map(|ty| generate_wire_struct(ty, api_file))
        .collect::<Vec<_>>();
    let wire_funcs = api_file
        .funcs
        .iter()
        .map(generate_wire_func)
        .collect::<Vec<_>>();
    let wasm_wire2api_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_wasm_wire2api_func(ty, api_file))
        .collect::<Vec<_>>();
    let wasm_api2wire_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_wasm_api2wire_func(ty, api_file))
        .collect::<Vec<_>>();
    let dart_api2wire_funcs = distinct_input_types
        .iter()
        .map(generate_api2wire_func)
        .collect::<Vec<_>>();
    let dart_api_fill_to_wire_funcs = distinct_input_types
        .iter()
        .map(|ty| generate_api_fill_to_wire_func(ty, api_file))
        .collect::<Vec<_>>();
    let dart_wire2api_funcs = distinct_output_types
        .iter()
        .map(|ty| generate_wire2api_func(ty, api_file))
        .collect::<Vec<_>>();

    let needs_freezed = distinct_types
        .iter()
        .any(|ty| matches!(ty, EnumRef(e) if e.is_struct));
    let freezed_header = if needs_freezed {
        DartBasicCode {
            import: "import 'package:freezed_annotation/freezed_annotation.dart';".to_string(),
            part: "part 'bridge_generated.freezed.dart';".to_string(),
            body: "".to_string(),
        }
    } else {
        DartBasicCode::default()
    };

    let wasm_header = DartBasicCode {
        import: "@JS() library bridge_generated_web; import 'package:js/js.dart';".to_owned(),
        ..Default::default()
    };

    let common_header = DartBasicCode {
        import: "import 'dart:convert';
            import 'dart:typed_data';"
            .to_string(),
        part: "".to_string(),
        body: "".to_string(),
    };

    let decl_body = format!(
        "abstract class {} {{
            {}
        }}

        {}
        ",
        dart_api_class_name,
        dart_func_signatures_and_implementations
            .iter()
            .map(|(sig, _, comm)| format!("{}{}", comm, sig))
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_structs.join("\n\n"),
    );

    let impl_body = format!(
        "class {} extends FlutterRustBridgeBase<{}> implements {} {{
            factory {}(ffi.DynamicLibrary dylib) => {}.raw({}(dylib));

            {}.raw({} inner) : super(inner);

            {}

            // Section: api2wire
            {}

            // Section: api_fill_to_wire
            {}
        }}

        // Section: wire2api
        {}
        ",
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_api_class_name,
        dart_api_impl_class_name,
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_api_impl_class_name,
        dart_wire_class_name,
        dart_func_signatures_and_implementations
            .iter()
            .map(|(_, imp, _)| imp.clone())
            .collect::<Vec<_>>()
            .join("\n\n"),
        dart_api2wire_funcs.join("\n\n"),
        dart_api_fill_to_wire_funcs.join("\n\n"),
        dart_wire2api_funcs.join("\n\n"),
    );

    let decl_code = &common_header
        + &freezed_header
        + &DartBasicCode {
            import: "".to_string(),
            part: "".to_string(),
            body: decl_body,
        };

    let impl_code = &common_header
        + &DartBasicCode {
            import: "import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';".to_string(),
            part: "".to_string(),
            body: impl_body,
        };

    let web_impl = wasm_header
        + &common_header
        + &DartBasicCode {
            import: "import '../bridge_generated.dart';".to_owned(),
            body: [
                wire_structs,
                wire_funcs,
                wasm_wire2api_funcs,
                wasm_api2wire_funcs,
            ]
            .concat()
            .join("\n\n"),
            ..Default::default()
        };

    let file_prelude = DartBasicCode {
        import: format!("{}

                // ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, camel_case_types
                ",
                        CODE_HEADER
        ),
        part: "".to_string(),
        body: "".to_string(),
    };

    (file_prelude, decl_code, impl_code, web_impl)
}

fn generate_api_func(func: &ApiFunc) -> (String, String, String) {
    let raw_func_param_list = func
        .inputs
        .iter()
        .map(|input| {
            format!(
                "{}{} {}",
                input.ty.required_modifier(),
                input.ty.dart_api_type(),
                input.name.dart_style()
            )
        })
        .collect::<Vec<_>>();

    let full_func_param_list = [raw_func_param_list, vec!["dynamic hint".to_string()]].concat();

    let wire_param_list = [
        if func.mode.has_port_argument() {
            vec!["port".to_string()]
        } else {
            vec![]
        },
        func.inputs
            .iter()
            .map(|input| {
                // edge case: ffigen performs its own bool-to-int conversions
                if let ApiType::Primitive(ApiTypePrimitive::Bool) = input.ty {
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

    let partial = format!(
        "{} {}({{ {} }})",
        func.mode.dart_return_type(&func.output.dart_api_type()),
        func.name.to_case(Case::Camel),
        full_func_param_list.join(","),
    );

    let execute_func_name = match func.mode {
        ApiFuncMode::Normal => "executeNormal",
        ApiFuncMode::Sync => "executeSync",
        ApiFuncMode::Stream => "executeStream",
    };

    let signature = format!("{};", partial);

    let comments = dart_comments(&func.comments);

    let task_common_args = format!(
        "
        constMeta: const FlutterRustBridgeTaskConstMeta(
            debugName: \"{}\",
            argNames: [{}],
        ),
        argValues: [{}],
        hint: hint,
        ",
        func.name,
        func.inputs
            .iter()
            .map(|input| format!("\"{}\"", input.name.dart_style()))
            .collect::<Vec<_>>()
            .join(", "),
        func.inputs
            .iter()
            .map(|input| input.name.dart_style())
            .collect::<Vec<_>>()
            .join(", "),
    );

    let implementation = match func.mode {
        ApiFuncMode::Sync => format!(
            "{} => {}(FlutterRustBridgeSyncTask(
            callFfi: () => inner.{}({}),
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            task_common_args,
        ),
        _ => format!(
            "{} => {}(FlutterRustBridgeTask(
            callFfi: (port) => inner.{}({}),
            parseSuccessData: _wire2api_{},
            {}
        ));",
            partial,
            execute_func_name,
            func.wire_func_name(),
            wire_param_list.join(", "),
            func.output.safe_ident(),
            task_common_args,
        ),
    };

    (signature, implementation, comments)
}

fn generate_api2wire_func(ty: &ApiType) -> String {
    let body = match ty {
        Primitive(ApiTypePrimitive::Bool) => "return raw ? 1 : 0;".to_owned(),
        Primitive(_) => "return raw;".to_string(),
        Delegate(d) => match d {
            ApiTypeDelegate::String => {
                "return _api2wire_uint_8_list(utf8.encoder.convert(raw));".to_string()
            }
            ApiTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".to_string(),
            ApiTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!("return _api2wire_{}(raw);", d.get_delegate().safe_ident())
            }
            ApiTypeDelegate::StringList => "final ans = inner.new_StringList(raw.length);
            for (var i = 0; i < raw.length; i++) {
                ans.ref.ptr[i] = _api2wire_String(raw[i]);
            }
            return ans;"
                .to_owned(),
        },
        Optional(opt) => format!(
            "return raw == null ? ffi.nullptr : _api2wire_{}(raw);",
            opt.inner.safe_ident()
        ),
        PrimitiveList(_) => {
            // NOTE Dart code *only* allocates memory. It never *release* memory by itself.
            // Instead, Rust receives that pointer and now it is in control of Rust.
            // Therefore, *never* continue to use this pointer after you have passed the pointer
            // to Rust.
            // NOTE WARN: Never use the [calloc] provided by Dart FFI to allocate any memory.
            // Instead, ask Rust to allocate some memory and return raw pointers. Otherwise,
            // memory will be allocated in one dylib (e.g. libflutter.so), and then be released
            // by another dylib (e.g. my_rust_code.so), especially in Android platform. It can be
            // undefined behavior.
            format!(
                "final ans = inner.new_{}(raw.length);
                ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
                return ans;",
                ty.safe_ident(),
            )
        }
        GeneralList(list) => {
            // NOTE 内存策略同PrimitiveList（比如Uint8List之类），见那边的注释
            format!(
                "final ans = inner.new_{}(raw.length);
                for (var i = 0; i < raw.length; ++i) {{
                    _api_fill_to_wire_{}(raw[i], ans.ref.ptr[i]);
                }}
                return ans;",
                ty.safe_ident(),
                list.inner.safe_ident()
            )
        }
        Boxed(b) => match &b.inner {
            Primitive(_) => {
                format!("return inner.new_{}(raw);", ty.safe_ident())
            }
            inner => {
                format!(
                    "final ptr = inner.new_{}();
                    _api_fill_to_wire_{}(raw, ptr.ref);
                    return ptr;",
                    ty.safe_ident(),
                    inner.safe_ident(),
                )
            }
        },
        EnumRef(e) if !e.is_struct => "return raw.index;".to_owned(),
        // skip, handled by transfomers
        StructRef(_) | EnumRef(_) => return "".to_string(),
    };

    format!(
        "{} _api2wire_{}({} raw) {{
            {}
        }}
        ",
        ty.dart_wire_type(),
        ty.safe_ident(),
        ty.dart_api_type(),
        body,
    )
}

fn generate_api_fill_to_wire_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let body = match &ty {
        StructRef(s) => {
            let s = s.get(api_file);
            s.fields
                .iter()
                .map(|field| {
                    format!(
                        "wireObj.{} = _api2wire_{}(apiObj.{});",
                        field.name.rust_style(),
                        field.ty.safe_ident(),
                        field.name.dart_style()
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        }
        EnumRef(enu) if enu.is_struct => enu
            .get(api_file)
            .variants()
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                if let ApiVariantKind::Value = &variant.kind {
                    format!(
                        "if (apiObj is {}) {{ wireObj.tag = {}; return; }}",
                        variant.name, idx
                    )
                } else {
                    let r = format!("wireObj.kind.ref.{}.ref", variant.name);
                    let body: Vec<_> = match &variant.kind {
                        ApiVariantKind::Struct(st) => st
                            .fields
                            .iter()
                            .map(|field| {
                                format!(
                                    "{}.{} = _api2wire_{}(apiObj.{});",
                                    r,
                                    field.name.rust_style(),
                                    field.ty.safe_ident(),
                                    field.name.dart_style()
                                )
                            })
                            .collect(),
                        _ => unreachable!(),
                    };
                    format!(
                        "if (apiObj is {0}) {{
                            wireObj.tag = {1};
                            wireObj.kind = inner.inflate_{2}_{0}();
                            {3}
                        }}",
                        variant.name,
                        idx,
                        enu.name,
                        body.join("\n")
                    )
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Optional(opt) => {
            if !opt.needs_initialization() || opt.is_list() {
                return String::new();
            }
            format!(
                "if (apiObj != null) _api_fill_to_wire_{}(apiObj, wireObj);",
                opt.inner.safe_ident()
            )
        }
        Boxed(boxed) if !matches!(boxed.inner, Primitive(_)) => format!(
            " _api_fill_to_wire_{}(apiObj, wireObj.ref);",
            boxed.inner.safe_ident()
        ),
        Primitive(_) | Delegate(_) | PrimitiveList(_) | GeneralList(_) | Boxed(_) | EnumRef(_) => {
            return "".to_string();
        }
    };

    format!(
        "void _api_fill_to_wire_{}({} apiObj, {} wireObj) {{
            {}
        }}",
        ty.safe_ident(),
        ty.dart_api_type(),
        ty.optional_inner().dart_wire_type(),
        body,
    )
}

fn generate_wire2api_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let gen_simple_type_cast = |s: &str| format!("return raw as {};", s);

    let body = match ty {
        Primitive(ApiTypePrimitive::Unit) => "return;".to_owned(),
        Primitive(p) => gen_simple_type_cast(&p.dart_api_type()),
        Delegate(d) => match d {
            ApiTypeDelegate::String
            | ApiTypeDelegate::SyncReturnVecU8
            | ApiTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                gen_simple_type_cast(&d.dart_api_type())
            }
            ApiTypeDelegate::StringList => {
                "return (raw as List<dynamic>).cast<String>();".to_owned()
            }
        },
        Optional(opt) => format!(
            "return raw == null ? null : _wire2api_{}(raw);",
            opt.inner.safe_ident()
        ),
        PrimitiveList(list) => gen_simple_type_cast(&list.dart_api_type()),
        GeneralList(list) => format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            list.inner.safe_ident()
        ),
        StructRef(s_ref) => {
            let s = s_ref.get(api_file);
            let inner = s
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    format!(
                        "{}: _wire2api_{}(arr[{}]),",
                        field.name.dart_style(),
                        field.ty.safe_ident(),
                        idx
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");

            format!(
                "final arr = raw as List<dynamic>;
                if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');
                return {}({});",
                s.fields.len(),
                s.fields.len(),
                s.name, inner,
            )
        }
        EnumRef(enu) if !enu.is_struct => format!("return {}.values[raw];", enu.name),
        EnumRef(enu) => {
            let enu = enu.get(api_file);
            let variants = enu
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    let args = match &variant.kind {
                        ApiVariantKind::Value => "".to_owned(),
                        ApiVariantKind::Struct(st) => st
                            .fields
                            .iter()
                            .enumerate()
                            .map(|(idx, field)| {
                                let val = format!(
                                    "_wire2api_{}(raw[{}]),",
                                    field.ty.safe_ident(),
                                    idx + 1
                                );
                                if st.is_fields_named {
                                    format!("{}: {}", field.name.dart_style(), val)
                                } else {
                                    val
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(""),
                    };
                    format!("case {}: return {}({});", idx, variant.name, args)
                })
                .collect::<Vec<_>>();
            format!(
                "switch (raw[0]) {{
                    {}
                    default: throw Exception(\"unreachable\");
                }}",
                variants.join("\n"),
            )
        }
        Boxed(boxed) => match &boxed.inner {
            StructRef(inner) => format!("return _wire2api_{}(raw);", inner.safe_ident()),
            _ => gen_simple_type_cast(&ty.dart_api_type()),
        },
    };

    format!(
        "{} _wire2api_{}(dynamic raw) {{
            {}
        }}
        ",
        ty.dart_api_type(),
        ty.safe_ident(),
        body,
    )
}

/// A trailing newline is included if comments is not empty.
fn dart_comments(comments: &[Comment]) -> String {
    let mut comments = comments
        .iter()
        .map(Comment::comment)
        .collect::<Vec<_>>()
        .join("\n");
    if !comments.is_empty() {
        comments.push('\n');
    }
    comments
}

fn generate_api_struct(s: &ApiStruct) -> String {
    let field_declarations = s
        .fields
        .iter()
        .map(|f| {
            let comments = dart_comments(&f.comments);
            format!(
                "{}final {} {};",
                comments,
                f.ty.dart_api_type(),
                f.name.dart_style()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let constructor_params = s
        .fields
        .iter()
        .map(|f| format!("{}this.{},", f.ty.required_modifier(), f.name.dart_style()))
        .collect::<Vec<_>>()
        .join("");

    let comments = dart_comments(&s.comments);

    format!(
        "{}class {} {{
            {}

            {}({{{}}});
        }}",
        comments, s.name, field_declarations, s.name, constructor_params
    )
}

fn generate_api_enum(enu: &ApiEnum) -> String {
    let comments = dart_comments(&enu.comments);
    if enu.is_struct() {
        let variants = enu
            .variants()
            .iter()
            .map(|variant| {
                let args = match &variant.kind {
                    ApiVariantKind::Value => "".to_owned(),
                    ApiVariantKind::Struct(ApiStruct {
                        is_fields_named: false,
                        fields,
                        ..
                    }) => {
                        let types = fields.iter().map(|field| &field.ty).collect::<Vec<_>>();
                        let split = optional_boundary_index(&types);
                        let types = fields
                            .iter()
                            .map(|field| {
                                format!(
                                    "{}{} {},",
                                    dart_comments(&field.comments),
                                    field.ty.dart_api_type(),
                                    field.name.dart_style()
                                )
                            })
                            .collect::<Vec<_>>();
                        if let Some(idx) = split {
                            let before = &types[..idx];
                            let after = &types[idx..];
                            format!("{}[{}]", before.join(""), after.join(""))
                        } else {
                            types.join(",")
                        }
                    }
                    ApiVariantKind::Struct(st) => {
                        let fields = st
                            .fields
                            .iter()
                            .map(|field| {
                                format!(
                                    "{}{}{} {},",
                                    dart_comments(&field.comments),
                                    field.ty.required_modifier(),
                                    field.ty.dart_api_type(),
                                    field.name.dart_style()
                                )
                            })
                            .collect::<Vec<_>>();
                        format!("{{ {} }}", fields.join(""))
                    }
                };
                format!(
                    "{}const factory {}.{}({}) = {};",
                    dart_comments(&variant.comments),
                    enu.name,
                    variant.name.dart_style(),
                    args,
                    variant.name.rust_style(),
                )
            })
            .collect::<Vec<_>>();
        format!(
            "@freezed
            class {0} with _${0} {{
                {1}
            }}",
            enu.name,
            variants.join("\n")
        )
    } else {
        let variants = enu
            .variants()
            .iter()
            .map(|variant| {
                format!(
                    "{}{},",
                    dart_comments(&variant.comments),
                    variant.name.rust_style()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{}enum {} {{
                {}
            }}",
            comments, enu.name, variants
        )
    }
}

fn generate_wire_struct_core(name: &str, fields: &[ApiField]) -> String {
    let params = fields
        .iter()
        .map(|field| {
            format!(
                "{}{} {}",
                field.ty.required_modifier(),
                field.ty.dart_js_wire_type(),
                field.name.rust_style()
            )
        })
        .collect::<Vec<_>>();
    format!(
        "@JS()
        @anonymous
        class {name} {{ external factory {name}({{ {params}, }}); }}",
        name = name,
        params = params.join(",")
    )
}

fn generate_wire_struct(ty: &ApiType, api_file: &ApiFile) -> String {
    let factory_params: Vec<_> = match ty {
        StructRef(st) => {
            return generate_wire_struct_core(&st.rust_wire_type(), &st.get(api_file).fields)
        }
        EnumRef(e) if e.is_struct => vec!["required int tag".to_owned(), "dynamic kind".to_owned()],
        _ => return "".to_owned(),
    };
    let enum_structs = if let EnumRef(enu) = ty {
        enu.get(api_file)
            .variants()
            .iter()
            .filter_map(|variant| match &variant.kind {
                ApiVariantKind::Struct(st) => Some(generate_wire_struct_core(
                    &format!("{}_{}", enu.name, variant.name.rust_style()),
                    &st.fields,
                )),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    } else {
        "".to_owned()
    };
    format!(
        "@JS()
        @anonymous
        class {name} {{ external factory {name}({{ {params}, }}); }}

        {}",
        enum_structs,
        name = ty.rust_wire_type(),
        params = factory_params.join(",")
    )
}

fn generate_wire_func(func: &ApiFunc) -> String {
    let port = func
        .mode
        .has_port_argument()
        .then(|| "int port_".to_owned());
    let args = port
        .into_iter()
        .chain(
            func.inputs
                .iter()
                .map(|inp| format!("{} {}", inp.ty.dart_js_wire_type(), inp.name.rust_style())),
        )
        .collect::<Vec<_>>();
    format!(
        r#"@JS(r"wasm_bindgen.{name}")
        external void {name}({args});"#,
        name = func.wire_func_name(),
        args = args.join(",")
    )
}

fn generate_wasm_api2wire_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let body: Cow<str> = match ty {
        Delegate(_) | Primitive(_) => "return raw;".into(),
        StructRef(st) => {
            let wire = st.rust_wire_type();
            let fields = st
                .get(api_file)
                .fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: _api2wire_{}(raw.{})",
                        field.name.rust_style(),
                        field.ty.safe_ident(),
                        field.name.dart_style()
                    )
                })
                .collect::<Vec<_>>();
            format!("return {}({});", wire, fields.join(",")).into()
        }
        Boxed(b) => format!("return _api2wire_{}(raw);", b.inner.safe_ident()).into(),
        Optional(opt) => format!(
            "return raw == null ? null : _api2wire_{}(raw);",
            opt.inner.safe_ident()
        )
        .into(),
        _ => "throw UnimplementedError();".into(),
    };
    format!(
        "{} _api2wire_{}({} raw) {{
            {}
        }}",
        ty.dart_js_wire_type(),
        ty.safe_ident(),
        ty.dart_api_type(),
        body
    )
}

fn generate_wasm_wire2api_func(ty: &ApiType, api_file: &ApiFile) -> String {
    let body: Cow<str> = match ty {
        Delegate(_) | Primitive(_) => format!("return raw as {};", ty.dart_api_type()).into(),
        GeneralList(list) => format!(
            "return (raw as List<dynamic>).map(_wire2api_{}).toList();",
            list.inner.safe_ident()
        )
        .into(),
        Boxed(b) => format!("return _wire2api_{}(raw);", b.inner.safe_ident()).into(),
        // Boxed(b) if !b.exist_in_real_api => return "".to_owned(),
        _ => return generate_wire2api_func(ty, api_file),
    };
    format!(
        "{} _wire2api_{}(dynamic /*{}*/ raw) {{
            {}
        }}",
        ty.dart_api_type(),
        ty.safe_ident(),
        ty.dart_js_wire_type(),
        body
    )
}
