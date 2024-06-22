use crate::codegen::ir::early_generator::pack::IrEarlyGeneratorPack;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;

pub(crate) fn generate(pack: &mut IrEarlyGeneratorPack) -> anyhow::Result<()> {
    if !should_enable_ui(pack)? {
        return Ok(());
    }
    
    TODO;

    Ok(())
}

fn should_enable_ui(pack: &mut IrEarlyGeneratorPack) -> anyhow::Result<bool> {
    for ty in &pack.hir_flat_pack.structs {
        let attr = FrbAttributes::parse(&ty.src.attrs)?;
        if attr.ui_state() {
            return Ok(true);
        }
    }
    Ok(false)
}
