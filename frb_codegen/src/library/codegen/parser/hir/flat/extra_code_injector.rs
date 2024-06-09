use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::codegen::parser::hir::flat::parser::syn_item::parse_syn_item;
use crate::utils::namespace::Namespace;
use anyhow::Context;

// just a convenient wrapper around `inject_extra_code`
pub(crate) fn inject_extra_codes(
    pack: &mut HirFlatPack,
    namespace: &Namespace,
    blocks: &[InjectExtraCodeBlock],
) -> anyhow::Result<()> {
    for block in blocks {
        inject_extra_code(pack, namespace, &block.code, block.should_parse)?;
    }
    Ok(())
}

#[derive(Clone)]
pub(crate) struct InjectExtraCodeBlock {
    pub code: String,
    pub should_parse: bool,
}

fn inject_extra_code(
    pack: &mut HirFlatPack,
    namespace: &Namespace,
    extra_code: &str,
    should_parse: bool,
) -> anyhow::Result<()> {
    pack.extra_rust_output_code += extra_code;
    if should_parse {
        parse_synthesized_syn_items(pack, extra_code, namespace)?;
    }
    Ok(())
}

fn parse_synthesized_syn_items(
    pack: &mut HirFlatPack,
    extra_code: &str,
    namespace: &Namespace,
) -> anyhow::Result<()> {
    let meta = HirNaiveFlatItemMeta {
        namespace: namespace.to_owned(),
        sources: vec![HirGenerationSource::Normal],
        is_module_public: true,
    };
    let syn_file = syn::parse_file(extra_code).with_context(|| format!("code={extra_code}"))?;
    for syn_item in syn_file.items {
        parse_syn_item(syn_item, &meta, pack)?;
    }
    Ok(())
}
