mod ty;
mod ty_boxed;
mod ty_dart_opaque;
mod ty_delegate;
mod ty_dynamic;
mod ty_enum;
mod ty_general_list;
mod ty_optional;
mod ty_optional_list;
mod ty_primitive;
mod ty_primitive_list;
mod ty_record;
mod ty_rust_opaque;
mod ty_struct;
mod ty_sync_return;

pub use ty::*;
pub use ty_boxed::*;
pub use ty_dart_opaque::*;
pub use ty_delegate::*;
pub use ty_dynamic::*;
pub use ty_enum::*;
pub use ty_general_list::*;
pub use ty_optional::*;
pub use ty_optional_list::*;
pub use ty_primitive::*;
pub use ty_primitive_list::*;
pub use ty_record::*;
pub use ty_rust_opaque::*;
pub use ty_struct::*;
pub use ty_sync_return::*;

use std::collections::HashSet;
use std::fmt::Display;

use crate::config::all_configs::AllConfigs;
use crate::ir::IrType::*;
use crate::others::*;
use crate::target::Acc;
use crate::target::Target;
use crate::target::Target::*;
use crate::utils::method::FunctionName;

use crate::utils::misc::ExtraTraitForVec;
use crate::{ir::*, Opts};
use itertools::Itertools;

pub const HANDLER_NAME: &str = "FLUTTER_RUST_BRIDGE_HANDLER";

pub struct Output {
    pub code: Acc<String>,
    pub extern_func_names: Vec<String>,
    pub wasm_exports: Vec<IrFuncDisplay>,
}

impl Output {
    pub fn get_exclude_symbols(
        &self,
        all_symbols: &[String],
        config: &Opts,
        all_configs: &AllConfigs,
    ) -> Vec<String> {
        let mut exclude_symbols = all_symbols
            .iter()
            .filter(|s| !self.extern_func_names.contains(s))
            .map(|s| (*s).clone())
            .collect::<Vec<_>>();

        log::debug!(
            "for block:{}, the raw exclude_symbols:{exclude_symbols:#?}",
            config.block_index
        ); // TODO: delete
        if !all_configs.is_multi_blocks_case() {
            return exclude_symbols;
        }

        if !config.shared {
            exclude_symbols.extend(all_configs.get_shared_methods_wire_names());
        }

        log::debug!(
            "for block:{}, the final exclude_symbols:{exclude_symbols:#?}",
            config.block_index
        ); // TODO: delete

        exclude_symbols
    }
}

pub fn generate(config: &Opts, all_configs: &AllConfigs) -> Output {
    let mut generator = Generator::new(config);
    let code = generator.generate(config, all_configs);
    Output {
        code,
        extern_func_names: generator.extern_func_collector.names,
        wasm_exports: generator.extern_func_collector.wasm_exports,
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

    fn generate(&mut self, config: &Opts, all_configs: &AllConfigs) -> Acc<String> {
        let mut lines = Acc::<Vec<_>>::default();

        let distinct_input_types =
            all_configs.get_types(config.block_index, true, true, true, false);
        let distinct_output_types =
            all_configs.get_types(config.block_index, true, true, false, true);

        lines.push(r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, clippy::unit_arg, clippy::double_parens, non_snake_case, clippy::too_many_arguments)]"#.to_string());
        lines.push(code_header());

        lines.push(String::new());
        lines.push("use flutter_rust_bridge::*;".to_owned());
        lines.push("use core::panic::UnwindSafe;".to_owned());

        let (base_module, shared_module) = if config.shared {
            let module = all_configs.get_rust_module_name(Some(config)).unwrap();
            (module.clone(), Some(module))
        } else {
            (
                all_configs.get_rust_module_name(Some(config)).unwrap(),
                all_configs.get_rust_module_name(None),
            )
        };

        if !config.shared {
            lines.push(format!("use crate::{base_module}::*;"));
            lines.push("use std::sync::Arc;".to_owned());
            lines.push("use std::ffi::c_void;".to_owned());
        }
        lines.push("use flutter_rust_bridge::rust2dart::IntoIntoDart;".to_owned());
        lines.push(String::new());
        lines.push(self.section_header_comment("imports"));
        lines.extend(self.generate_imports(
            config,
            all_configs,
            &base_module,
            shared_module.as_deref(),
            &distinct_input_types,
            &distinct_output_types,
        ));
        lines.push(String::new());

        lines.push_all(self.section_header_comment("wire functions"));
        let funcs = &all_configs.get_funcs(config.block_index, true);
        lines += funcs
            .iter()
            .map(|f| self.generate_wire_func(f, config, shared_module.as_deref(), all_configs))
            .collect();

        lines.push(self.section_header_comment("wrapper structs"));
        lines.extend(
            distinct_output_types
                .iter()
                .filter_map(|ty| self.generate_wrapper_struct(ty, config, all_configs)),
        );
        lines.push(self.section_header_comment("static checks"));
        let static_checks: Vec<_> = distinct_output_types
            .iter()
            .filter_map(|ty| self.generate_static_checks(ty, config, all_configs))
            .collect();
        if !static_checks.is_empty() {
            lines.push("const _: fn() = || {".to_owned());
            lines.extend(static_checks);
            lines.push("};".to_owned());
        }

        lines.push_all(self.section_header_comment("allocate functions"));
        lines += distinct_input_types
            .iter()
            .map(|f| self.generate_allocate_funcs(f, all_configs))
            .collect();

        lines.push_all(self.section_header_comment("related functions"));
        lines += distinct_output_types
            .iter()
            .map(|f| self.generate_related_funcs(f, all_configs))
            .collect();

        lines.push_all(self.section_header_comment("impl Wire2Api"));
        lines.push(self.generate_wire2api_misc().to_string());
        lines += distinct_input_types
            .iter()
            .map(|ty| self.generate_wire2api_func(ty, all_configs))
            .collect();

        lines.push(self.section_header_comment("impl IntoDart"));
        lines.extend(
            distinct_output_types
                .iter()
                .filter(|ty| !ty.is_sync_return() && !ty.is_boxed())
                .map(|ty| self.generate_impl_intodart(ty, all_configs)),
        );

        lines.push(self.section_header_comment("executor"));
        lines.push(self.generate_executor(all_configs));

        self.generate_io_part(&mut lines, &distinct_input_types, all_configs);
        self.generate_wasm_part(&mut lines, &distinct_input_types, all_configs);

        lines.join("\n")
    }

    fn generate_io_part(
        &mut self,
        lines: &mut Acc<Vec<String>>,
        distinct_input_types: &[IrType],
        all_configs: &AllConfigs,
    ) {
        let core_input_types = distinct_input_types
            .iter()
            .map(|each| each.get_core_type().clone())
            .collect::<Vec<_>>()
            .find_uniques_in_order(false);
        lines.io.push(self.section_header_comment("wire structs"));
        lines.io.extend(
            core_input_types
                .iter()
                .map(|ty| self.generate_wire_struct(ty, all_configs)),
        );
        lines.io.extend(
            distinct_input_types
                .iter()
                .map(|ty| TypeRustGenerator::new(ty.clone(), self.config, all_configs).structs()),
        );

        (lines.io).push(self.section_header_comment("impl NewWithNullPtr"));
        if !all_configs.is_multi_blocks_case() || self.config.shared {
            (lines.io).push(self.generate_new_with_nullptr_misc().to_string());
        }

        lines.io.extend({
            core_input_types
                .iter()
                .map(|ty| self.generate_new_with_nullptr_func(ty, all_configs))
        });

        (lines.io).push(self.section_header_comment("sync execution mode utility"));

        // add `free_WireSyncReturn` only for single-block case or the share block in multi-blocks case
        let is_multi_blocks = all_configs.is_multi_blocks_case();
        if (!is_multi_blocks && !self.config.shared) || (is_multi_blocks && self.config.shared) {
            lines.io.push(self.generate_sync_execution_mode_utility());
        }
    }

    fn generate_wasm_part(
        &mut self,
        lines: &mut Acc<Vec<String>>,
        distinct_input_types: &[IrType],
        all_configs: &AllConfigs,
    ) {
        let _ir_file = all_configs.get_ir_file(self.config.block_index).unwrap();
        (lines.wasm).push(self.section_header_comment("impl Wire2Api for JsValue"));
        (lines.wasm).push(
            "impl<T> Wire2Api<Option<T>> for JsValue where JsValue: Wire2Api<T> {
                fn wire2api(self) -> Option<T> {
                    (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
                }
            }"
            .into(),
        );
        lines.wasm.extend(
            distinct_input_types
                .iter()
                .filter_map(|ty| self.generate_wasm2api_func(ty, all_configs)),
        );
    }

    fn section_header_comment(&self, section_name: &str) -> String {
        format!("// Section: {section_name}\n")
    }

    fn generate_imports(
        &self,
        config: &Opts,
        all_configs: &AllConfigs,
        rust_wire_mod: &str,
        shared_rust_wire_mod: Option<&str>,
        distinct_input_types: &[IrType],
        distinct_output_types: &[IrType],
    ) -> impl Iterator<Item = String> {
        let input_types_import_statements = distinct_input_types
            .iter()
            .map(|api_type| generate_import(api_type, self.config, all_configs))
            .collect::<Vec<_>>();
        let output_types_import_statements = distinct_output_types
            .iter()
            .map(|api_type| generate_import(api_type, self.config, all_configs))
            .collect::<Vec<_>>();

        let mut import_crates = input_types_import_statements
            .into_iter()
            .chain(output_types_import_statements)
            // Filter out `None` and unwrap
            .flatten()
            // Don't include imports from the API file
            .filter(|import| !import.starts_with(&format!("use crate::{rust_wire_mod}::")))
            // de-duplicate
            .collect::<HashSet<String>>();

        if config.shared {
            import_crates = import_crates.iter().map(|s| format!("pub {}", s)).collect()
        } else {
            // import shared-block module for regular block, if essential
            if let Some(shared_rust_wire_mod) = shared_rust_wire_mod {
                let shared_crate_str = format!("use crate::{shared_rust_wire_mod}");
                import_crates.extend(vec![
                    format!("{shared_crate_str};"),
                    format!("{shared_crate_str}::*;"),
                ])
            }
        }

        import_crates.into_iter()
    }

    fn generate_executor(&mut self, all_configs: &AllConfigs) -> String {
        let ir_file = all_configs.get_ir_file(self.config.block_index).unwrap();
        if ir_file.has_executor {
            "/* nothing since executor detected */".to_string()
        } else {
            format!(
                "support::lazy_static! {{
                    pub static ref {HANDLER_NAME}: support::DefaultHandler = Default::default();
                }}"
            )
        }
    }

    fn generate_sync_execution_mode_utility(&mut self) -> String {
        self.extern_func_collector.generate(
            "free_WireSyncReturn",
            [("ptr: support::WireSyncReturn", "")],
            None,
            "unsafe { let _ = support::box_from_leak_ptr(ptr); };",
            Io,
        )
    }

    fn generate_wire_func(
        &mut self,
        func: &IrFunc,
        config: &Opts,
        shared_rust_wire_mod: Option<&str>,
        all_configs: &AllConfigs,
    ) -> Acc<String> {
        let f = FunctionName::deserialize(&func.name);
        let struct_name = f.struct_name();
        let mut params = if func.mode.has_port_argument() {
            Acc {
                io: vec!["port_: i64".to_owned()],
                wasm: vec!["port_: MessagePort".to_owned()],
                common: vec!["port_: MessagePort".to_owned()],
            }
        } else {
            Acc::default()
        };

        let distinct_input_types_in_shared_block = if config.shared {
            vec![]
        } else if let Some(shared_config) = all_configs.get_shared_config() {
            all_configs.get_types(shared_config.block_index, false, true, true, false)
        } else {
            vec![]
        };

        params += (func.inputs)
            .iter()
            .map(|field| {
                let rust_api_type = field.ty.rust_api_type();
                let name = field.name.rust_style();

                let is_field_shared = |field: &IrField| {
                    if let Optional(ir) = &field.ty {
                        distinct_input_types_in_shared_block.contains(&ir.inner)
                    } else {
                        distinct_input_types_in_shared_block.contains(&field.ty)
                    }
                };

                Acc::new(|target| match target {
                    Common => {
                        format!(
                            "{}: impl {}Wire2Api<{}> + UnwindSafe",
                            name,
                            if !config.shared && is_field_shared(field) {
                                format!("{}::", shared_rust_wire_mod.unwrap())
                            } else {
                                "".into()
                            },
                            rust_api_type
                        )
                    }
                    Io | Wasm => format!(
                        "{}: {}{}",
                        name,
                        field.ty.rust_wire_modifier(target),
                        field.ty.rust_wire_type(target)
                    ),
                })
            })
            .collect();

        let mut inner_func_params = [
            vec![],
            func.inputs
                .iter()
                .map(|field| format!("api_{}", field.name.rust_style()))
                .collect::<Vec<_>>(),
        ]
        .concat();
        let ir_file = all_configs.get_ir_file(config.block_index).unwrap();
        if let IrFuncMode::Stream { argument_index } = func.mode {
            inner_func_params.insert(
                argument_index,
                format!(
                    "task_callback.stream_sink::<_,{}>()",
                    func.output.intodart_type(ir_file)
                ),
            );
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
            .map(|field| format!("let api_{0} = {0}.wire2api();", field.name.rust_style()))
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
            format!(
                r"{}::{}({})",
                struct_name.unwrap(),
                method_name,
                inner_func_params.join(", ")
            )
        } else {
            format!("{}({})", func.name, inner_func_params.join(", "))
        };
        let code_call_inner_func_result = if func.fallible {
            code_call_inner_func
        } else {
            format!("Result::<_,()>::Ok({code_call_inner_func})")
        };

        let (handler_func_name, return_type, code_closure) = match func.mode {
            IrFuncMode::Sync => (
                String::from("wrap_sync"),
                Some("support::WireSyncReturn"),
                format!(
                    "{code_wire2api}
                    {code_call_inner_func_result}"
                ),
            ),
            IrFuncMode::Normal | IrFuncMode::Stream { .. } => {
                let output = if matches!(func.mode, IrFuncMode::Stream { .. }) {
                    String::from("()")
                } else {
                    func.output.intodart_type(ir_file)
                };
                (
                    format!("wrap::<_,_,_,{output},_>"),
                    None,
                    format!("{code_wire2api} move |task_callback| {code_call_inner_func_result}"),
                )
            }
        };

        let body = format!(
            "{HANDLER_NAME}.{handler_func_name}({wrap_info_obj}, move || {{ {code_closure} }})"
        );
        let redirect_body = format!(
            "{}_impl({})",
            func.wire_func_name(),
            (func.mode.has_port_argument().then_some("port_"))
                .into_iter()
                .chain(func.inputs.iter().map(|arg| arg.name.rust_style()))
                .collect::<Vec<_>>()
                .join(","),
        );
        Acc::new(|target| match target {
            Io | Wasm => self.extern_func_collector.generate(
                &func.wire_func_name(),
                if target.is_wasm() {
                    &params.wasm[..]
                } else {
                    &params.io[..]
                }
                .iter()
                .map(|item| (item, ""))
                .collect::<Vec<_>>(),
                return_type,
                &redirect_body,
                target,
            ),
            Common => format!(
                "fn {}_impl({}) {} {{ {} }}",
                func.wire_func_name(),
                params.common.join(","),
                return_type.map(|t| format!("-> {t}")).unwrap_or_default(),
                body,
            ),
        })
    }

    fn generate_wire_struct(&mut self, ty: &IrType, all_configs: &AllConfigs) -> String {
        if let Some(fields) =
            TypeRustGenerator::new(ty.clone(), self.config, all_configs).wire_struct_fields()
        {
            format!(
                r###"
                #[repr(C)]
                #[derive(Clone)]
                pub struct {} {{
                    {}
                }}
                "###,
                ty.rust_wire_type(Target::Io),
                fields.join(",\n"),
            )
        } else {
            "".to_string()
        }
    }

    fn generate_allocate_funcs(&mut self, ty: &IrType, all_configs: &AllConfigs) -> Acc<String> {
        TypeRustGenerator::new(ty.clone(), self.config, all_configs)
            .allocate_funcs(&mut self.extern_func_collector)
            .map(|func, _| func.unwrap_or_default())
    }

    fn generate_related_funcs(&mut self, ty: &IrType, all_configs: &AllConfigs) -> Acc<String> {
        TypeRustGenerator::new(ty.clone(), self.config, all_configs)
            .related_funcs(&mut self.extern_func_collector)
            .map(|func, _| func.unwrap_or_default())
    }

    fn generate_wire2api_misc(&self) -> &'static str {
        r#"pub trait Wire2Api<T> {
            fn wire2api(self) -> T;
        }

        impl<T, S> Wire2Api<Option<T>> for *mut S
        where
            *mut S: Wire2Api<T>
        {
            fn wire2api(self) -> Option<T> {
                (!self.is_null()).then(|| self.wire2api())
            }
        }"#
    }

    fn generate_wire2api_func(&mut self, ty: &IrType, all_configs: &AllConfigs) -> Acc<String> {
        TypeRustGenerator::new(ty.clone(), self.config, all_configs)
            .wire2api_body()
            .map(|body, target| {
                body.map(|body| {
                    format!(
                        "impl Wire2Api<{api}> for {}{} {{
                            fn wire2api(self) -> {api} {{
                                {}
                            }}
                        }}",
                        ty.rust_wire_modifier(target),
                        ty.rust_wire_type(target),
                        body,
                        api = ty.rust_api_type(),
                    )
                })
                .unwrap_or_default()
            })
    }

    fn generate_static_checks(
        &mut self,
        ty: &IrType,
        config: &Opts,
        all_configs: &AllConfigs,
    ) -> Option<String> {
        let _ir_file = all_configs.get_ir_file(config.block_index).unwrap();
        TypeRustGenerator::new(ty.clone(), self.config, all_configs).static_checks()
    }

    fn generate_wrapper_struct(
        &mut self,
        ty: &IrType,
        _config: &Opts,
        all_configs: &AllConfigs,
    ) -> Option<String> {
        // the generated wrapper structs need to be public for the StreamSinkTrait impl to work
        match ty {
            IrType::StructRef(_)
            | IrType::EnumRef(_)
            | IrType::Delegate(IrTypeDelegate::PrimitiveEnum { .. }) => {
                TypeRustGenerator::new(ty.clone(), self.config, all_configs)
                    .wrapper_struct()
                    .map(|wrapper| {
                        format!(
                            r###"
                            #[derive(Clone)]
                            pub struct {}({});
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

    fn generate_new_with_nullptr_func(&mut self, ty: &IrType, all_configs: &AllConfigs) -> String {
        TypeRustGenerator::new(ty.clone(), self.config, all_configs)
            .new_with_nullptr(&mut self.extern_func_collector)
    }

    fn generate_impl_intodart(&mut self, ty: &IrType, all_configs: &AllConfigs) -> String {
        TypeRustGenerator::new(ty.clone(), self.config, all_configs).impl_intodart()
    }

    fn generate_wasm2api_func(&self, ty: &IrType, all_configs: &AllConfigs) -> Option<String> {
        TypeRustGenerator::new(ty.clone(), self.config, all_configs)
            .wire2api_jsvalue()
            .map(|body| {
                format!(
                    "impl Wire2Api<{api}> for JsValue {{
                        fn wire2api(self) -> {api} {{
                            {}
                        }}
                    }}",
                    body,
                    api = ty.rust_api_type()
                )
            })
    }
}

pub fn generate_import(
    api_type: &IrType,
    config: &Opts,
    all_configs: &AllConfigs,
) -> Option<String> {
    TypeRustGenerator::new(api_type.clone(), config, all_configs).imports()
}

pub fn generate_list_allocate_func(
    collector: &mut ExternFuncCollector,
    safe_ident: &str,
    list: &impl IrTypeTrait,
    inner: &IrType,
) -> String {
    // let wasm = false;
    collector.generate(
        &format!("new_{safe_ident}"),
        [("len: i32", "int")],
        Some(
            &[
                list.rust_wire_modifier(Target::Io).as_str(),
                list.rust_wire_type(Target::Io).as_str(),
            ]
            .concat(),
        ),
        &format!(
            "let wrap = {} {{ ptr: support::new_leak_vec_ptr({}, len), len }};
                         support::new_leak_box_ptr(wrap)",
            list.rust_wire_type(Target::Io),
            // "Default::default()".to_string()
            // if inner.has_default_trait() {
            if inner.is_primitive() {
                // A primitive enum list can use a default value since
                // `<i32>::new_with_null_ptr()` isn't implemented.
                "Default::default()".to_string()
            } else {
                format!(
                    "<{}{}>::new_with_null_ptr()",
                    inner.rust_ptr_modifier(),
                    inner.rust_wire_type(Target::Io)
                )
            }
        ),
        Io,
    )
}

pub const NO_PARAMS: Option<(&str, &str)> = None;

pub struct ExternFuncCollector {
    names: Vec<String>,
    wasm_exports: Vec<IrFuncDisplay>,
}

impl ExternFuncCollector {
    fn new() -> Self {
        ExternFuncCollector {
            names: Default::default(),
            wasm_exports: vec![],
        }
    }

    /// Functions starting with "wire_" are assumed to be from the original set of IrFuncs
    /// and not re-exported to WASM.
    fn generate(
        &mut self,
        func_name: &str,
        params: impl IntoIterator<Item = (impl Display, impl Display)>,
        return_type: Option<&str>,
        body: &str,
        target: Target,
    ) -> String {
        let params = params.into_iter().collect::<Vec<_>>();
        if matches!(target, Io) {
            self.names.push(func_name.to_string());
        } else if target.is_wasm() && !func_name.starts_with("wire_") {
            self.wasm_exports.push(IrFuncDisplay {
                name: func_name.to_owned(),
                inputs: params
                    .iter()
                    .map(|(verbatim, dart)| {
                        let verbatim = format!("{verbatim}");
                        let (key, _) = verbatim.split_once(':').expect("Missing middle colon");
                        IrParam {
                            name: key.to_owned(),
                            ty: format!("{dart}"),
                        }
                    })
                    .collect(),
                output: return_type.map(String::from).unwrap_or_default(),
                has_port_argument: false,
            })
        }

        format!(
            r#"
                {attr}
                pub {call_conv} fn {}({}) {} {{
                    {}
                }}
            "#,
            func_name,
            params.into_iter().map(|param| param.0).join(","),
            return_type.map_or("".to_string(), |r| format!("-> {r}")),
            body,
            attr = target.extern_func_attr(),
            call_conv = target.call_convention(),
        )
    }
}

pub fn get_into_into_dart(name: impl Display, wrapper_name: Option<impl Display>) -> String {
    match wrapper_name {
        None => {
            // case for types without mirror_ wrapper
            format!(
                "impl rust2dart::IntoIntoDart<{name}> for {name} {{
                fn into_into_dart(self) -> Self {{
                    self
                }}
            }}"
            )
        }
        Some(wrapper) => {
            // case for type with mirror_ wrapper
            format!(
                "impl rust2dart::IntoIntoDart<{wrapper}> for {name} {{
                fn into_into_dart(self) -> {wrapper} {{
                    {wrapper}(self)
                }}
            }}"
            )
        }
    }
}
