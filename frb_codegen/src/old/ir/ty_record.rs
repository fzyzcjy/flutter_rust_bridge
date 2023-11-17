use crate::{ir::*, target::Target};

impl IrTypeTrait for IrTypeRecord {
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
            .collect_vec()
            .join(",");
        format!("({values},)")
    }

    fn intodart_type(&self, ir_pack: &IrPack) -> String {
        let values = self
            .values
            .iter()
            .map(|e| e.intodart_type(ir_pack))
            .collect_vec()
            .join(",");
        format!("({values},)")
    }
}
