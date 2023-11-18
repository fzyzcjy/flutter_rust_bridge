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

use crate::ir::IrType::*;
use crate::others::*;
use crate::target::Acc;
use crate::target::Target;
use crate::target::Target::*;
use crate::utils::method::FunctionName;
use crate::utils::misc::BlockIndex;
use crate::{ir::*, Opts};
use itertools::Itertools;

pub struct Output {
    pub code: Acc<String>,
    pub extern_func_names: Vec<String>,
    pub wasm_exports: Vec<IrFuncDisplay>,
}

impl Output {
    pub fn get_exclude_symbols(&self, all_symbols: &[String]) -> Vec<String> {
        all_symbols
            .iter()
            .filter(|s| !self.extern_func_names.contains(s))
            .map(|s| (*s).clone())
            .collect_vec()
    }
}

pub fn generate(ir_pack: &IrPack, rust_wire_mod: &str, config: &Opts) -> Output {
    let mut generator = Generator::new(config);
    let code = generator.generate(ir_pack, rust_wire_mod);

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

    fn generate(&mut self, ir_pack: &IrPack, rust_wire_mod: &str) -> Acc<String> {
        let mut lines = Acc::<Vec<_>>::default();

        let distinct_input_types = ir_pack.distinct_types(true, false);
        let distinct_output_types = ir_pack.distinct_types(false, true);

        lines.push(r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, clippy::unit_arg, clippy::double_parens, non_snake_case, clippy::too_many_arguments)]"#.to_string());
        lines.push(code_header());

        lines.push(String::new());
        lines.push(format!("use crate::{rust_wire_mod}::*;"));
        lines.push("use flutter_rust_bridge::*;".to_owned());
        lines.push("use core::panic::UnwindSafe;".to_owned());
        lines.push("use std::sync::Arc;".to_owned());
        lines.push("use std::ffi::c_void;".to_owned());
        lines.push("use flutter_rust_bridge::rust2dart::IntoIntoDart;".to_owned());
        lines.push(String::new());

        lines.push(self.section_header_comment("imports"));
        lines.extend(self.generate_imports(
            ir_pack,
            rust_wire_mod,
            &distinct_input_types,
            &distinct_output_types,
        ));
        lines.push(String::new());

        lines.push_all(self.section_header_comment("wire functions"));
        lines += ir_pack
            .funcs
            .iter()
            .map(|f| self.generate_wire_func(f, ir_pack))
            .collect();

        lines.push(self.section_header_comment("wrapper structs"));
        // TODO
        // lines.extend(
        //     distinct_output_types
        //         .iter()
        //         .filter_map(|ty| self.generate_wrapper_struct_name(ty, ir_pack)),
        // );

        lines.push(self.section_header_comment("static checks"));
        // TODO why only `distinct_output_types`, not input types?
        // TODO
        // let static_checks = distinct_output_types
        //     .iter()
        //     .filter_map(|ty| self.generate_static_checks(ty, ir_pack))
        //     .collect_vec();

        lines.push_all(self.section_header_comment("allocate functions"));
        lines += distinct_input_types
            .iter()
            .map(|f| self.generate_allocate_funcs(f, ir_pack))
            .collect();

        lines.push_all(self.section_header_comment("related functions"));
        lines += distinct_output_types
            .iter()
            .map(|f| self.generate_related_funcs(f, ir_pack))
            .collect();

        lines.push_all(self.section_header_comment("impl Wire2Api"));
        // TODO -> generate_impl_wire2api
        // lines.push(self.generate_wire2api_misc().to_string());
        // lines += distinct_input_types
        //     .iter()
        //     .map(|ty| self.generate_wire2api_func(ty, ir_pack))
        //     .collect();

        lines.push(self.section_header_comment("impl IntoDart"));
        // TODO -> generate_impl_into_dart
        // lines.extend(
        //     distinct_output_types
        //         .iter()
        //         .map(|ty| self.generate_impl_intodart(ty, ir_pack)),
        // );

        lines.push(self.section_header_comment("executor"));
        lines.push(self.generate_executor(ir_pack));

        self.generate_io_part(&mut lines, &distinct_input_types, ir_pack);
        self.generate_wasm_part(&mut lines, &distinct_input_types, ir_pack);

        lines.join("\n")
    }

    fn generate_io_part(
        &mut self,
        lines: &mut Acc<Vec<String>>,
        distinct_input_types: &[IrType],
        ir_pack: &IrPack,
    ) {
        lines.io.push(self.section_header_comment("wire structs"));
        // TODO these become: wire::rust::class::generate
        // lines.io.extend(
        //     distinct_input_types
        //         .iter()
        //         .map(|ty| self.generate_wire_struct(ty, ir_pack)),
        // );
        // lines.io.extend(
        //     distinct_input_types
        //         .iter()
        //         .map(|ty| TypeRustGenerator::new(ty.clone(), ir_pack, self.config).structs()),
        // );

        (lines.io).push(self.section_header_comment("impl NewWithNullPtr"));
        // TODO
        // (lines.io).push(self.generate_new_with_nullptr_misc().to_string());
        // lines.io.extend(
        //     distinct_input_types
        //         .iter()
        //         .map(|ty| self.generate_new_with_nullptr_func(ty, ir_pack)),
        // );

        if self.config.block_index == BlockIndex::PRIMARY {
            (lines.io).push(self.section_header_comment("sync execution mode utility"));
            lines.io.push(self.generate_sync_execution_mode_utility());
        }
    }

    fn generate_wasm_part(
        &mut self,
        lines: &mut Acc<Vec<String>>,
        distinct_input_types: &[IrType],
        ir_pack: &IrPack,
    ) {
        // moved
        // (lines.wasm).push(self.section_header_comment("impl Wire2Api for JsValue"));
        // (lines.wasm).push(
        //     "impl<T> Wire2Api<Option<T>> for JsValue where JsValue: Wire2Api<T> {
        //         fn wire2api(self) -> Option<T> {
        //             (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
        //         }
        //     }"
        //     .into(),
        // );

        // TODO moved
        // lines.wasm.extend(
        //     distinct_input_types
        //         .iter()
        //         .filter_map(|ty| self.generate_wasm2api_func(ty, ir_pack)),
        // );
    }

    fn section_header_comment(&self, section_name: &str) -> String {
        format!("// Section: {section_name}\n")
    }

    fn generate_imports(
        &self,
        ir_pack: &IrPack,
        rust_wire_mod: &str,
        distinct_input_types: &[IrType],
        distinct_output_types: &[IrType],
    ) -> impl Iterator<Item = String> {
        let input_type_imports = distinct_input_types
            .iter()
            .map(|api_type| generate_import(api_type, ir_pack, self.config));
        let output_type_imports = distinct_output_types
            .iter()
            .map(|api_type| generate_import(api_type, ir_pack, self.config));

        input_type_imports
            .chain(output_type_imports)
            // Filter out `None` and unwrap
            .flatten()
            // Don't include imports from the API file
            .filter(|import| !import.starts_with(&format!("use crate::{rust_wire_mod}::")))
            // de-duplicate
            .collect::<HashSet<String>>()
            .into_iter()
    }

    fn generate_executor(&mut self, ir_pack: &IrPack) -> String {
        if ir_pack.has_executor {
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

    fn generate_wire_func(&mut self, func: &IrFunc, ir_pack: &IrPack) -> Acc<String> {
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
        params += (func.inputs)
            .iter()
            .map(|field| {
                let name = field.name.rust_style();
                Acc::new(|target| match target {
                    Common => format!(
                        "{}: impl Wire2Api<{}> + UnwindSafe",
                        name,
                        field.ty.rust_api_type()
                    ),
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
                .collect_vec(),
        ]
        .concat();
        if let IrFuncMode::Stream { argument_index } = func.mode {
            inner_func_params.insert(
                argument_index,
                format!(
                    "task_callback.stream_sink::<_,{}>()",
                    func.output.intodart_type(ir_pack)
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
            .collect_vec()
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
                    func.output.intodart_type(ir_pack)
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
                .collect_vec()
                .join(","),
        );
        Acc::new(|target| match target {
            Io | Wasm => self.extern_func_collector.generate(
                &func.wire_func_name(),
                if target == Target::Wasm {
                    &params.wasm[..]
                } else {
                    &params.io[..]
                }
                .iter()
                .map(|item| (item, ""))
                .collect_vec(),
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

    fn generate_allocate_funcs(&mut self, ty: &IrType, ir_pack: &IrPack) -> Acc<String> {
        TypeRustGenerator::new(ty.clone(), ir_pack, self.config)
            .allocate_funcs(&mut self.extern_func_collector, self.config.block_index)
            .map(|func, _| func.unwrap_or_default())
    }

    fn generate_related_funcs(&mut self, ty: &IrType, ir_pack: &IrPack) -> Acc<String> {
        TypeRustGenerator::new(ty.clone(), ir_pack, self.config)
            .related_funcs(&mut self.extern_func_collector, self.config.block_index)
            .map(|func, _| func.unwrap_or_default())
    }
}

pub fn generate_import(api_type: &IrType, ir_pack: &IrPack, config: &Opts) -> Option<String> {
    TypeRustGenerator::new(api_type.clone(), ir_pack, config).imports()
}

pub fn generate_list_allocate_func(
    collector: &mut ExternFuncCollector,
    safe_ident: &str,
    list: &impl IrTypeTrait,
    inner: &IrType,
    block_index: BlockIndex,
) -> String {
    // let wasm = false;
    collector.generate(
        &format!("new_{safe_ident}_{block_index}"),
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
            if inner.is_primitive() {
                // A primitive enum list can use a default value since
                // `<i32>::new_with_null_ptr()` isn't implemented.
                "Default::default()".to_string()
            } else {
                format!(
                    "<{}{}>::new_with_null_ptr()",
                    general_list_maybe_extra_pointer_indirection(IrTypeGeneralList { inner }),
                    inner.rust_wire_type(Target::Io)
                )
            }
        ),
        Io,
    )
}

pub const NO_PARAMS: Option<(&str, &str)> = None;
