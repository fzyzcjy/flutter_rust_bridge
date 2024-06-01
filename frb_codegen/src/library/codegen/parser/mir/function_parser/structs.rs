use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::skip::MirSkip;

pub(crate) enum ParseFunctionOutput {
    Ok(MirFunc),
    Skip(MirSkip),
}
