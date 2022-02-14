use crate::ir::*;

#[derive(Debug, Clone)]
pub struct ApiField {
    pub ty: ApiType,
    pub name: ApiIdent,
    pub comments: Vec<Comment>,
}

impl ApiField {
    pub fn name_rust_style(&self, is_fields_named: bool) -> String {
        if is_fields_named {
            self.name.rust_style().to_string()
        } else {
            // TO DO this is so hacky...
            self.name.rust_style().replace("field", "")
        }
    }
}
