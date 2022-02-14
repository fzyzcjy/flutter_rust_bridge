use crate::ir::*;
use convert_case::{Case, Casing};

#[derive(Debug, Clone)]
pub struct ApiTypeStructRef {
    pub name: String,
}

impl ApiTypeStructRef {
    pub fn get<'a>(&self, f: &'a ApiFile) -> &'a ApiStruct {
        &f.struct_pool[&self.name]
    }
}

impl ApiTypeChild for ApiTypeStructRef {
    fn visit_children_types<F: FnMut(&ApiType) -> bool>(&self, f: &mut F, api_file: &ApiFile) {
        for field in &self.get(api_file).fields {
            field.ty.visit_types(f, api_file);
        }
    }

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }
    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }

    fn dart_wire_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct ApiStruct {
    pub name: String,
    pub path: Option<Vec<String>>,
    pub fields: Vec<ApiField>,
    pub is_fields_named: bool,
    pub comments: Vec<Comment>,
}

impl ApiStruct {
    pub fn brackets_pair(&self) -> (char, char) {
        if self.is_fields_named {
            ('{', '}')
        } else {
            ('(', ')')
        }
    }
}
