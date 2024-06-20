use crate::codegen::ir::mir::func::MirFunc;
use crate::utils::namespace::NamespacedName;
use itertools::Itertools;

crate::mir! {
pub struct IrSkip {
    pub name: NamespacedName,
    pub reason: IrSkipReason,
}

#[derive(Copy, PartialOrd, Ord)]
pub(crate) enum IrSkipReason {
    IgnoreBecauseFunctionNotPub,
    IgnoreBecauseFunctionGeneric,
    IgnoreBecauseTypeNotUsedByPub,
    IgnoreBecauseNotDefinedTrait,
    IgnoreBecauseExplicitAttribute,
    IgnoreBecauseType,
    IgnoreBecauseParseMethodOwnerTy,
    IgnoreBecauseParseOwnerCannotFindTrait,
    IgnoreBecauseNotAllowedOwner,
    IgnoreBecauseOwnerTyShouldIgnore,
    IgnoreBecauseSelfTypeNotAllowed,
    IgnoreSilently,
    Err,
}
}

impl IrSkipReason {
    pub(crate) fn explanation_prefix(&self) -> Option<String> {
        Some(match self {
            Self::IgnoreBecauseFunctionNotPub => {
                "These functions are ignored because they are not marked as `pub`".to_owned()
            }
            Self::IgnoreBecauseFunctionGeneric => {
                "These functions are ignored because they have generic arguments".to_owned()
            }
            Self::IgnoreBecauseTypeNotUsedByPub => {
                "These types are ignored because they are not used by any `pub` functions".to_owned()
            }
            Self::IgnoreBecauseNotDefinedTrait => {
                "These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore)".to_owned()
            }
            Self::IgnoreBecauseParseOwnerCannotFindTrait => {
                "These function are ignored because cannot find trait when parsing owner (put an empty `#[frb]` on it to unignore)".to_owned()
            }
            Self::IgnoreSilently => return None,
            Self::Err => {
                "These functions have error during generation (see debug logs or enable `stop_on_error: true` for more details)".to_owned()
            }
            _ => format!("These functions are ignored (category: {:?})", self)
        })
    }
}

pub(crate) enum IrValueOrSkip<T, S> {
    Value(T),
    Skip(S),
}

impl<T, S> IrValueOrSkip<T, S> {
    pub(crate) fn value(self) -> T {
        match self {
            Self::Value(inner) => inner,
            _ => unreachable!(),
        }
    }

    pub(crate) fn skip(self) -> S {
        match self {
            Self::Skip(inner) => inner,
            _ => unreachable!(),
        }
    }

    pub(crate) fn split(items: Vec<Self>) -> (Vec<T>, Vec<S>) {
        let (values, skips): (Vec<_>, Vec<_>) =
            (items.into_iter()).partition(|item| matches!(item, IrValueOrSkip::Value(_)));
        let values = values.into_iter().map(|x| x.value()).collect_vec();
        let skips = skips.into_iter().map(|x| x.skip()).collect_vec();
        (values, skips)
    }
}

pub type MirFuncOrSkip = IrValueOrSkip<MirFunc, IrSkip>;
