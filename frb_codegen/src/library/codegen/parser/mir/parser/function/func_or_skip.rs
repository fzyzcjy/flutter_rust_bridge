use crate::codegen::ir::mir::func::MirFunc;
use crate::codegen::ir::misc::skip::IrSkip;
use itertools::Itertools;

pub(crate) enum MirFuncOrSkip {
    Func(MirFunc),
    Skip(IrSkip),
}

impl MirFuncOrSkip {
    pub(crate) fn func(self) -> MirFunc {
        match self {
            Self::Func(inner) => inner,
            _ => unreachable!(),
        }
    }

    pub(crate) fn skip(self) -> IrSkip {
        match self {
            Self::Skip(inner) => inner,
            _ => unreachable!(),
        }
    }

    pub(crate) fn split(items: Vec<MirFuncOrSkip>) -> (Vec<MirFunc>, Vec<IrSkip>) {
        let (funcs, skips): (Vec<_>, Vec<_>) =
            (items.into_iter()).partition(|item| matches!(item, MirFuncOrSkip::Func(_)));
        let funcs = funcs.into_iter().map(|x| x.func()).collect_vec();
        let skips = skips.into_iter().map(|x| x.skip()).collect_vec();
        (funcs, skips)
    }
}
