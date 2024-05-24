use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::func::IrFunc;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::misc::extract_src_types_in_paths;
use crate::codegen::parser::source_graph::modules::Struct;
use crate::codegen::parser::type_parser::{TypeParser, TypeParserParsingContext};
use itertools::Itertools;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(crate) fn parse_auto_accessors(
    src_structs: &HashMap<String, &Struct>,
    type_parser: &mut TypeParser,
    rust_input_paths: &[PathBuf],
    rust_crate_dir: &Path,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
) -> anyhow::Result<Vec<IrFunc>> {
    let src_structs_in_paths =
        extract_src_types_in_paths(src_structs, rust_input_paths, rust_crate_dir)?;
    Ok(src_structs_in_paths
        .iter()
        .map(|struct_name| {
            parse_auto_accessors_of_struct(
                struct_name,
                type_parser,
                default_stream_sink_codec,
                default_rust_opaque_codec,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec())
}

fn parse_auto_accessors_of_struct(
    struct_name: &NamespacedName,
    type_parser: &mut TypeParser,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
) -> anyhow::Result<Vec<IrFunc>> {
    if !is_struct_opaque(
        struct_name,
        type_parser,
        default_stream_sink_codec,
        default_rust_opaque_codec,
    )? {
        return Ok(vec![]);
    }

    todo!()
}

fn is_struct_opaque(
    struct_name: &NamespacedName,
    type_parser: &mut TypeParser,
    default_stream_sink_codec: CodecMode,
    default_rust_opaque_codec: RustOpaqueCodecMode,
) -> anyhow::Result<bool> {
    let context = TypeParserParsingContext {
        initiated_namespace: struct_name.namespace.to_owned(),
        func_attributes: FrbAttributes::parse(&[])?,
        default_stream_sink_codec,
        default_rust_opaque_codec,
        owner: None,
    };

    let ty = type_parser.parse_type(&syn::parse_str(&struct_name.name)?, &context)?;
    Ok(matches!(ty, IrType::RustAutoOpaqueImplicit(_)))
}
