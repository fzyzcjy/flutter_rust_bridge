use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypeStructRef {
    fn dart_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
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
}
