use crate::ir::*;
use crate::target::Target;
use convert_case::{Case, Casing};

impl IrTypeTrait for IrTypeEnumRef {}

impl IrEnum {
    pub fn is_struct(&self) -> bool {
        self.is_struct
    }
}
