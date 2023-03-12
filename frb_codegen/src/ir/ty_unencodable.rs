use crate::{ir::*, target::Target};
use quote::ToTokens;

// use convert_case::{Case, Casing};
// use lazy_static::lazy_static;
// use quote::ToTokens;
// use regex::Regex;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ArgsRefs<'a> {
    Generic(&'a [IrType]),
    Signature(&'a [IrType]),
}

pub trait ArgsUnpackable {
    fn unpack(&self) -> Vec<(&str, Option<ArgsRefs>)>;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct IrTypeUnencodable {
    pub underlying_type: Box<syn::Type>,
    pub segments: Vec<(String, Option<Args>)>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Args {
    Generic(Vec<IrType>),
    Signature(Vec<IrType>),
}

impl ArgsUnpackable for Vec<(String, Option<Args>)> {
    fn unpack(&self) -> Vec<(&str, Option<ArgsRefs>)> {
        return self
            .iter()
            .map(|(ident, maybe_args)| {
                (
                    &ident[..],
                    maybe_args.as_ref().map(|args| match &args {
                        Args::Generic(types) => ArgsRefs::Generic(&types[..]),
                        Args::Signature(types) => ArgsRefs::Signature(&types[..]),
                    }),
                )
            })
            .collect();
    }
}

// fn char_not_alphanumeric(c: char) -> bool {
//     !c.is_alphanumeric()
// }

// fn rust_type_to_dart_type(rust: &str) -> String {
//     lazy_static! {
//         static ref OPAQUE_FILTER: Regex =
//             Regex::new(r"(\bdyn|'static|\bDartSafe|\+ (Send|Sync|UnwindSafe|RefUnwindSafe))\b")
//                 .unwrap();
//     }
//     OPAQUE_FILTER
//         .replace_all(rust, "")
//         .replace(char_not_alphanumeric, "_")
//         .to_case(Case::Pascal)
// }

impl IrTypeTrait for IrTypeUnencodable {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {
        // Do nothing.
    }

    fn safe_ident(&self) -> String {
        "UNENCODABLE_SAFE_IDENT".to_string()
    }

    fn dart_api_type(&self) -> String {
        "UNENCODABLE_DART_API_TYPE".to_string()
    }

    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "Object".to_owned()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.underlying_type.to_token_stream().to_string()
    }

    fn rust_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }
}
