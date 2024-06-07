use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::parser::hir::flat::parser::syn_item::parse_syn_item;

pub(crate) fn inject_extra_code(pack: &mut HirFlatPack, extra_code: &str) -> anyhow::Result<()> {
    pack.extra_code += extra_code;

    for syn_item in TODO {
        parse_syn_item()?;
    }

    Ok(())
}
