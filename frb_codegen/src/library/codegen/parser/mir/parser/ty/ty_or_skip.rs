use crate::codegen::ir::mir::skip::MirSkip;
use crate::codegen::ir::mir::ty::MirType;

pub(crate) enum MirTypeOrSkip {
    Type(MirType),
    Skip(MirSkip),
}

impl MirTypeOrSkip {
    pub(crate) fn ty(self) -> MirType {
        match self {
            Self::Type(inner) => inner,
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
