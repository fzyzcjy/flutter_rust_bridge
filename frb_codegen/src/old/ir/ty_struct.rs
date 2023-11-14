use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeStructRef {
    pub fn get<'a>(&self, f: &'a IrFile) -> &'a IrStruct {
        &f.struct_pool[&self.name]
    }
}

impl IrTypeTrait for IrTypeStructRef {
    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }
    fn dart_wire_type(&self, target: Target) -> String {
        if target.is_wasm() {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "JsValue".into()
        } else {
            format!("wire_{}", self.name)
        }
    }

    fn intodart_type(&self, ir_file: &IrFile) -> String {
        let wrapper = self.get(ir_file).wrapper_name.as_ref();
        wrapper.unwrap_or(&self.rust_api_type()).clone()
    }
}

crate::ir! {
pub struct IrStruct {
    pub name: String,
    pub wrapper_name: Option<String>,
    pub path: Option<Vec<String>>,
    pub fields: Vec<IrField>,
    pub is_fields_named: bool,
    pub dart_metadata: Vec<IrDartAnnotation>,
    pub comments: Vec<IrComment>,
}
}

impl IrStruct {
    pub fn brackets_pair(&self) -> (char, char) {
        if self.is_fields_named {
            ('{', '}')
        } else {
            ('(', ')')
        }
    }

    pub fn using_freezed(&self) -> bool {
        self.dart_metadata.iter().any(|it| it.content == "freezed")
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn const_capable(&self) -> bool {
        self.fields.iter().all(|field| field.is_final)
    }
}
