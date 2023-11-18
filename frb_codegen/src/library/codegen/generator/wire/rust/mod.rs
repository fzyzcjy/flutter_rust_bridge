use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::misc::generate_code_header;
use crate::codegen::generator::wire::rust::base::WireRustGeneratorContext;
use crate::codegen::generator::wire::rust::common::generate_imports;
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

pub(crate) mod api2wire;
pub(crate) mod base;
pub(crate) mod common;
mod internal_config;
pub(crate) mod wire2api;
mod wire_func;

pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> Acc<String> {
    let mut lines = Acc::<Vec<_>>::default();

    let distinct_input_types = ir_pack.distinct_types(true, false);
    let distinct_output_types = ir_pack.distinct_types(false, true);
    let input_and_output_types = distinct_input_types
        .iter()
        .chain(distinct_output_types.iter())
        .collect_vec();

    lines.push(FILE_ATTRIBUTES.to_string());
    lines.push(generate_code_header());

    lines.push(section_header_comment("imports"));
    lines.push(generate_imports(input_and_output_types, context));

    lines.push(section_header_comment("wire functions"));
    // TODO
    // lines += ir_pack
    //     .funcs
    //     .iter()
    //     .map(|f| generate_wire_func(f, ir_pack))
    //     .collect();

    lines.push(section_header_comment("wrapper structs"));
    // TODO
    // lines.extend(
    //     distinct_output_types
    //         .iter()
    //         .filter_map(|ty| generate_wrapper_struct_name(ty, ir_pack)),
    // );

    lines.push(section_header_comment("static checks"));
    // TODO why only `distinct_output_types`, not input types?
    // TODO
    // let static_checks = distinct_output_types
    //     .iter()
    //     .filter_map(|ty| generate_static_checks(ty, ir_pack))
    //     .collect_vec();

    lines.push(section_header_comment("allocate functions"));
    // TODO
    // lines += distinct_input_types
    //     .iter()
    //     .map(|f| generate_generate_allocate_funcs(f, ir_pack))
    //     .collect();

    lines.push(section_header_comment("related functions"));
    // TODO
    // lines += distinct_output_types
    //     .iter()
    //     .map(|f| generate_generate_related_funcs(f, ir_pack))
    //     .collect();

    lines.push(section_header_comment("impl Wire2Api"));
    // TODO -> generate_impl_wire2api
    // lines.push(generate_wire2api_misc().to_string());
    // lines += distinct_input_types
    //     .iter()
    //     .map(|ty| generate_wire2api_func(ty, ir_pack))
    //     .collect();

    lines.push(section_header_comment("impl IntoDart"));
    // TODO -> generate_impl_into_dart
    // lines.extend(
    //     distinct_output_types
    //         .iter()
    //         .map(|ty| generate_impl_intodart(ty, ir_pack)),
    // );

    lines.push(section_header_comment("executor"));
    lines.push(generate_executor(ir_pack));

    generate_io_part(&mut lines, &distinct_input_types, ir_pack);
    generate_wasm_part(&mut lines, &distinct_input_types, ir_pack);

    lines.join("\n")
}

const FILE_ATTRIBUTES: &'static str = r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, clippy::unit_arg, clippy::double_parens, non_snake_case, clippy::too_many_arguments)]"#;

fn section_header_comment(section_name: &str) -> String {
    format!("// Section: {section_name}\n")
}
