use crate::{ir::*, target::Target};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ArgsRefs<'a> {
    Generic(&'a [IrType]),
    Signature(&'a [IrType]),
}

pub trait Splayable {
    fn splay(&self) -> Vec<(&str, Option<ArgsRefs>)>;
}

crate::ir! {
pub enum Args {
    Generic(Vec<IrType>),
    Signature(Vec<IrType>),
}
}

crate::ir! {
/// A component of a fully qualified name and any type arguments for it
pub struct NameComponent {
    pub ident: String,
    pub args: Option<Args>,
}
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

impl IrTypeTrait for IrTypeUnencodable {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {
        // Do nothing.
    }

    fn safe_ident(&self) -> String {
        todo!("generate code for type \"{}\"", self.string);
    }

    fn dart_api_type(&self) -> String {
        todo!("generate code for type \"{}\"", self.string);
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "Object".to_owned()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.string.clone()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }
}
