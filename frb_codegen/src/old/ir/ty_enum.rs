use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypeEnumRef {
    fn dart_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            "List<dynamic>".into()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        match &self.get(ir_pack).wrapper_name {
            Some(wrapper) => wrapper.clone(),
            None => self.dart_api_type(),
        }
    }
}

impl IrEnum {
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }
}
