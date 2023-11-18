use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::misc::generate_code_header;
use crate::codegen::generator::wire::rust::base::{WireRustGenerator, WireRustGeneratorContext};
use crate::codegen::generator::wire::rust::misc::wire_func::generate_wire_func;
use crate::codegen::generator::wire::rust::misc::{
    generate_imports, generate_static_checks, generate_wrapper_struct,
};
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

pub(crate) mod api2wire;
pub(crate) mod base;
mod internal_config;
pub(crate) mod misc;
pub(crate) mod wire2api;

// TODO
// pub(crate) fn generate(ir_pack: &IrPack, context: WireRustGeneratorContext) -> Acc<String> {
//     let mut lines = Acc::<Vec<_>>::default();
//
//     let distinct_input_types = ir_pack.distinct_types(true, false);
//     let distinct_output_types = ir_pack.distinct_types(false, true);
//     let input_and_output_types = distinct_input_types
//         .iter()
//         .cloned()
//         .chain(distinct_output_types.iter().cloned())
//         .collect_vec();
//
//     lines.push(FILE_ATTRIBUTES.to_string());
//     lines.push(generate_code_header());
//
//     lines.push(section_header_comment("imports"));
//     lines.push(generate_imports(&input_and_output_types, context));
//
//     lines.push(section_header_comment("wire functions"));
//     lines += ir_pack
//         .funcs
//         .iter()
//         .map(|f| generate_wire_func(f, context))
//         .collect();
//
//     lines.push(section_header_comment("wrapper structs"));
//     lines.extend(
//         distinct_output_types
//             .iter()
//             .filter_map(|ty| generate_wrapper_struct(ty, context)),
//     );
//
//     lines.push(section_header_comment("static checks"));
//     lines.push(generate_static_checks(&input_and_output_types, context));
//
//     lines.push(section_header_comment("allocate functions"));
//     lines += distinct_input_types
//         .iter()
//         .map(|ty| WireRustGenerator::new(ty, context).generate_allocate_funcs())
//         .collect();
//
//     lines.push(section_header_comment("related functions"));
//     // TODO
//     // lines += distinct_output_types
//     //     .iter()
//     //     .map(|f| generate_generate_related_funcs(f, ir_pack))
//     //     .collect();
//
//     lines.push(section_header_comment("impl Wire2Api"));
//     // TODO -> generate_impl_wire2api
//     // lines.push(generate_wire2api_misc().to_string());
//     // lines += distinct_input_types
//     //     .iter()
//     //     .map(|ty| generate_wire2api_func(ty, ir_pack))
//     //     .collect();
//
//     lines.push(section_header_comment("impl IntoDart"));
//     // TODO -> generate_impl_into_dart
//     // lines.extend(
//     //     distinct_output_types
//     //         .iter()
//     //         .map(|ty| generate_impl_intodart(ty, ir_pack)),
//     // );
//
//     lines.push(section_header_comment("executor"));
//     lines.push(generate_executor(ir_pack));
//
//     generate_io_part(&mut lines, &distinct_input_types, ir_pack);
//     generate_wasm_part(&mut lines, &distinct_input_types, ir_pack);
//
//     lines.join("\n")
// }
//
// const FILE_ATTRIBUTES: &'static str = r#"#![allow(non_camel_case_types, unused, clippy::redundant_closure, clippy::useless_conversion, clippy::unit_arg, clippy::double_parens, non_snake_case, clippy::too_many_arguments)]"#;
//
// fn section_header_comment(section_name: &str) -> String {
//     format!("// Section: {section_name}\n")
// }
