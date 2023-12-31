// Name "structure" not "struct", since the latter is a keyword

use crate::codegen::ir::annotation::IrDartAnnotation;
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::{Namespace, NamespacedName};
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use convert_case::{Case, Casing};

crate::ir! {
pub struct IrTypeStructRef {
    pub ident: IrStructIdent,
    pub is_exception: bool,
}

pub struct IrStructIdent(pub NamespacedName);

pub struct IrStruct {
    pub name: NamespacedName,
    pub wrapper_name: Option<String>,
    pub fields: Vec<IrField>,
    pub is_fields_named: bool,
    pub dart_metadata: Vec<IrDartAnnotation>,
    pub comments: Vec<IrComment>,
}
}

impl IrTypeStructRef {
    pub(crate) fn get<'a>(&self, ir_context: &'a impl IrContext) -> &'a IrStruct {
        (ir_context.struct_pool().get(&self.ident))
            // frb-coverage:ignore-start
            .unwrap_or_else(|| panic!("no entry found for key={:?}", self.ident))
        // frb-coverage:ignore-end
    }
}

impl IrTypeTrait for IrTypeStructRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        _ir_context: &impl IrContext,
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

impl IrStruct {
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

impl From<NamespacedName> for IrStructIdent {
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
