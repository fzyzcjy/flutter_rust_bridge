use crate::codegen::ir::mir::func::MirFunc;

pub(crate) enum ParseFunctionOutput {
    Ok(MirFunc),
    Skipped,
    Err(String),
}
