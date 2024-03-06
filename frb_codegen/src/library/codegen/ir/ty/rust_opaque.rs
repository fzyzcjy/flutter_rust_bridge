use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use itertools::Itertools;
use lazy_static::lazy_static;
use quote::ToTokens;
use regex::Regex;
use serde::{Deserialize, Serialize, Serializer};
use strum_macros::{Display, EnumIter};
use syn::Type;

crate::ir! {
pub struct IrTypeRustOpaque {
    pub namespace: Namespace,
    pub inner: IrRustOpaqueInner,
    pub codec: RustOpaqueCodecMode,
    pub brief_name: bool,
}

pub struct IrRustOpaqueInner(pub String);
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

impl IrTypeRustOpaque {
    pub fn new(
        namespace: Namespace,
        inner: IrRustOpaqueInner,
        codec: RustOpaqueCodecMode,
        brief_name: bool,
    ) -> Self {
        Self {
            namespace,
            inner,
            codec,
            brief_name,
        }
    }

    pub(crate) fn get_delegate(&self) -> IrType {
        Self::DELEGATE_TYPE.clone()
    }

    pub(crate) const DELEGATE_TYPE: IrType = IrType::Primitive(IrTypePrimitive::Usize);
}

impl IrTypeTrait for IrTypeRustOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.get_delegate().visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        format!("RustOpaque_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("RustOpaque{}<{}>", self.codec, self.inner.0)
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.namespace.clone())
    }

    // Because we are using usize on the wirre
    fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        Some(&IrTypePrimitive::Usize)
    }
}

impl IrRustOpaqueInner {
    pub(crate) fn safe_ident(&self) -> String {
        lazy_static! {
            static ref NEG_FILTER: Regex = Regex::new(r"[^a-zA-Z0-9_]").unwrap();
        }
        NEG_FILTER.replace_all(&self.0, "").into_owned()
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
