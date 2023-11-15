use crate::{ir::*, target::Target};

impl IrTypeTrait for IrTypeRecord {
    fn dart_api_type(&self) -> String {
        let values = self
            .values
            .iter()
            .map(IrType::dart_api_type)
            .collect::<Vec<_>>()
            .join(",");
        if self.values.len() == 1 {
            format!("({values},)")
        } else {
            format!("({values})")
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        if target.is_wasm() {
            "List<dynamic>".to_string()
        } else {
            self.rust_wire_type(target)
        }
    }

    fn rust_api_type(&self) -> String {
        let values = self
            .values
            .iter()
            .map(IrType::rust_api_type)
            .collect::<Vec<_>>()
            .join(",");
        format!("({values},)")
    }

    fn rust_wire_type(&self, target: Target) -> String {
        if target.is_wasm() {
            "JsValue".to_string()
        } else {
            format!("wire_{}", self.safe_ident())
        }
    }

    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let values = self
            .values
            .iter()
            .map(|e| e.intodart_type(ir_pack))
            .collect::<Vec<_>>()
            .join(",");
        format!("({values},)")
    }
}
