// Name "structure" not "struct", since the latter is a keyword

use crate::codegen::mir::annotation::MirDartAnnotation;
use crate::codegen::mir::comment::MirComment;
use crate::codegen::mir::field::MirField;
use crate::codegen::mir::namespace::{Namespace, NamespacedName};
use crate::codegen::mir::ty::{MirContext, MirType, MirTypeTrait};
use convert_case::{Case, Casing};

crate::ir! {
pub struct MirTypeStructRef {
    pub ident: MirStructIdent,
    pub is_exception: bool,
}

pub struct MirStructIdent(pub NamespacedName);

pub struct MirStruct {
    pub name: NamespacedName,
    pub wrapper_name: Option<String>,
    pub fields: Vec<MirField>,
    pub is_fields_named: bool,
    pub dart_metadata: Vec<MirDartAnnotation>,
    pub ignore: bool,
    pub generate_hash: bool,
    pub generate_eq: bool,
    pub comments: Vec<MirComment>,
}
}

impl MirTypeStructRef {
    pub fn get<'a>(&self, ir_context: &'a impl MirContext) -> &'a MirStruct {
        (ir_context.struct_pool().get(&self.ident))
            // frb-coverage:ignore-start
            .unwrap_or_else(|| panic!("no entry found for key={:?}", self.ident))
        // frb-coverage:ignore-end
    }
}

impl MirTypeTrait for MirTypeStructRef {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        _ir_context: &impl MirContext,
    ) {
        for field in &self.get(_ir_context).fields {
            field.ty.visit_types(f, _ir_context);
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
}

impl MirStruct {
    pub fn using_freezed(&self) -> bool {
        self.dart_metadata.iter().any(|it| it.content == "freezed")
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn brackets_pair(&self) -> (char, char) {
        rust_brackets_pair(self.is_fields_named)
    }
}

impl From<NamespacedName> for MirStructIdent {
    fn from(value: NamespacedName) -> Self {
        Self(value)
    }
}

pub fn rust_brackets_pair(keyword_arg: bool) -> (char, char) {
    if keyword_arg {
        ('{', '}')
    } else {
        ('(', ')')
    }
}
