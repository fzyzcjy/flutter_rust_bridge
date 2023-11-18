use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::misc::code_header;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::common::generate_imports;
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

pub(crate) mod api2wire;
pub(crate) mod base;
pub(crate) mod common;
pub(crate) mod wire2api;
mod wire_func;

pub(crate) fn generate(
    ir_pack: &IrPack,
    context: WireRustGeneratorContext,
    // TODO this should be in config?
    rust_wire_mod: &str,
) -> Acc<String> {
    let mut lines = Acc::<Vec<_>>::default();

    let distinct_input_types = ir_pack.distinct_types(true, false);
    let distinct_output_types = ir_pack.distinct_types(false, true);

    lines.push(r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, clippy::unit_arg, clippy::double_parens, non_snake_case, clippy::too_many_arguments)]"#.to_string());
    lines.push(code_header());

    lines.push(generate_imports(
        distinct_input_types
            .iter()
            .chain(distinct_output_types.iter()),
        context,
        rust_wire_mod,
    ));

    lines.push_all(self.section_header_comment("wire functions"));
    // TODO
    // lines += ir_pack
    //     .funcs
    //     .iter()
    //     .map(|f| self.generate_wire_func(f, ir_pack))
    //     .collect();

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
    // TODO
    // lines += distinct_input_types
    //     .iter()
    //     .map(|f| self.generate_generate_allocate_funcs(f, ir_pack))
    //     .collect();

    lines.push_all(self.section_header_comment("related functions"));
    // TODO
    // lines += distinct_output_types
    //     .iter()
    //     .map(|f| self.generate_generate_related_funcs(f, ir_pack))
    //     .collect();

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
