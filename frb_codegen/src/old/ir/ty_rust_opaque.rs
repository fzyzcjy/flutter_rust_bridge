use crate::{ir::*, target::Target};

use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use quote::ToTokens;
use regex::Regex;

fn char_not_alphanumeric(c: char) -> bool {
    !c.is_alphanumeric()
}

fn rust_type_to_dart_type(rust: &str) -> String {
    lazy_static! {
        static ref OPAQUE_FILTER: Regex =
            Regex::new(r"(\bdyn|'static|\bDartSafe|\+ (Send|Sync|UnwindSafe|RefUnwindSafe))\b")
                .unwrap();
    }
    OPAQUE_FILTER
        .replace_all(rust, "")
        .replace(char_not_alphanumeric, "_")
        .to_case(Case::Pascal)
}

impl From<&syn::Type> for IrTypeRustOpaque {
    fn from(rust_ty: &syn::Type) -> Self {
        let inner_dart = match rust_ty {
            syn::Type::Tuple(tup) if tup.elems.is_empty() => "void".to_owned(),
            _ => rust_type_to_dart_type(&rust_ty.into_token_stream().to_string()),
        };

        Self {
            inner_rust: rust_ty.to_token_stream().to_string(),
            inner_dart,
        }
    }
}

impl From<String> for IrTypeRustOpaque {
    fn from(inner_rust: String) -> Self {
        let inner_dart = rust_type_to_dart_type(&inner_rust);
        Self {
            inner_rust,
            inner_dart,
        }
    }
}

impl IrTypeRustOpaque {
    pub fn new_unit() -> Self {
        Self {
            inner_rust: "()".to_owned(),
            inner_dart: "void".to_owned(),
        }
    }
}

impl IrTypeTrait for IrTypeRustOpaque {
    fn dart_wire_type(&self, target: crate::target::Target) -> String {
        if let Target::Wasm = target {
            "Object".to_owned()
        } else {
            self.rust_wire_type(target)
        }
    }
}
