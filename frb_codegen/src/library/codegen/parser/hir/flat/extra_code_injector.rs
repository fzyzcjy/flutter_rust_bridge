use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::misc::generation_source::HirGenerationSource;
use crate::codegen::ir::hir::naive_flat::item::HirNaiveFlatItemMeta;
use crate::codegen::parser::hir::flat::parser::syn_item::parse_syn_item;

pub(crate) fn inject_extra_code(pack: &mut HirFlatPack, extra_code: &str) -> anyhow::Result<()> {
    pack.extra_code += extra_code;
    parse_synthesized_syn_items(pack, extra_code)?;
    Ok(())
}

fn parse_synthesized_syn_items(pack: &mut HirFlatPack, extra_code: &str) -> Result<(), Error> {
    let meta = HirNaiveFlatItemMeta {
        namespace: TODO,
        sources: vec![HirGenerationSource::Normal],
        is_module_public: true,
    };
    let syn_file = syn::parse_file(extra_code)?;
    for syn_item in syn_file.items {
        parse_syn_item(syn_item, &meta, pack)?;
    }
    Ok(())
}
