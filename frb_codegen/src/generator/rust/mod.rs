mod ty;
mod ty_boxed;
mod ty_delegate;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_primitive;
mod ty_primitive_list;
mod ty_struct;
pub use ty::*;
pub use ty_boxed::*;
pub use ty_delegate::*;
pub use ty_enum::*;
pub use ty_general_list::*;
pub use ty_optional::*;
pub use ty_primitive::*;
pub use ty_primitive_list::*;
pub use ty_struct::*;

use std::collections::HashSet;

use crate::ir::IrType::*;
use crate::method_utils::FunctionName;
use crate::others::*;
use crate::utils::BlockIndex;
use crate::{ir::*, Opts};

pub const HANDLER_NAME: &str = "FLUTTER_RUST_BRIDGE_HANDLER";

pub struct Output {
    pub code: String,
    pub extern_func_names: Vec<String>,
}

impl Output {
    pub fn get_exclude_symbols(&self, all_symbols: &[String]) -> Vec<String> {
        all_symbols
            .iter()
            .filter(|s| !self.extern_func_names.contains(s))
            .map(|s| (*s).clone())
            .collect::<Vec<_>>()
    }
}

pub fn generate(ir_file: &IrFile, rust_wire_mod: &str, config: &Opts) -> Output {
    let mut generator = Generator::new(config);
    let code = generator.generate(ir_file, rust_wire_mod);

    Output {
        code,
        extern_func_names: generator.extern_func_collector.names,
    }
}

struct Generator<'a> {
    extern_func_collector: ExternFuncCollector,
    config: &'a Opts,
}

impl<'a> Generator<'a> {
    fn new(config: &'a Opts) -> Self {
        Self {
            extern_func_collector: ExternFuncCollector::new(),
            config,
        }
    }

    fn generate(&mut self, ir_file: &IrFile, rust_wire_mod: &str) -> String {
        let mut lines: Vec<String> = vec![];

        let distinct_input_types = ir_file.distinct_types(true, false);
        let distinct_output_types = ir_file.distinct_types(false, true);

        lines.push(r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, clippy::unit_arg, clippy::double_parens, non_snake_case)]"#.to_string());
        lines.push(CODE_HEADER.to_string());

        lines.push(String::new());
        lines.push(format!("use crate::{}::*;", rust_wire_mod));
        lines.push("use flutter_rust_bridge::*;".to_string());
        lines.push(String::new());

        lines.push(self.section_header_comment("imports"));
        lines.extend(self.generate_imports(
            ir_file,
            rust_wire_mod,
            &distinct_input_types,
            &distinct_output_types,
        ));
        lines.push(String::new());

        lines.push(self.section_header_comment("wire functions"));
        lines.extend(
            ir_file
                .funcs
                .iter()
                .map(|f| self.generate_wire_func(f, ir_file)),
        );

        if !self.config.wasm {
            lines.push(self.section_header_comment("wire structs"));
            lines.extend(
                distinct_input_types
                    .iter()
                    .map(|ty| self.generate_wire_struct(ty, ir_file, self.config.wasm)),
            );
            lines.extend(
                distinct_input_types
                    .iter()
                    .map(|ty| TypeRustGenerator::new(ty.clone(), ir_file, self.config).structs()),
            );
        }

        lines.push(self.section_header_comment("wrapper structs"));
        lines.extend(
            distinct_output_types
                .iter()
                .filter_map(|ty| self.generate_wrapper_struct(ty, ir_file)),
        );
        lines.push(self.section_header_comment("static checks"));
        let static_checks: Vec<_> = distinct_output_types
            .iter()
            .filter_map(|ty| self.generate_static_checks(ty, ir_file))
            .collect();
        if !static_checks.is_empty() {
            lines.push("const _: fn() = || {".to_owned());
            lines.extend(static_checks);
            lines.push("};".to_owned());
        }

        if !self.config.wasm {
            lines.push(self.section_header_comment("allocate functions"));
            lines.extend(
                distinct_input_types
                    .iter()
                    .map(|f| self.generate_allocate_funcs(f, ir_file)),
            );
        }

        lines.push(self.section_header_comment("impl Wire2Api"));
        lines.push(self.generate_wire2api_misc().to_string());
        lines.extend(
            distinct_input_types
                .iter()
                .map(|ty| self.generate_wire2api_func(ty, ir_file)),
        );

        lines.push(self.section_header_comment("impl Wire2Api for JsValue"));
        lines.extend(
            distinct_input_types
                .iter()
                .filter_map(|ty| self.generate_wasm2api_func(ty, ir_file)),
        );

        if !self.config.wasm {
            lines.push(self.section_header_comment("impl NewWithNullPtr"));
            lines.push(self.generate_new_with_nullptr_misc().to_string());
            lines.extend(
                distinct_input_types
                    .iter()
                    .map(|ty| self.generate_new_with_nullptr_func(ty, ir_file)),
            );
        }

        lines.push(self.section_header_comment("impl IntoDart"));
        lines.extend(
            distinct_output_types
                .iter()
                .map(|ty| self.generate_impl_intodart(ty, ir_file)),
        );

        lines.push(self.section_header_comment("executor"));
        lines.push(self.generate_executor(ir_file));

        if self.config.block_index == BlockIndex::PRIMARY && !self.config.wasm {
            lines.push(self.section_header_comment("sync execution mode utility"));
            lines.push(self.generate_sync_execution_mode_utility());
        }

        lines.join("\n")
    }

    fn section_header_comment(&self, section_name: &str) -> String {
        format!("// Section: {}\n", section_name)
    }

    fn generate_imports(
        &self,
        ir_file: &IrFile,
        rust_wire_mod: &str,
        distinct_input_types: &[IrType],
        distinct_output_types: &[IrType],
    ) -> impl Iterator<Item = String> {
        let input_type_imports = distinct_input_types
            .iter()
            .map(|api_type| generate_import(api_type, ir_file, self.config));
        let output_type_imports = distinct_output_types
            .iter()
            .map(|api_type| generate_import(api_type, ir_file, self.config));

        input_type_imports
            .chain(output_type_imports)
            // Filter out `None` and unwrap
            .flatten()
            // Don't include imports from the API file
            .filter(|import| !import.starts_with(&format!("use crate::{}::", rust_wire_mod)))
            // de-duplicate
            .collect::<HashSet<String>>()
            .into_iter()
    }

    fn generate_executor(&mut self, ir_file: &IrFile) -> String {
        if ir_file.has_executor {
            "/* nothing since executor detected */".to_string()
        } else {
            format!(
                "support::lazy_static! {{
                    pub static ref {}: support::DefaultHandler = Default::default();
                }}",
                HANDLER_NAME
            )
        }
    }

    fn generate_sync_execution_mode_utility(&mut self) -> String {
        self.extern_func_collector.generate(
            "free_WireSyncReturnStruct",
            &["val: support::WireSyncReturnStruct"],
            None,
            "unsafe { let _ = support::vec_from_leak_ptr(val.ptr, val.len); }",
        )
    }

    fn generate_wire_func(&mut self, func: &IrFunc, ir_file: &IrFile) -> String {
        let f = FunctionName::deserialize(&func.name);
        let struct_name = f.struct_name();
        let wasm = self.config.wasm;
        let params = [
            if func.mode.has_port_argument() {
                vec!["port_: MessagePort".to_string()]
            } else {
                vec![]
            },
            func.inputs
                .iter()
                .map(|field| {
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field.ty.rust_wire_modifier(wasm),
                        field.ty.rust_wire_type(wasm)
                    )
                })
                .collect::<Vec<_>>(),
        ]
        .concat();

        let mut inner_func_params = [
            vec![],
            func.inputs
                .iter()
                .map(|field| format!("api_{}", field.name.rust_style()))
                .collect::<Vec<_>>(),
        ]
        .concat();
        if let IrFuncMode::Stream { argument_index } = func.mode {
            inner_func_params.insert(argument_index, "task_callback.stream_sink()".to_string());
        }
        let wrap_info_obj = format!(
            "WrapInfo{{ debug_name: \"{}\", port: {}, mode: FfiCallMode::{} }}",
            func.name,
            if func.mode.has_port_argument() {
                "Some(port_)"
            } else {
                "None"
            },
            func.mode.ffi_call_mode(),
        );

        let code_wire2api = func
            .inputs
            .iter()
            .map(|field| {
                format!(
                    "let api_{} = {}.wire2api();",
                    field.name.rust_style(),
                    field.name.rust_style()
                )
            })
            .collect::<Vec<_>>()
            .join("");

        let code_call_inner_func = if f.is_non_static_method() || f.is_static_method() {
            let method_name = if f.is_non_static_method() {
                inner_func_params[0] = format!("&{}", inner_func_params[0]);
                FunctionName::deserialize(&func.name).method_name()
            } else if f.is_static_method() {
                FunctionName::deserialize(&func.name)
                    .static_method_name()
                    .unwrap()
            } else {
                panic!("{} is not a method, nor a static method.", func.name)
            };
            TypeRustGenerator::new(func.output.clone(), ir_file, self.config).wrap_obj(format!(
                r"{}::{}({})",
                struct_name.unwrap(),
                method_name,
                inner_func_params.join(", ")
            ))
        } else {
            TypeRustGenerator::new(func.output.clone(), ir_file, self.config).wrap_obj(format!(
                "{}({})",
                func.name,
                inner_func_params.join(", ")
            ))
        };
        let code_call_inner_func_result = if func.fallible {
            code_call_inner_func
        } else {
            format!("Ok({})", code_call_inner_func)
        };

        let (handler_func_name, return_type, code_closure) = match func.mode {
            IrFuncMode::Sync => (
                "wrap_sync",
                Some("support::WireSyncReturnStruct"),
                format!(
                    "{}
                    {}",
                    code_wire2api, code_call_inner_func_result,
                ),
            ),
            IrFuncMode::Normal | IrFuncMode::Stream { .. } => (
                "wrap",
                None,
                format!(
                    "{}
                    move |task_callback| {}
                    ",
                    code_wire2api, code_call_inner_func_result,
                ),
            ),
        };

        let body = format!(
            "{}.{}({}, move || {{ {} }})",
            HANDLER_NAME, handler_func_name, wrap_info_obj, code_closure,
        );
        if self.config.wasm {
            format!(
                "#[wasm_bindgen] pub fn {}({}) {} {{ {} }}",
                func.wire_func_name(),
                params.join(","),
                return_type
                    .map(|t| format!("-> {t}"))
                    .unwrap_or_else(String::new),
                body
            )
        } else {
            self.extern_func_collector.generate(
                &func.wire_func_name(),
                &params
                    .iter()
                    .map(std::ops::Deref::deref)
                    .collect::<Vec<_>>(),
                return_type,
                &body,
            )
        }
    }

    fn generate_wire_struct(&mut self, ty: &IrType, ir_file: &IrFile, wasm: bool) -> String {
        if let Some(fields) =
            TypeRustGenerator::new(ty.clone(), ir_file, self.config).wire_struct_fields()
        {
            if !wasm {
                format!(
                    r###"
                #[repr(C)]
                #[derive(Clone)]
                pub struct {} {{
                    {}
                }}
                "###,
                    ty.rust_wire_type(wasm),
                    fields.join(",\n"),
                )
            } else {
                format!(
                    "#[wasm_bindgen] extern \"C\" {{
                        pub type {};
                        /* 
                        {}
                        */
                    }}",
                    ty.rust_wire_type(wasm),
                    fields.join("\n")
                )
            }
        } else {
            "".to_string()
        }
    }

    fn generate_allocate_funcs(&mut self, ty: &IrType, ir_file: &IrFile) -> String {
        TypeRustGenerator::new(ty.clone(), ir_file, self.config)
            .allocate_funcs(&mut self.extern_func_collector, self.config.block_index)
    }

    fn generate_wire2api_misc(&self) -> &'static str {
        r"pub trait Wire2Api<T> {
            fn wire2api(self) -> T;
        }

        impl<T, S> Wire2Api<Option<T>> for *mut S
            where
                *mut S: Wire2Api<T>
        {
            fn wire2api(self) -> Option<T> {
                if self.is_null() {
                    None
                } else {
                    Some(self.wire2api())
                }
            }
        }
        "
    }

    fn generate_wire2api_func(&mut self, ty: &IrType, ir_file: &IrFile) -> String {
        if let Some(body) = TypeRustGenerator::new(ty.clone(), ir_file, self.config).wire2api_body()
        {
            format!(
                "impl Wire2Api<{}> for {} {{
                    fn wire2api(self) -> {} {{
                        {}
                    }}
                }}",
                ty.rust_api_type(),
                ty.rust_wire_modifier(self.config.wasm) + &ty.rust_wire_type(self.config.wasm),
                ty.rust_api_type(),
                body,
            )
        } else {
            "".to_string()
        }
    }

    fn generate_static_checks(&mut self, ty: &IrType, ir_file: &IrFile) -> Option<String> {
        TypeRustGenerator::new(ty.clone(), ir_file, self.config).static_checks()
    }

    fn generate_wrapper_struct(&mut self, ty: &IrType, ir_file: &IrFile) -> Option<String> {
        match ty {
            IrType::StructRef(_)
            | IrType::EnumRef(_)
            | IrType::Delegate(IrTypeDelegate::PrimitiveEnum { .. }) => {
                TypeRustGenerator::new(ty.clone(), ir_file, self.config)
                    .wrapper_struct()
                    .map(|wrapper| {
                        format!(
                            r###"
                            #[derive(Clone)]
                            struct {}({});
                            "###,
                            wrapper,
                            ty.rust_api_type(),
                        )
                    })
            }
            _ => None,
        }
    }

    fn generate_new_with_nullptr_misc(&self) -> &'static str {
        "pub trait NewWithNullPtr {
            fn new_with_null_ptr() -> Self;
        }

        impl<T> NewWithNullPtr for *mut T {
            fn new_with_null_ptr() -> Self {
                std::ptr::null_mut()
            }
        }
        "
    }

    fn generate_new_with_nullptr_func(&mut self, ty: &IrType, ir_file: &IrFile) -> String {
        TypeRustGenerator::new(ty.clone(), ir_file, self.config)
            .new_with_nullptr(&mut self.extern_func_collector)
    }

    fn generate_impl_intodart(&mut self, ty: &IrType, ir_file: &IrFile) -> String {
        TypeRustGenerator::new(ty.clone(), ir_file, self.config).impl_intodart()
    }

    fn generate_wasm2api_func(&self, ty: &IrType, ir_file: &IrFile) -> Option<String> {
        TypeRustGenerator::new(ty.clone(), ir_file, self.config)
            .wasm2api_body()
            .map(|body| {
                format!(
                    "impl Wire2Api<{wire_type}> for JsValue {{
                        fn wire2api(self) -> {wire_type} {{
                            {}
                        }}
                    }}",
                    body,
                    wire_type = ty.rust_wire_type(true)
                )
            })
    }
}

pub fn generate_import(api_type: &IrType, ir_file: &IrFile, config: &Opts) -> Option<String> {
    TypeRustGenerator::new(api_type.clone(), ir_file, config).imports()
}

pub fn generate_list_allocate_func(
    collector: &mut ExternFuncCollector,
    safe_ident: &str,
    list: &impl IrTypeTrait,
    inner: &IrType,
    Opts {
        wasm, block_index, ..
    }: &Opts,
) -> String {
    let wasm = *wasm;
    collector.generate(
        &format!("new_{}_{}", safe_ident, block_index),
        &["len: i32"],
        Some(&[
            list.rust_wire_modifier(wasm).as_str(),
            list.rust_wire_type(wasm).as_str()
        ].concat()),
        &format!(
            "let wrap = {} {{ ptr: support::new_leak_vec_ptr(<{}{}>::new_with_null_ptr(), len), len }};
                support::new_leak_box_ptr(wrap)",
            list.rust_wire_type(wasm),
            inner.rust_ptr_modifier(),
            inner.rust_wire_type(wasm)
        ),
    )
}

pub struct ExternFuncCollector {
    names: Vec<String>,
}

impl ExternFuncCollector {
    fn new() -> Self {
        ExternFuncCollector { names: vec![] }
    }

    fn generate(
        &mut self,
        func_name: &str,
        params: &[&str],
        return_type: Option<&str>,
        body: &str,
    ) -> String {
        self.names.push(func_name.to_string());

        format!(
            r#"
                #[no_mangle]
                pub extern "C" fn {}({}) {} {{
                    {}
                }}
            "#,
            func_name,
            params.join(", "),
            return_type.map_or("".to_string(), |r| format!("-> {}", r)),
            body,
        )
    }
}
