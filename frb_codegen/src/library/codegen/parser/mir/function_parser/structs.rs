use crate::codegen::ir::mir::func::MirFunc;

pub(crate) enum ParseFunctionOutput {
    Ok(MirFunc),
    Skipped(ParseFunctionOutputSkipped),
}

pub(crate) enum ParseFunctionOutputSkipped {
    Ignored,
    Err(String),
}
