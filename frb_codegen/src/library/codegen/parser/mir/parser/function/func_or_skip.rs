use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::mir::skip::MirSkip;

pub(crate) enum MirFuncOrSkip {
    Ok(MirFunc),
    Skip(MirSkip),
}

impl MirFuncOrSkip {
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
