use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::skip::MirSkip;

pub(crate) enum ParseFunctionOutput {
    Ok(MirFunc),
    Skip(MirSkip),
}

impl ParseFunctionOutput {
    pub(crate) fn ok(self) -> MirFunc {
        match self {
            Self::Ok(inner) => inner,
            _ => unreachable!(),
        }
    }

    pub(crate) fn skip(self) -> MirSkip {
        match self {
            Self::Skip(inner) => inner,
            _ => unreachable!(),
        }
    }
}
