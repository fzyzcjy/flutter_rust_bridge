use crate::codegen::ir::mir::func::MirFunc;

pub(crate) enum ParseFunctionOutput {
    Ok(MirFunc),
    Skipped,
    Error(anyhow::Error),
}
