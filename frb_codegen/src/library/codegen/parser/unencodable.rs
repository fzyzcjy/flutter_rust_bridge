use crate::codegen::ir::ty::unencodable::{Args, NameComponent};
use crate::codegen::ir::ty::IrType;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub(crate) enum ArgsRefs<'a> {
    Generic(&'a [IrType]),
    Signature(&'a [IrType]),
}

pub(crate) trait Splayable {
    fn splay(&self) -> Vec<(&str, Option<ArgsRefs>)>;
}

impl Splayable for Vec<NameComponent> {
    /// Spread and turn out the data of a fully qualified name for structural pattern matching.
    fn splay(&self) -> Vec<(&str, Option<ArgsRefs>)> {
        return self
            .iter()
            .map(|NameComponent { ident, args }| {
                (
                    &ident[..],
                    args.as_ref().map(|args| match &args {
                        Args::Generic(types) => ArgsRefs::Generic(&types[..]),
                        Args::Signature(types) => ArgsRefs::Signature(&types[..]),
                    }),
                )
            })
            .collect();
    }
}
