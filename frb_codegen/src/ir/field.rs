use crate::ir::*;

#[derive(Debug, Clone)]
pub struct IrField {
    pub ty: IrType,
    pub name: IrIdent,
    pub comments: Vec<IrComment>,
}

impl IrField {
    pub fn name_rust_style(&self, is_fields_named: bool) -> String {
        if is_fields_named {
            self.name.rust_style().to_string()
        } else {
            // TO DO this is so hacky...
            self.name.rust_style().replace("field", "")
        }
    }
}
