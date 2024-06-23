use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::Namespace;
use convert_case::{Case, Casing};
use itertools::Itertools;
use lazy_static::lazy_static;
use quote::ToTokens;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use strum_macros::{Display, EnumIter};
use syn::Type;

crate::mir! {
pub struct MirTypeRustOpaque {
    pub namespace: Namespace,
    pub inner: MirRustOpaqueInner,
    pub codec: RustOpaqueCodecMode,
    pub dart_api_type: Option<String>,
    pub brief_name: bool,
}

pub struct MirRustOpaqueInner(pub MirLifetimeAwareType);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, Display, EnumIter)]
pub(crate) enum RustOpaqueCodecMode {
    Nom,
    Moi,
}

impl RustOpaqueCodecMode {
    pub(crate) fn arc_ty(self) -> &'static str {
        match self {
            RustOpaqueCodecMode::Nom => "StdArc",
            RustOpaqueCodecMode::Moi => "MoiArc",
        }
    }

    pub(crate) fn needs_unsafe_block(self) -> bool {
        self == RustOpaqueCodecMode::Nom
    }
}

impl MirTypeRustOpaque {
    pub(crate) fn get_delegate(&self) -> MirType {
        Self::DELEGATE_TYPE.clone()
    }

    pub(crate) const DELEGATE_TYPE: MirType = MirType::Primitive(MirTypePrimitive::Usize);

    pub(crate) fn sanitized_type(&self) -> String {
        rust_type_to_sanitized_type(&self.inner.0.with_static_lifetime(), self.brief_name)
    }
}

impl MirTypeTrait for MirTypeRustOpaque {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.get_delegate().visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!("RustOpaque_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!(
            "RustOpaque{}<{}>",
            self.codec,
            self.inner.0.with_static_lifetime()
        )
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.namespace.clone())
    }

    // Because we are using usize on the wirre
    fn as_primitive(&self) -> Option<&MirTypePrimitive> {
        Some(&RUST_OPAQUE_AS_PRIMITIVE)
    }

    fn cloned_getter_semantics_reasonable(&self) -> bool {
        true
    }
}

pub(super) const RUST_OPAQUE_AS_PRIMITIVE: MirTypePrimitive = MirTypePrimitive::Usize;

impl MirRustOpaqueInner {
    pub(crate) fn safe_ident(&self) -> String {
        lazy_static! {
            static ref NEG_FILTER: Regex = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
        }
        NEG_FILTER
            .replace_all(&self.0.with_static_lifetime(), "")
            .into_owned()
    }
}

// TODO move
/// A component of a fully qualified name and any type arguments for it
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct NameComponent {
    pub ident: String,
    #[serde(serialize_with = "serialize_vec_syn")]
    pub args: Vec<Type>,
}

fn serialize_vec_syn<T: ToTokens, S: Serializer>(values: &[T], s: S) -> Result<S::Ok, S::Error> {
    let str = (values.iter())
        .map(|value| quote::quote!(#value).to_string())
        .join(", ");
    str.serialize(s)
}

fn rust_type_to_sanitized_type(raw: &str, brief_name: bool) -> String {
    lazy_static! {
        static ref OPAQUE_FILTER: Regex = Regex::new(r"((\bdyn|'static|\bDartSafe|\bRustAutoOpaqueInner|\bAssertUnwindSafe|\+ (Send|Sync|UnwindSafe|RefUnwindSafe))\b)|([a-zA-Z0-9_ ]+::)").unwrap();
        static ref OPAQUE_BRIEF_NAME_FILTER: Regex = Regex::new(r"(\bRwLock)\b").unwrap();
    }

    let mut ans = raw.to_owned();

    // ans = replace_all_lifetimes_to_static(&ans);

    ans = OPAQUE_FILTER.replace_all(&ans, "").to_string();

    if brief_name {
        ans = OPAQUE_BRIEF_NAME_FILTER.replace_all(&ans, "").to_string();
    }

    ans.replace(char_not_alphanumeric, "_")
        .to_case(Case::Pascal)
}

fn char_not_alphanumeric(c: char) -> bool {
    !c.is_alphanumeric()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_rust_type_to_sanitized_type() {
        assert_eq!(&rust_type_to_sanitized_type("SomeType", true), "SomeType");
        assert_eq!(
            &rust_type_to_sanitized_type(
                "flutter_rust_bridge::for_generated::rust_async::RwLock<crate::api::simple::AnotherOpaqueType>",
                true
            ),
            "AnotherOpaqueType"
        );
        assert_eq!(&rust_type_to_sanitized_type("flutter_rust_bridge::for_generated::rust_async::RwLock<(crate::api::simple::MyOpaqueType,crate::api::simple::AnotherOpaqueType,)>", true), "MyOpaqueTypeAnotherOpaqueType");
    }
}
