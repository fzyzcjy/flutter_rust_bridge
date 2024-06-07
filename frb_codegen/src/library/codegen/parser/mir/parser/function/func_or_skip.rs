use itertools::Itertools;
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

    pub(crate) fn split(items: Vec<MirFuncOrSkip>) -> (Vec<MirFunc>, Vec<MirSkip>) {
        let (funcs, skips): (Vec<_>, Vec<_>) =
            (items.into_iter()).partition(|item| matches!(item, MirFuncOrSkip::Ok(_)));
        let funcs = funcs.into_iter().map(|x| x.ok()).collect_vec();
        let skips = skips.into_iter().map(|x| x.skip()).collect_vec();
        (funcs, skips)
    }
}
