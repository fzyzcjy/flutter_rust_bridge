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

    fn generate() -> Acc<String> {}

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

        // if self.config.block_index == BlockIndex::PRIMARY {
        (lines.io).push(self.section_header_comment("sync execution mode utility"));
        // TODO
        // lines.io.push(self.generate_sync_execution_mode_utility());
        // }
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
}
