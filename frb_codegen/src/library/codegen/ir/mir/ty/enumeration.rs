// Name "enumeration" not "enum", since the latter is a keyword

use crate::codegen::ir::mir::comment::MirComment;
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::structure::MirStruct;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::{Namespace, NamespacedName};
use convert_case::{Case, Casing};

crate::mir! {
pub struct MirTypeEnumRef {
    pub ident: MirEnumIdent,
    pub is_exception: bool,
}

pub struct MirEnumIdent(pub NamespacedName);

pub struct MirEnum {
    pub name: NamespacedName,
    pub wrapper_name: Option<String>,
    pub comments: Vec<MirComment>,
    pub variants: Vec<MirEnumVariant>,
    pub mode: MirEnumMode,
    pub ignore: bool,
    pub needs_json_serializable: bool,
}

#[derive(Copy)]
pub enum MirEnumMode {
    Simple,
    Complex,
}

pub struct MirEnumVariant {
    pub name: MirIdent,
    pub wrapper_name: MirIdent,
    pub comments: Vec<MirComment>,
    pub kind: MirVariantKind,
}

pub enum MirVariantKind {
    Value,
    Struct(MirStruct),
}
}

impl MirTypeEnumRef {
    #[inline]
    pub fn get<'a>(&self, file: &'a impl MirContext) -> &'a MirEnum {
        (file.enum_pool().get(&self.ident))
            .unwrap_or_else(|| panic!("enum_pool does not contain {self:?}"))
    }
}

impl MirTypeTrait for MirTypeEnumRef {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        let enu = self.get(mir_context);
        for variant in enu.variants() {
            if let MirVariantKind::Struct(st) = &variant.kind {
                st.fields
                    .iter()
                    .for_each(|field| field.ty.visit_types(f, mir_context));
            }
        }
    }

    fn safe_ident(&self) -> String {
        self.ident.0.name.to_case(Case::Snake)
    }

    fn rust_api_type(&self) -> String {
        self.ident.0.rust_style()
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.ident.0.namespace.clone())
    }

    fn should_ignore(&self, mir_context: &impl MirContext) -> bool {
        self.get(mir_context).ignore
    }
}

impl MirEnum {
    pub fn variants(&self) -> &[MirEnumVariant] {
        &self.variants
    }
}

impl MirVariantKind {
    pub(crate) fn fields(&self) -> Vec<MirField> {
        match self {
            MirVariantKind::Value => vec![],
            MirVariantKind::Struct(st) => st.fields.clone(),
        }
    }
}

impl From<NamespacedName> for MirEnumIdent {
    fn from(value: NamespacedName) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::ir::mir::field::{MirField, MirFieldSettings};
    use crate::codegen::ir::mir::ident::MirIdent;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::structure::MirStruct;

    /// Preserves enum identity, namespace, and struct variant fields.
    #[test]
    fn enum_reference_and_variant_fields_keep_their_contracts() {
        let namespace = Namespace::new_raw("crate::api".to_owned());
        let enum_ref = MirTypeEnumRef {
            ident: MirEnumIdent(NamespacedName::new(
                namespace.clone(),
                "HttpStatus".to_owned(),
            )),
            is_exception: false,
        };
        let value_fields = MirVariantKind::Value.fields();
        let struct_fields = MirVariantKind::Struct(MirStruct {
            name: NamespacedName::new(namespace.clone(), "StatusPayload".to_owned()),
            wrapper_name: None,
            fields: vec![MirField {
                ty: MirType::Primitive(MirTypePrimitive::I32),
                name: MirIdent::new("code".to_owned(), None),
                is_final: false,
                is_rust_public: None,
                comments: vec![],
                default: None,
                settings: MirFieldSettings::default(),
            }],
            is_fields_named: true,
            dart_metadata_raw: vec![],
            ignore: false,
            needs_json_serializable: false,
            generate_hash: false,
            generate_eq: false,
            dart_collection_deep_equality: false,
            ui_state: false,
            comments: vec![],
        })
        .fields();

        assert_eq!(enum_ref.safe_ident(), "http_status");
        assert_eq!(enum_ref.rust_api_type(), "crate::api::HttpStatus");
        assert_eq!(enum_ref.self_namespace(), Some(namespace));
        assert!(value_fields.is_empty());
        assert!(matches!(
            struct_fields.as_slice(),
            [MirField {
                ty: MirType::Primitive(MirTypePrimitive::I32),
                ..
            }]
        ));
    }
}
