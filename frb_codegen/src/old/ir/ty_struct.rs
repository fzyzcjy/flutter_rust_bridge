use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypeStructRef {
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

    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let wrapper = self.get(ir_pack).wrapper_name.as_ref();
        wrapper.unwrap_or(&self.rust_api_type()).clone()
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

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    pub fn const_capable(&self) -> bool {
        self.fields.iter().all(|field| field.is_final)
    }
}
