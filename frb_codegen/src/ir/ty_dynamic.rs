use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrTypeDynamic;

impl IrTypeTrait for IrTypeDynamic {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, _f: &mut F, _ir_file: &IrFile) {}

    fn safe_ident(&self) -> String {
        "dartabi".to_owned()
    }

    fn dart_api_type(&self) -> String {
        "dynamic".to_owned()
    }

    fn dart_wire_type(&self, _target: crate::target::Target) -> String {
        panic!("Functions cannot receive dynamic parameters.")
    }

    fn rust_api_type(&self) -> String {
        "flutter_rust_bridge::DartAbi".to_owned()
    }

    fn rust_wire_type(&self, _target: crate::target::Target) -> String {
        panic!("Functions cannot receive dynamic parameters.")
    }
}
